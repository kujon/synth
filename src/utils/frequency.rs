use ::std::f32::consts::PI;

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

pub enum Notes {
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

impl Notes {
    pub fn to_frequency(&self) -> Frequency {
        match self {
            Notes::C => Frequency::Hertz(261.63),
            Notes::CSharp => Frequency::Hertz(277.18),
            Notes::D => Frequency::Hertz(293.66),
            Notes::DSharp => Frequency::Hertz(311.13),
            Notes::E => Frequency::Hertz(329.63),
            Notes::F => Frequency::Hertz(349.23),
            Notes::FSharp => Frequency::Hertz(369.99),
            Notes::G => Frequency::Hertz(392.00),
            Notes::GSharp => Frequency::Hertz(415.30),
            Notes::A => Frequency::Hertz(440.00),
            Notes::ASharp => Frequency::Hertz(466.16),
            Notes::B => Frequency::Hertz(493.88),
        }
    }

    pub fn semitone_down(&self) -> Notes {
        match self {
            Notes::C => Notes::B,
            Notes::CSharp => Notes::C,
            Notes::D => Notes::CSharp,
            Notes::DSharp => Notes::D,
            Notes::E => Notes::DSharp,
            Notes::F => Notes::E,
            Notes::FSharp => Notes::F,
            Notes::G => Notes::FSharp,
            Notes::GSharp => Notes::G,
            Notes::A => Notes::GSharp,
            Notes::ASharp => Notes::A,
            Notes::B => Notes::ASharp,
        }
    }
}
