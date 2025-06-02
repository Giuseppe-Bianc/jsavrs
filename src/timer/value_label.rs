// Etichetta per il valore temporale con formattazione
use std::fmt;
use crate::timer::time_values::{MICROSECONDS_FACTOR, MILLISECONDS_FACTOR, SECONDS_FACTOR};
#[derive(Debug, Clone, Copy)]
pub struct ValueLabel {
    pub time_val: f64,
    pub time_label: &'static str,
}

impl ValueLabel {
    pub fn new(time_val: f64, time_label: &'static str) -> Self {
        ValueLabel { time_val, time_label }
    }

    pub fn format_time(&self) -> String {
        let total_nanos = match self.time_label {
            "s"  => (self.time_val * SECONDS_FACTOR).round() as u128,
            "ms" => (self.time_val * MILLISECONDS_FACTOR).round() as u128,
            "us" => (self.time_val * MICROSECONDS_FACTOR).round() as u128,
            "ns" => self.time_val.round() as u128,
            _ => return format!("{:.3} {}", self.time_val, self.time_label),
        };

        match self.time_label {
            "s" => {
                let secs = total_nanos / 1_000_000_000;
                let rem = total_nanos % 1_000_000_000;
                let millis = rem / 1_000_000;
                let rem2 = rem % 1_000_000;
                let micros = rem2 / 1_000;
                let nanos = rem2 % 1_000;
                format!("{}s,{}ms,{}μs,{}ns", secs, millis, micros, nanos)
            }
            "ms" => {
                let millis = total_nanos / 1_000_000;
                let rem = total_nanos % 1_000_000;
                let micros = rem / 1_000;
                let nanos = rem % 1_000;
                format!("{}ms,{}μs,{}ns", millis, micros, nanos)
            }
            "us" => {
                let micros = total_nanos / 1_000;
                let nanos = total_nanos % 1_000;
                format!("{}μs,{}ns", micros, nanos)
            }
            "ns" => format!("{}ns", total_nanos),
            _ => format!("{:.3} {}", self.time_val, self.time_label),
        }
    }
}

impl fmt::Display for ValueLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format_time())
    }
}