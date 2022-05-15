use ::rodio::source::Source;

use crate::utils::frequency::Frequency;

pub struct SquareWave {
    freq: Frequency,
    samples_elapsed: usize,
}

impl SquareWave {
    pub fn new(frequency: Frequency) -> SquareWave {
        SquareWave {
            freq: frequency,
            samples_elapsed: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let x = self.freq.to_angular_value() * self.samples_elapsed as f32 / 44100.0;
        self.samples_elapsed = self.samples_elapsed.wrapping_add(1);
        Some(if x.sin() > 0.0 { 0.3 } else { -0.3 })
    }
}

impl Source for SquareWave {
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
