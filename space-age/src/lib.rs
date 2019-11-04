// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    pub secs: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { secs: s }
    }
}

pub trait Planet {
    const SECONDS_IN_YEAR: f64;
    fn years_during(d: &Duration) -> f64 {
        d.secs as f64 / Self::SECONDS_IN_YEAR
    }
}

const SECONDS_IN_EARTH_YEAR: f64 = 31557600.0;

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const SECONDS_IN_YEAR: f64 = 0.2408467 * SECONDS_IN_EARTH_YEAR;
}
impl Planet for Venus {
    const SECONDS_IN_YEAR: f64 = 0.61519726 * SECONDS_IN_EARTH_YEAR;
}
impl Planet for Earth {
    const SECONDS_IN_YEAR: f64 = 1.0 * SECONDS_IN_EARTH_YEAR;
}
impl Planet for Mars {
    const SECONDS_IN_YEAR: f64 = 1.8808158 * SECONDS_IN_EARTH_YEAR;
}
impl Planet for Jupiter {
    const SECONDS_IN_YEAR: f64 = 11.862615 * SECONDS_IN_EARTH_YEAR;
}
impl Planet for Saturn {
    const SECONDS_IN_YEAR: f64 = 29.447498 * SECONDS_IN_EARTH_YEAR;
}
impl Planet for Uranus {
    const SECONDS_IN_YEAR: f64 = 84.016846 * SECONDS_IN_EARTH_YEAR;
}
impl Planet for Neptune {
    const SECONDS_IN_YEAR: f64 = 164.79132 * SECONDS_IN_EARTH_YEAR;
}
