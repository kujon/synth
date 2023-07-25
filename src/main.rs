mod input;
mod oscilators;
mod utils;
mod voices;

use std::cmp::Ordering;
use std::collections::{BTreeMap, VecDeque};
use std::time::SystemTime;

use ::rodio::{OutputStream, Sink};
// TO DO: replace this crate with proper windowing so that we don't have to grant accessibility permissions to listen to keystrokes globally.
use rdev::{listen, Event, EventType, Key};
use voices::voices::Voices;

use crate::input::keys::ToNote;
use crate::oscilators::sine::SineWave;
use crate::oscilators::square::SquareWave;
use crate::utils::frequency::Note;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let num_voices = 4;
    let mut voices = Voices::new(num_voices);

    // This will block.
    if let Err(error) = listen(move |event| match event.event_type {
        EventType::KeyPress(key) => {
            if let Some(note) = key.to_note() {
                match Sink::try_new(&stream_handle) {
                    Ok(sink) => {
                        sink.append(SineWave::new(note.to_frequency()));
                        sink.play();
                        let played = voices.play(key, sink);
                        if played.is_some() {
                            println!("Key pressed: {:?}, voices: {:?}", key, voices);
                        }
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
        }
        EventType::KeyRelease(key) => {
            let removed = voices.stop(&key);
            if removed.is_some() {
                println!("Key released: {:?}, voices: {:?}", key, voices);
            }
        }
        _ => (),
    }) {
        println!("Error: {:?}", error)
    }

    Ok(())
}
