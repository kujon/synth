use rdev::Key;

use crate::utils::frequency::Note;

pub trait ToNote {
    fn to_note(&self) -> Option<Note>;
}

impl ToNote for Key {
    fn to_note(&self) -> Option<Note> {
        match self {
            Key::Num1 => Some(Note::C),
            Key::Num2 => Some(Note::D),
            Key::Num3 => Some(Note::E),
            Key::Num4 => Some(Note::F),
            Key::Num5 => Some(Note::G),
            Key::Num6 => Some(Note::A),
            Key::Num7 => Some(Note::B),
            _ => None,
        }
    }
}
