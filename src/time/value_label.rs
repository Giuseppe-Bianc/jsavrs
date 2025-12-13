// Etichetta per il valore temporale con formattazione
use crate::time::time_values::{MICROSECONDS_FACTOR, MILLISECONDS_FACTOR, SECONDS_FACTOR};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct ValueLabel {
    time_val: f64,
    time_label: TimeUnit, // Cambiato da &'static str a enum TimeUnit
}

#[repr(u8)] // More compact representation
#[derive(Debug, Clone, Copy)]
enum TimeUnit {
    Seconds = 0,
    Milliseconds = 1,
    Microseconds = 2,
    Nanoseconds = 3,
    Other(&'static str),
}

impl TimeUnit {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Seconds => "s",
            Self::Milliseconds => "ms",
            Self::Microseconds => "us",
            Self::Nanoseconds => "ns",
            Self::Other(s) => s,
        }
    }
}

impl ValueLabel {
    #[inline]
    #[must_use]
    pub const fn time_val(&self) -> f64 {
        self.time_val
    }

    #[inline]
    #[must_use]
    pub const fn time_label(&self) -> &'static str {
        self.time_label.as_str()
    }

    #[must_use]
    pub fn new(time_val: f64, time_label: &'static str) -> Self {
        let unit = match time_label {
            "s" => TimeUnit::Seconds,
            "ms" => TimeUnit::Milliseconds,
            "us" => TimeUnit::Microseconds,
            "ns" => TimeUnit::Nanoseconds,
            _ => TimeUnit::Other(time_label),
        };

        Self { time_val, time_label: unit }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn format_time(&self) -> String {
        match self.time_label {
            TimeUnit::Seconds => {
                let total_nanos = (self.time_val * SECONDS_FACTOR).round().max(0.0) as u128;
                let secs = total_nanos / 1_000_000_000;
                let rem = total_nanos % 1_000_000_000;
                let millis = rem / 1_000_000;
                let rem2 = rem % 1_000_000;
                let micros = rem2 / 1_000;
                let nanos = rem2 % 1_000;
                format!("{secs}s,{millis}ms,{micros}μs,{nanos}ns")
            }
            TimeUnit::Milliseconds => {
                let total_nanos = (self.time_val * MILLISECONDS_FACTOR).round().max(0.0) as u128;
                let millis = total_nanos / 1_000_000;
                let rem = total_nanos % 1_000_000;
                let micros = rem / 1_000;
                let nanos = rem % 1_000;
                format!("{millis}ms,{micros}μs,{nanos}ns")
            }
            TimeUnit::Microseconds => {
                let total_nanos = (self.time_val * MICROSECONDS_FACTOR).round().max(0.0) as u128;
                let micros = total_nanos / 1_000;
                let nanos = total_nanos % 1_000;
                format!("{micros}μs,{nanos}ns")
            }
            TimeUnit::Nanoseconds => {
                let nanos = self.time_val.round().max(0.0) as u128;
                format!("{nanos}ns")
            }
            TimeUnit::Other(_) => {
                format!("{:.3} {}", self.time_val, self.time_label.as_str())
            }
        }
    }
}

impl fmt::Display for ValueLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format_time())
    }
}
