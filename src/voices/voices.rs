use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{write, Debug},
    time::SystemTime,
};

use rdev::Key;
use rodio::Sink;

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

pub struct Voices {
    voices: BTreeMap<WithCreationTime<Key>, Sink>,
    capacity: usize,
}

impl Voices {
    pub fn new(capacity: usize) -> Self {
        Self {
            voices: BTreeMap::new(),
            capacity,
        }
    }

    fn is_playing(&self, key: &Key) -> bool {
        self.voices.contains_key(&WithCreationTime::new(*key))
    }

    pub fn play(&mut self, key: Key, sink: Sink) -> Option<&Sink> {
        if !self.is_playing(&key) {
            if self.voices.len() == self.capacity {
                self.voices.pop_first();
            }
            self.voices.insert(WithCreationTime::new(key), sink);
            self.voices.get(&WithCreationTime::new(key))
        } else {
            None
        }
    }

    pub fn stop(&mut self, key: &Key) -> Option<Sink> {
        self.voices.remove(&WithCreationTime::new(*key))
    }
}

impl Debug for Voices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys: Vec<_> = self.voices.iter().map(|v| v.0.item).collect();

        write!(f, "capacity: {:?}, keys: {:?}", self.capacity, keys)
    }
}
