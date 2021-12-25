// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { s }
    }
}

pub trait Planet {
    #[must_use]
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_planet_years_during {
    ($($ty:ty: $r:literal),*) => {
        $(impl Planet for $ty {
            fn years_during(d: &Duration) -> f64 {
                d.s as f64 / (31_557_600.0 * $r)
            }
        })*
    };
}

impl_planet_years_during!(
    Mercury: 0.240_846_7,
    Venus: 0.615_197_26,
    Earth: 1.0,
    Mars: 1.880_815_8,
    Jupiter: 11.862_615,
    Saturn: 29.447_498,
    Uranus: 84.016_846,
    Neptune: 164.79132
);
