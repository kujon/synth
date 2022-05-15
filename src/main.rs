mod oscilators;
mod utils;

use ::rodio::OutputStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    let source = oscilators::square::SquareWave::new(utils::frequency::Frequency::Hertz(440.0));

    let _ = stream_handle.play_raw(source)?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
