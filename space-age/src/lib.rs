static SECONDS_PER_EATH_YEAR: u64 = 31557600;

#[derive(Debug)]
pub struct Duration {
    time: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            time: s
        }
    }
}

pub trait Planet {
    const ORBITAL_YEARS: f64;
    fn years_during(d: &Duration) -> f64 {
        let earth_years = d.time as f64 / SECONDS_PER_EATH_YEAR as f64;
        earth_years / Self::ORBITAL_YEARS
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
    const ORBITAL_YEARS: f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBITAL_YEARS: f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBITAL_YEARS: f64 = 1.0;
}
impl Planet for Mars {
    const ORBITAL_YEARS: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL_YEARS: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBITAL_YEARS: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBITAL_YEARS: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBITAL_YEARS: f64 = 164.79132;
}
