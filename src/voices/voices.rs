use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug, time::SystemTime};

use rdev::Key;

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

    fn new_with_time(item: T, creation_time: SystemTime) -> Self {
        Self {
            item,
            creation_time,
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

pub struct Voices<T> {
    voices: BTreeMap<WithCreationTime<Key>, T>,
    capacity: usize,
}

impl <T> Voices<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            voices: BTreeMap::new(),
            capacity,
        }
    }

    fn is_playing(&self, key: Key) -> bool {
        self.voices.contains_key(&WithCreationTime::new(key))
    }

    pub fn play(&mut self, key: Key, sink: T) -> Option<&T> {
        if !self.is_playing(key) {
            if self.voices.len() == self.capacity {
                self.voices.pop_first();
            }
            let w = WithCreationTime::new(key);
            self.voices.insert(w, sink);
            self.voices.get(&w)
        } else {
            None
        }
    }

    pub fn play_with_time(&mut self, key: Key, time: SystemTime, sink: T) -> Option<&T> {
        if !self.is_playing(key) {
            if self.voices.len() == self.capacity {
                self.voices.pop_first();
            }
            let w = WithCreationTime::new_with_time(key, time);
            self.voices.insert(w, sink);
            self.voices.get(&w)
        } else {
            None
        }
    }

    pub fn stop(&mut self, key: Key) -> Option<T> {
        self.voices.remove(&WithCreationTime::new(key))
    }
}

impl <T> Debug for Voices<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys: Vec<_> = self.voices.iter().map(|v| v.0.item).collect();

        write!(f, "capacity: {:?}, keys: {:?}", self.capacity, keys)
    }
}