// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod waves;
use waves::{SawWave, SquareWave, TriangleWave};

use rodio::Source;

// note freqs
const C4: f32 = 261.63;
const C4S: f32 = 277.18;
const D4: f32 = 293.66;
const D4S: f32 = 311.13;
const E4: f32 = 329.63;
const F4: f32 = 349.23;
const F4S: f32 = 369.99;
const G4: f32 = 392.00;
const G4S: f32 = 415.30;
const A4: f32 = 440.00;
const A4S: f32 = 466.16;
const B4: f32 = 493.88;
const C5: f32 = 523.25;

struct MyApp {
    notes: std::collections::HashMap<egui::Key, Note>,
    wave: NoteWave,
    apply_reverb: bool,
    bpm: f32,
}

enum NoteWave {
    Sine,
    Square,
    Saw,
    Triangle,
}

struct Note {
    sink: rodio::Sink,
    freq: f32,
    wave: NoteWave,
    reverb: bool,
}

impl Note {
    fn new(sink: rodio::Sink, freq: f32) -> Self {
        Self {
            sink,
            freq,
            wave: NoteWave::Sine,
            reverb: false,
        }
    }

    fn set_wave(&mut self, wave: NoteWave) {
        self.wave = wave;
    }

    fn set_reverb(&mut self, reverb: bool) {
        self.reverb = reverb;
    }

    fn stop(&self) {
        self.sink.stop();
    }

    fn gen_source(&self) -> Box<dyn rodio::Source<Item = f32> + Send> {
        match self.wave {
            NoteWave::Sine => {
                let source = rodio::source::SineWave::new(self.freq).amplify(0.5);
                if self.reverb {
                    let reverb = SabinReverb::new(100, 0.5, Box::new(source));
                    return Box::new(reverb);
                }
                return Box::new(source);
            }
            NoteWave::Square => {
                let source = SquareWave::new(self.freq).amplify(0.5);
                if self.reverb {
                    let reverb = SabinReverb::new(250, 0.7, Box::new(source));
                    return Box::new(reverb);
                } else {
                    return Box::new(source);
                }
            }
            NoteWave::Saw => {
                let source = SawWave::new(self.freq).amplify(0.5);
                if self.reverb {
                    let reverb = SabinReverb::new(100, 0.5, Box::new(source));
                    return Box::new(reverb);
                } else {
                    return Box::new(source);
                }
            }
            NoteWave::Triangle => {
                let source = TriangleWave::new(self.freq).amplify(0.5);
                if self.reverb {
                    let reverb = SabinReverb::new(100, 0.5, Box::new(source));
                    return Box::new(reverb);
                } else {
                    return Box::new(source);
                }
            }
        }
    }

    fn play(&self) {
        self.sink.append(self.gen_source());
        self.sink.play();
    }
}

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    // get the sink
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // setup ui
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(move |_cc| {
            Box::new(MyApp {
                wave: NoteWave::Sine,
                apply_reverb: false,
                bpm: 120.0,
                notes: vec![
                    (
                        egui::Key::A,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), C4),
                    ),
                    (
                        egui::Key::W,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), C4S),
                    ),
                    (
                        egui::Key::S,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), D4),
                    ),
                    (
                        egui::Key::E,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), D4S),
                    ),
                    (
                        egui::Key::D,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), E4),
                    ),
                    (
                        egui::Key::F,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), F4),
                    ),
                    (
                        egui::Key::T,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), F4S),
                    ),
                    (
                        egui::Key::G,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), G4),
                    ),
                    (
                        egui::Key::Y,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), G4S),
                    ),
                    (
                        egui::Key::H,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), A4),
                    ),
                    (
                        egui::Key::U,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), A4S),
                    ),
                    (
                        egui::Key::J,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), B4),
                    ),
                    (
                        egui::Key::K,
                        Note::new(rodio::Sink::try_new(&stream_handle).unwrap(), C5),
                    ),
                ]
                .into_iter()
                .collect(),
            })
        }),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Wavebender");
            ui.separator();

            ui.label("BPM");
            ui.add(egui::Slider::new(&mut self.bpm, 60.0..=240.0).text("BPM"));

            ui.label("Synthesizer");
            {
                const SAMPLES: usize = 10000;
                let plot: egui::plot::PlotPoints = self
                    .notes
                    .get(&egui::Key::A)
                    .unwrap()
                    .clone()
                    .gen_source()
                    .take(SAMPLES)
                    .enumerate()
                    .map(|(x, y)| [x as f64 / SAMPLES as f64, y as f64])
                    .collect();
                let line = egui::plot::Line::new(plot);
                egui::plot::Plot::new("waveform")
                    .view_aspect(2.0)
                    .show(ui, |ui| ui.line(line));

                // you can change the wave type
                ui.horizontal(|ui| {
                    if ui.button("Sine").on_hover_text("Sine wave").clicked() {
                        for (_, note) in self.notes.iter_mut() {
                            note.set_wave(NoteWave::Sine);
                        }
                    }
                    if ui.button("Square").on_hover_text("Square wave").clicked() {
                        for (_, note) in self.notes.iter_mut() {
                            note.set_wave(NoteWave::Square);
                        }
                    }
                    if ui.button("Saw").on_hover_text("Saw wave").clicked() {
                        for (_, note) in self.notes.iter_mut() {
                            note.set_wave(NoteWave::Saw);
                        }
                    }
                    if ui
                        .button("Triangle")
                        .on_hover_text("Triangle wave")
                        .clicked()
                    {
                        for (_, note) in self.notes.iter_mut() {
                            note.set_wave(NoteWave::Triangle);
                        }
                    }
                });

                // reverb options
                ui.checkbox(&mut self.apply_reverb, "Reverb");
                for (_, note) in self.notes.iter_mut() {
                    note.set_reverb(self.apply_reverb);
                }
            }

            for (key, note) in self.notes.iter_mut() {
                if ctx.input(|i| i.key_pressed(*key) && i.num_presses(*key) == 1) {
                    note.play();
                }

                if ctx.input(|i| i.key_released(*key)) {
                    note.stop();
                }
            }
        });
    }
}

