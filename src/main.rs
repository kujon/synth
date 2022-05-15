use::std::fs::File;
use::std::io::BufReader;
use::rodio::{Decoder, OutputStream, source::Source};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (stream, stream_handle) = OutputStream::try_default()?;

    let file = File::open("/Users/jakubkorzeniowski/Downloads/Soviet_Anthem_Instrumental_1955.ogg")?;
    let file_handle = BufReader::new(file);

    let source = Decoder::new(file_handle)?;

    let _ = stream_handle.play_raw(source.convert_samples())?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
