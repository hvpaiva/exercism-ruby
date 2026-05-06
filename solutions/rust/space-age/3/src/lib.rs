#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

impl Duration {
    const EARTH_YEAR_SECONDS: f64 = 31557600.0;

    fn to_years(&self) -> f64 {
        self.0 / Self::EARTH_YEAR_SECONDS
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.to_years() / Self::EARTH_YEAR_RATIO
    }

    const EARTH_YEAR_RATIO: f64;
}

macro_rules! planets {
    ($($t:ident => $e:expr),+) => {
        $(
            pub struct $t;
            impl Planet for $t {
                const EARTH_YEAR_RATIO: f64 = $e;
            }
        )*
    }
}

planets!(
    Mercury => 0.240_846_7,
    Venus => 0.615_197_26,
    Earth => 1.0,
    Mars => 1.880_815_8,
    Jupiter => 11.862_615,
    Saturn => 29.447_498,
    Uranus => 84.016_846,
    Neptune => 164.791_32
);
