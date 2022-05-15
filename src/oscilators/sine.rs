use ::rodio::source::Source;

use crate::utils::frequency::Frequency;

pub struct SineWave {
    freq: Frequency,
    samples_elapsed: usize,
}

impl SineWave {
    pub fn new(freq: Frequency) -> SineWave {
        SineWave {
            freq: freq,
            samples_elapsed: 0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let x = self.freq.to_angular_value() * self.samples_elapsed as f32 / 44100.0;
        self.samples_elapsed = self.samples_elapsed.wrapping_add(1);
        Some(x.sin())
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn total_duration(&self) -> Option<::std::time::Duration> {
        None
    }

    fn sample_rate(&self) -> u32 {
        44100
    }
}
