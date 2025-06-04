use crate::time::time_values::{MFACTOR, SECONDS_FACTOR, TILE_PADDING};
use crate::time::times::{TimePrintFn, Times, simple_format};
use crate::time::value_label::ValueLabel;
use std::fmt;
use std::ops::{Div, DivAssign};
use std::time::{Duration, Instant};

pub struct Timer {
    title: String,
    title_padding: usize,
    time_print: TimePrintFn,
    start: Instant,
    cycles: usize,
}

impl Timer {
    pub fn new(title: &str) -> Self {
        Timer::with_formatter(title, simple_format)
    }

    pub fn with_formatter(title: &str, time_print: TimePrintFn) -> Self {
        Timer {
            title: title.to_string(),
            title_padding: title.len() + TILE_PADDING,
            time_print,
            start: Instant::now(),
            cycles: 1,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn make_time(&self) -> f64 {
        self.elapsed().as_nanos() as f64 / self.cycles as f64
    }

    pub fn make_time_str(&self) -> ValueLabel {
        Times::from_nanoseconds(self.make_time()).get_relevant_timeframe()
    }

    pub fn time_it<F>(&mut self, f: F, target_time: f64) -> String
    where
        F: Fn(),
    {
        let original_start = self.start;
        self.start = Instant::now();
        let mut n = 0;
        let mut total_time;

        loop {
            f();
            n += 1;
            total_time = self.start.elapsed().as_nanos() as f64;

            if n >= MFACTOR || total_time >= target_time * SECONDS_FACTOR {
                break;
            }
        }

        let avg_time = total_time / n as f64;
        let time_str = Times::from_nanoseconds(avg_time).get_relevant_timeframe();
        self.start = original_start;

        format!("{time_str} for {n} tries")
    }

    // Renamed to avoid shadowing Display::to_string
    pub fn as_string(&self) -> String {
        let time_str = self.make_time_str();
        (self.time_print)(&self.title, self.title_padding, &time_str)
    }
}

impl Div<usize> for Timer {
    type Output = Self;

    fn div(mut self, rhs: usize) -> Self {
        if rhs == 0 {
            panic!("Cannot divide timer by zero");
        }
        self.cycles = rhs;
        self
    }
}

impl DivAssign<usize> for Timer {
    fn div_assign(&mut self, rhs: usize)  {
        if rhs == 0 {
            panic!("Cannot divide timer by zero");
        }
        self.cycles = rhs;
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Call the renamed method to avoid recursion
        write!(f, "{}", self.as_string())
    }
}

// Automatic timer (prints time when dropped)
pub struct AutoTimer {
    timer: Timer,
}

impl AutoTimer {
    pub fn new(title: &str) -> Self {
        AutoTimer {
            timer: Timer::new(title),
        }
    }

    pub fn with_formatter(title: &str, time_print: TimePrintFn) -> Self {
        AutoTimer {
            timer: Timer::with_formatter(title, time_print),
        }
    }
}

impl Drop for AutoTimer {
    fn drop(&mut self) {
        // Now this will actually panic if the formatter does
        println!("{}", self.timer.as_string());
    }
}
