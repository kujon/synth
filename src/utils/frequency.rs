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
}
