// Costanti per la conversione temporale
pub const MICROSECONDS_FACTOR: f64 = 1_000.0;
pub const MILLISECONDS_FACTOR: f64 = 1_000_000.0;
pub const SECONDS_FACTOR: f64 = 1_000_000_000.0;
pub const MFACTOR: usize = 100;
pub const TILE_PADDING: usize = 10;

// Rappresenta i valori temporali in diverse unità
#[derive(Debug, Clone, Copy)]
pub struct TimeValues {
    seconds: f64,
    millis: f64,
    micro: f64,
    nano: f64,
}

impl TimeValues {
    pub fn from_nanoseconds(nanoseconds: f64) -> Self {
        TimeValues {
            seconds: nanoseconds / SECONDS_FACTOR,
            millis: nanoseconds / MILLISECONDS_FACTOR,
            micro: nanoseconds / MICROSECONDS_FACTOR,
            nano: nanoseconds,
        }
    }

    #[inline(always)]
    pub fn seconds(&self) -> f64 {
        self.seconds
    }
    #[inline(always)]
    pub fn millis(&self) -> f64 {
        self.millis
    }
    #[inline(always)]
    pub fn micro(&self) -> f64 {
        self.micro
    }
    #[inline(always)]
    pub fn nano(&self) -> f64 {
        self.nano
    }
}