// reverb
struct SabinReverb {
    delay: usize,
    feedback: f32,
    source: Box<dyn Source<Item = f32> + Send>,
    buffer: Vec<f32>,
    index: usize,
}

impl SabinReverb {
    pub fn new(delay: usize, feedback: f32, source: Box<dyn Source<Item = f32> + Send>) -> Self {
        SabinReverb {
            delay,
            feedback,
            source,
            buffer: vec![0.0; delay],
            index: 0,
        }
    }
}

impl Source for SabinReverb {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}

impl Iterator for SabinReverb {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let sample = self.source.next()?;
        let delayed_sample = self.buffer[self.index];
        self.buffer[self.index] = sample + delayed_sample * self.feedback;
        self.index = (self.index + 1) % self.delay;
        Some(sample + delayed_sample)
    }
}

// Envelope is a simple ADSR envelope
pub enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
    Done,
}

pub struct Envelope {
    state: EnvelopeState,
    attack_duration: usize,
    decay_duration: usize,
    sustain_level: f32,
    release_duration: usize,
    sample_rate: usize,
    current_sample: usize,
}

impl Envelope {
    pub fn new(
        attack_duration: usize,
        decay_duration: usize,
        sustain_level: f32,
        release_duration: usize,
        sample_rate: usize,
    ) -> Self {
        Envelope {
            state: EnvelopeState::Attack,
            attack_duration,
            decay_duration,
            sustain_level,
            release_duration,
            sample_rate,
            current_sample: 0,
        }
    }
}

impl Iterator for Envelope {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = match self.state {
            EnvelopeState::Attack => {
                let progress = self.current_sample as f32 / self.attack_duration as f32;
                progress
            }
            EnvelopeState::Decay => {
                let progress = self.current_sample as f32 / self.decay_duration as f32;
                1.0 - progress * (1.0 - self.sustain_level)
            }
            EnvelopeState::Sustain => self.sustain_level,
            EnvelopeState::Release => {
                let progress = self.current_sample as f32 / self.release_duration as f32;
                self.sustain_level * (1.0 - progress)
            }
            EnvelopeState::Done => return None,
        };

        self.current_sample += 1;

        match self.state {
            EnvelopeState::Attack if self.current_sample >= self.attack_duration => {
                self.state = EnvelopeState::Decay;
                self.current_sample = 0;
            }
            EnvelopeState::Decay if self.current_sample >= self.decay_duration => {
                self.state = EnvelopeState::Sustain;
                self.current_sample = 0;
            }
            EnvelopeState::Release if self.current_sample >= self.release_duration => {
                self.state = EnvelopeState::Done;
                self.current_sample = 0;
            }
            _ => (),
        }

        Some(value)
    }
}
