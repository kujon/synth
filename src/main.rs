use ::std::f32::consts::PI;

use ::rodio::{source::Source, Decoder, OutputStream};
use ::std::fs::File;
use ::std::io::BufReader;

enum Frequency {
    Angular(f32),
    Hertz(f32),
    BPM(f32),
}

impl Frequency {
    fn to_angular_value(&self) -> f32 {
        match self {
            Frequency::Angular(f) => *f,
            Frequency::Hertz(f) => *f * 2.0 * PI,
            Frequency::BPM(f) => (*f / 60.0) * 2.0 * PI,
        }
    }
}

struct SineWave {
    freq: Frequency,
    samples_elapsed: usize,
}

impl SineWave {
    fn new(freq: Frequency) -> SineWave {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (stream, stream_handle) = OutputStream::try_default()?;

    // let file =
    //     File::open("/Users/jakubkorzeniowski/Downloads/Soviet_Anthem_Instrumental_1955.ogg")?;
    // let file_handle = BufReader::new(file);

    // let source = Decoder::new(file_handle)?;

    let source = SineWave::new(Frequency::Hertz(440.0));

    let _ = stream_handle.play_raw(source)?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
