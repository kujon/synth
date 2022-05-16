use ::std::f32::consts::PI;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Frequency {
    Angular(f32),
    Hertz(f32),
    BPM(f32),
}

impl Frequency {
    pub fn to_angular_value(&self) -> f32 {
        match self {
            Frequency::Angular(f) => *f,
            Frequency::Hertz(f) => *f * 2.0 * PI,
            Frequency::BPM(f) => (*f / 60.0) * 2.0 * PI,
        }
    }

    pub fn octave_up(&self) -> Frequency {
        match self {
            Frequency::Angular(f) => Frequency::Angular(*f * 2.0),
            Frequency::Hertz(f) => Frequency::Hertz(*f * 2.0),
            Frequency::BPM(f) => Frequency::BPM(*f * 2.0),
        }
    }

    pub fn octave_down(&self) -> Frequency {
        match self {
            Frequency::Angular(f) => Frequency::Angular(*f / 2.0),
            Frequency::Hertz(f) => Frequency::Hertz(*f / 2.0),
            Frequency::BPM(f) => Frequency::BPM(*f / 2.0),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Note {
    pub fn to_frequency(&self) -> Frequency {
        match self {
            Note::C => Frequency::Hertz(261.63),
            Note::CSharp => Frequency::Hertz(277.18),
            Note::D => Frequency::Hertz(293.66),
            Note::DSharp => Frequency::Hertz(311.13),
            Note::E => Frequency::Hertz(329.63),
            Note::F => Frequency::Hertz(349.23),
            Note::FSharp => Frequency::Hertz(369.99),
            Note::G => Frequency::Hertz(392.00),
            Note::GSharp => Frequency::Hertz(415.30),
            Note::A => Frequency::Hertz(440.00),
            Note::ASharp => Frequency::Hertz(466.16),
            Note::B => Frequency::Hertz(493.88),
        }
    }

    pub fn semitone_down(&self) -> Note {
        match self {
            Note::C => Note::B,
            Note::CSharp => Note::C,
            Note::D => Note::CSharp,
            Note::DSharp => Note::D,
            Note::E => Note::DSharp,
            Note::F => Note::E,
            Note::FSharp => Note::F,
            Note::G => Note::FSharp,
            Note::GSharp => Note::G,
            Note::A => Note::GSharp,
            Note::ASharp => Note::A,
            Note::B => Note::ASharp,
        }
    }

    pub fn semitone_up(&self) -> Note {
        match self {
            Note::C => Note::CSharp,
            Note::CSharp => Note::D,
            Note::D => Note::DSharp,
            Note::DSharp => Note::E,
            Note::E => Note::F,
            Note::F => Note::FSharp,
            Note::FSharp => Note::G,
            Note::G => Note::GSharp,
            Note::GSharp => Note::A,
            Note::A => Note::ASharp,
            Note::ASharp => Note::B,
            Note::B => Note::C,
        }
    }
}
