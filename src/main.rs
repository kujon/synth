mod input;
mod oscilators;
mod utils;

use std::cmp::Ordering;
use std::collections::{BTreeMap, VecDeque};
use std::time::SystemTime;

use ::rodio::{OutputStream, Sink};
// TO DO: replace this crate with proper windowing so that we don't have to grant accessibility permissions to listen to keystrokes globally.
use rdev::{listen, Event, EventType, Key};

use crate::input::keys::ToNote;
use crate::oscilators::sine::SineWave;
use crate::oscilators::square::SquareWave;
use crate::utils::frequency::Note;

#[derive(Debug, Clone, Copy, Eq)]
struct WithCreationTime<T> {
    item: T,
    creation_time: SystemTime,
}

impl<T> WithCreationTime<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            creation_time: SystemTime::now(),
        }
    }
}

impl<T> PartialEq for WithCreationTime<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.item.eq(&other.item)
    }
}

impl<T> PartialOrd for WithCreationTime<T>
where
    T: PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.item == other.item {
            Some(Ordering::Equal)
        } else {
            self.creation_time.partial_cmp(&other.creation_time)
        }
    }
}

impl<T> Ord for WithCreationTime<T>
where
    T: PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.item == other.item {
            Ordering::Equal
        } else {
            self.creation_time.cmp(&other.creation_time)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let num_voices = 4;
    let mut voices: BTreeMap<WithCreationTime<Key>, Sink> = BTreeMap::new();

    // This will block.
    if let Err(error) = listen(move |event| match event.event_type {
        EventType::KeyPress(key) => {
            if !voices.contains_key(&WithCreationTime::new(key)) {
                if let Some(note) = key.to_note() {
                    match Sink::try_new(&stream_handle) {
                        Ok(sink) => {
                            println!("Note pressed {:?}", note);
                            sink.append(SineWave::new(note.to_frequency()));
                            sink.play();
                            if voices.len() == num_voices {
                                voices.pop_first();
                                println!("pop");
                                let just_notes: Vec<_> = voices.iter().map(|v| v.0.item).collect();
                                println!("{:?}", just_notes);
                            }
                            voices.insert(WithCreationTime::new(key), sink);

                            println!("start");
                            let just_notes: Vec<_> = voices.iter().map(|v| v.0.item).collect();
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
            let removed = voices.remove(&WithCreationTime::new(key));
            if removed.is_some() {
                println!("Note released {:?}", key.to_note());
                let just_notes: Vec<_> = voices.iter().map(|v| v.0.item).collect();
                println!("{:?}", just_notes);
            }
        }
        _ => (),
    }) {
        println!("Error: {:?}", error)
    }

    Ok(())
}
