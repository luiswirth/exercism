// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

impl std::ops::Div for &Duration {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.seconds as f64 / rhs.seconds as f64
    }
}

pub trait Planet {
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

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(7_600_544);
        d / &orbit_duration
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(19_414_149);
        d / &orbit_duration
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(31_557_600);
        d / &orbit_duration
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(59_354_032);
        d / &orbit_duration
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(374_355_659);
        d / &orbit_duration
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(929_292_363);
        d / &orbit_duration
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(2_651_370_019);
        d / &orbit_duration
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let orbit_duration = Duration::from(5_200_418_560);
        d / &orbit_duration
    }
}
