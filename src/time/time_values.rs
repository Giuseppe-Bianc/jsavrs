// Costanti per la conversione temporale
pub const MICROSECONDS_FACTOR: f64 = 1_000.0;
pub const MILLISECONDS_FACTOR: f64 = 1_000_000.0;
pub const SECONDS_FACTOR: f64 = 1_000_000_000.0;
pub const MFACTOR: usize = 100;
pub const TILE_PADDING: usize = 10;

// Rappresenta i valori temporali in diverse unità
#[derive(Debug, Clone, Copy)]
pub struct TimeValues {
    nano: f64, // Only store base representation
}

impl TimeValues {
    #[inline]
    #[must_use]
    pub const fn from_nanoseconds(nanoseconds: f64) -> Self {
        Self { nano: nanoseconds }
    }

    #[inline]
    #[must_use]
    pub const fn seconds(&self) -> f64 {
        self.nano / SECONDS_FACTOR
    }

    #[inline]
    #[must_use]
    pub const fn millis(&self) -> f64 {
        self.nano / MILLISECONDS_FACTOR
    }

    #[inline]
    #[must_use]
    pub const fn micro(&self) -> f64 {
        self.nano / MICROSECONDS_FACTOR
    }

    #[inline]
    #[must_use]
    pub const fn nano(&self) -> f64 {
        self.nano
    }
}
