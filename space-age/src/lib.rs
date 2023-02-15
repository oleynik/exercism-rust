// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {seconds: s}
    }
}

pub trait Planet {
    fn earth_orbital_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        const YEAR: u64 = 31_557_600;
        d.seconds as f64 / YEAR as f64 / Self::earth_orbital_period()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn earth_orbital_period() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn earth_orbital_period() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn earth_orbital_period() -> f64 {
        1f64
    }
}
impl Planet for Mars {
    fn earth_orbital_period() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn earth_orbital_period() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn earth_orbital_period() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn earth_orbital_period() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn earth_orbital_period() -> f64 {
        164.79132
    }
}
