// Contenitore per i valori temporali con etichette
use crate::time::time_values::TimeValues;
use crate::time::value_label::ValueLabel;

#[derive(Debug, Clone)]
pub struct TimeLabels {
    pub seconds: &'static str,
    pub millis: &'static str,
    pub micro: &'static str,
    pub nano: &'static str,
}

impl Default for TimeLabels {
    fn default() -> Self {
        Self { seconds: "s", millis: "ms", micro: "us", nano: "ns" }
    }
}

#[derive(Debug, Clone)]
pub struct Times {
    pub values: TimeValues,
    pub labels: TimeLabels,
}

impl Times {
    pub fn from_nanoseconds(nanoseconds: f64) -> Self {
        Self { values: TimeValues::from_nanoseconds(nanoseconds), labels: TimeLabels::default() }
    }

    pub fn get_relevant_timeframe(&self) -> ValueLabel {
        let s = self.values.seconds();
        let ms = self.values.millis();
        let us = self.values.micro();

        // Modifica: usare >= 1.0 invece di > 1.0
        if s >= 1.0 {
            ValueLabel::new(s, self.labels.seconds)
        } else if ms >= 1.0 {
            ValueLabel::new(ms, self.labels.millis)
        } else if us >= 1.0 {
            ValueLabel::new(us, self.labels.micro)
        } else {
            ValueLabel::new(self.values.nano(), self.labels.nano)
        }
    }
}

pub type TimePrintFn = fn(&str, usize, &ValueLabel) -> String;

pub fn simple_format(title: &str, _: usize, time: &ValueLabel) -> String {
    format!("{title}: Time = {time}")
}

pub fn big_format(title: &str, title_len: usize, time: &ValueLabel) -> String {
    let time_str = format!("Time = {time}");
    let total_len = title_len + time_str.len() + 3; // +3 for separators
    let title_section = format!(
        "|{: ^title_len$}|{: ^time_len$}|",
        title,
        time_str,
        title_len = title_len.saturating_sub(4), // Prevent underflow
        time_len = time_str.len() + 1
    );
    format!("\n{:-<total_len$}\n{}\n{:-<total_len$}", "", title_section, "")
}
