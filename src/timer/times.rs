// Contenitore per i valori temporali con etichette
use crate::timer::time_values::TimeValues;
use crate::timer::value_label::ValueLabel;
#[derive(Debug, Clone)]
pub struct Times {
    pub values: TimeValues,
    pub label_seconds: &'static str,
    pub label_millis: &'static str,
    pub label_micro: &'static str,
    pub label_nano: &'static str,
}

impl Times {
    pub fn from_nanoseconds(nanoseconds: f64) -> Self {
        Times {
            values: TimeValues::from_nanoseconds(nanoseconds),
            label_seconds: "s",
            label_millis: "ms",
            label_micro: "us",
            label_nano: "ns",
        }
    }

    pub fn get_relevant_timeframe(&self) -> ValueLabel {
        let s = self.values.seconds();
        let ms = self.values.millis();
        let us = self.values.micro();

        // Modifica: usare >= 1.0 invece di > 1.0
        if s >= 1.0 {
            ValueLabel::new(s, self.label_seconds)
        } else if ms >= 1.0 {
            ValueLabel::new(ms, self.label_millis)
        } else if us >= 1.0 {
            ValueLabel::new(us, self.label_micro)
        } else {
            ValueLabel::new(self.values.nano(), self.label_nano)
        }
    }
}

pub type TimePrintFn = fn(&str, usize, &ValueLabel) -> String;

pub fn simple_format(title: &str, _: usize, time: &ValueLabel) -> String {
    format!("{}: Time = {}", title, time)
}

pub fn big_format(title: &str, title_len: usize, time: &ValueLabel) -> String {
    let time_str = format!("Time = {}", time);
    let total_len = title_len + time_str.len() + 3;
    let title_section = format!("|{: ^title_len$}|{: ^time_len$}|",
                                title, time_str,
                                title_len = title_len - 4,
                                time_len = time_str.len() + 1
    );
    format!("\n{:-<total_len$}\n{}\n{:-<total_len$}", "", title_section, "")
}