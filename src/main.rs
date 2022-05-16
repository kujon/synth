mod input;
mod oscilators;
mod utils;

use ::rodio::{OutputStream, Sink};
use rdev::{listen, EventType};

use crate::input::keys::ToNote;
use crate::oscilators::square::SquareWave;
use crate::utils::frequency::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let mut current_note: Option<Note> = None;

    // This will block.
    if let Err(error) = listen(move |event| match event.event_type {
        EventType::KeyPress(key) => {
            if let Some(note) = key.to_note() {
                if Some(note) != current_note {
                    println!("Note pressed {:?}", note);
                    current_note = Some(note);
                    sink.append(SquareWave::new(note.to_frequency()));
                    sink.play();
                }
            }
        }
        EventType::KeyRelease(key) => {
            if let Some(note) = key.to_note() {
                println!("Note released {:?}", note);
                current_note = None;
                sink.stop();
            }
        }
        _ => (),
    }) {
        println!("Error: {:?}", error)
    }

    Ok(())
}
