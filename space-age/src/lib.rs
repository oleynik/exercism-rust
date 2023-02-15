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

macro_rules! planet {
    ($name: ident, $orbital_period: expr) => {
        pub struct $name;
        impl Planet for $name {
            fn earth_orbital_period() -> f64 {$orbital_period}
        }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1f64);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
