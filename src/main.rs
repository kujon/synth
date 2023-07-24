mod input;
mod oscilators;
mod utils;

use std::collections::VecDeque;

use ::rodio::{OutputStream, Sink};
// TO DO: replace this crate with proper windowing so that we don't have to grant accessibility permissions to listen to keystrokes globally.
use rdev::{listen, Event, EventType, Key};

use crate::input::keys::ToNote;
use crate::oscilators::sine::SineWave;
use crate::oscilators::square::SquareWave;
use crate::utils::frequency::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    let num_voices = 4;
    let mut voices: VecDeque<(Key, Sink)> = VecDeque::with_capacity(num_voices);

    // This will block.
    if let Err(error) = listen(move |event| match event.event_type {
        EventType::KeyPress(key) => {
            if !voices.iter().any(|v| v.0 == key) {
                if let Some(note) = key.to_note() {
                    match Sink::try_new(&stream_handle) {
                        Ok(sink) => {
                            println!("Note pressed {:?}", note);
                            sink.append(SquareWave::new(note.to_frequency()));
                            sink.play();
                            if voices.len() == voices.capacity() {
                                voices.pop_back();
                            }
                            voices.push_front((key, sink));

                            let just_notes: Vec<_> = voices.iter().map(|v| v.0).collect();
                            println!("{:?}", just_notes);
                        }
                        Err(e) => {
                            println!("{:?}", e);
                        }
                    }
                }
            }
        }
        EventType::KeyRelease(key) => {
            if let Some(index) = voices.iter().position(|(k, _)| k == &key) {
                println!("Note released {:?}", key.to_note());
                voices.remove(index);
                let just_notes: Vec<_> = voices.iter().map(|v| v.0).collect();
                println!("{:?}", just_notes);
            }
        }
        _ => (),
    }) {
        println!("Error: {:?}", error)
    }

    Ok(())
}
