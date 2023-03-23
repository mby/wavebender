pub struct SquareWave {
    freq: f32,
    phase: f32,
}

impl SquareWave {
    pub fn new(freq: f32) -> Self {
        Self { freq, phase: 0.0 }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += self.freq / 44100.0;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        Some(if self.phase < 0.5 { 1.0 } else { -1.0 })
    }
}

impl rodio::Source for SquareWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

pub struct SawWave {
    freq: f32,
    phase: f32,
}

impl SawWave {
    pub fn new(freq: f32) -> Self {
        Self { freq, phase: 0.0 }
    }
}

impl Iterator for SawWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += self.freq / 44100.0;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        Some(self.phase * 2.0 - 1.0)
    }
}

impl rodio::Source for SawWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

pub struct TriangleWave {
    freq: f32,
    phase: f32,
}

impl TriangleWave {
    pub fn new(freq: f32) -> Self {
        Self { freq, phase: 0.0 }
    }
}

impl Iterator for TriangleWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += self.freq / 44100.0;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        Some(if self.phase < 0.5 {
            self.phase * 4.0 - 1.0
        } else {
            3.0 - self.phase * 4.0
        })
    }
}

impl rodio::Source for TriangleWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
