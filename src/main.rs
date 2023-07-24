mod input;
mod oscilators;
mod utils;

use std::collections::VecDeque;

use ::rodio::{OutputStream, Sink};
// TO DO: replace this crate with proper windowing so that we don't have to grant accessibility permissions to listen to keystrokes globally.
use rdev::{listen, Event, EventType};

use crate::input::keys::ToNote;
use crate::oscilators::sine::SineWave;
use crate::oscilators::square::SquareWave;
use crate::utils::frequency::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    let mut voices: VecDeque<(Note, Sink)> = VecDeque::with_capacity(2);

    let mut current_note: Option<Note> = None;

    // This will block.
    if let Err(error) = listen(move |event| match event.event_type {
        EventType::KeyPress(key) => {
            if let Some(note) = key.to_note() {
                println!("Note pressed {:?}", note);

                if Some(note) != current_note {
                    match Sink::try_new(&stream_handle) {
                        Ok(sink) => {
                            current_note = Some(note);
                            sink.append(SquareWave::new(note.to_frequency()));
                            sink.play();
                            if voices.len() == voices.capacity() {
                                voices.pop_back();
                            }
                            voices.push_front((note, sink));
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                }
            }
        }
        EventType::KeyRelease(key) => {
            if let Some(note) = key.to_note() {
                println!("Note released {:?}", note);
                if let Some(index) = voices.iter().position(|(n, _)| n == &note) {
                    voices.remove(index);
                }
            }
        }
        _ => (),
    }) {
        println!("Error: {:?}", error)
    }

    Ok(())
}
