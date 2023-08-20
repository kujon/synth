use rdev::Key;

fn get_ordinal(key: Key) -> u8 {
    let ptr_to_option = (&key as *const Key) as *const u8;
    unsafe {
        *ptr_to_option
    }
}

pub struct Voices2<T> {
    in_order: Vec<(Key, T)>,
    mask: u128
}

impl <T> Voices2<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            in_order: Vec::with_capacity(capacity),
            mask: 0,
        }
    }

    pub fn is_playing(&self, key: Key) -> bool {
        let ord = get_ordinal(key);
        self.mask & (1 << ord) != 0
    }

    pub fn play(&mut self, key: Key, sink: T) {
        let ord = get_ordinal(key);
        let bit = 1 << ord;
        let playing = self.mask & bit != 0;
        
        if !playing {
            if self.in_order.len() == self.in_order.capacity() {
                self.in_order.remove(0);
            }
            self.in_order.push((key, sink));
            self.mask |= bit;
        }
    }

    pub fn stop(&mut self, key: Key) {
        let ord = get_ordinal(key);
        let bit = 1 << ord;
        let playing = self.mask & bit != 0;

        if playing {
            self.in_order.retain(|(k,_)| !matches!(*k, key));
            self.mask &= !bit;
        }
    }
}