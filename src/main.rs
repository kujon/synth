mod oscilators;
mod utils;

use ::rodio::{source::Source, OutputStream, Sink};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle).unwrap();

    let square = oscilators::square::SquareWave::new(utils::frequency::Notes::C.to_frequency())
        .take_duration(Duration::from_secs_f32(1.0));
    sink.append(square);

    sink.sleep_until_end();

    Ok(())
}
