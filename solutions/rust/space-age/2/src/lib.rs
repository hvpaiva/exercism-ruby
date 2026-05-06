#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64 / (Self::YEAR_IN_SECONDS as f64))
    }
}

impl Duration {
    const YEAR_IN_SECONDS: u64 = 31_557_600;

    fn to_years(&self) -> f64 {
        self.0
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.to_years() / Self::orbital_period()
    }

    fn orbital_period() -> f64;
}

macro_rules! planet {
    ($name:ident, $orbital_period:expr) => {
        pub struct $name;
        impl Planet for $name {
            fn orbital_period() -> f64 {
                $orbital_period
            }
        }
    };
}

planet!(Mercury, 0.240_846_7);
planet!(Venus, 0.615_197_26);
planet!(Earth, 1.0);
planet!(Mars, 1.880_815_8);
planet!(Jupiter, 11.862_615);
planet!(Saturn, 29.447_498);
planet!(Uranus, 84.016_846);
planet!(Neptune, 164.791_32);
