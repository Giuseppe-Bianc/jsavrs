use std::thread;
use std::time::Duration;
use insta::{assert_debug_snapshot, assert_snapshot, with_settings};
use jsavrs::time::timer::Timer;
use jsavrs::time::times::{big_format, Times};
use jsavrs::time::value_label::ValueLabel;

// Test helper: funzione che consuma tempo in modo controllato
fn timed_task(duration_ms: u64) {
    thread::sleep(Duration::from_millis(duration_ms));
}


#[test]
fn test_value_label_snapshots() {
    let test_cases = [
        (0.0, "ns"),
        (1.5, "ns"),
        (999.0, "ns"),
        (1000.0, "ns"),
        (1.0, "us"),
        (1.5, "us"),
        (999.5, "us"),
        (1.0, "ms"),
        (1.5, "ms"),
        (999.5, "ms"),
        (1.0, "s"),
        (1.5, "s"),
        (123.456, "s"),
        (0.4, "ns"),   // Edge: rounds to 0ns
        (1.234567, "ms"), // Detailed breakdown
    ];

    for (i, (value, unit)) in test_cases.iter().enumerate() {
        let vl = ValueLabel::new(*value, unit);
        let result = vl.format_time();
        assert_snapshot!(format!("value_label_{}_{}_{}", i, value, unit), &result);
    }

    // Unknown unit
    let vl = ValueLabel::new(1.23, "unknown");
    assert_snapshot!("value_label_unknown_unit", &vl.format_time());
}

#[test]
fn test_times_snapshots() {
    let test_cases = [
        0.0,          // 0ns
        0.999,        // 0.999ns
        1.0,          // 1ns
        999.999,      // 999.999ns
        1000.0,       // 1us
        1000.0001,    // 1.0000001us
        999_999.999,  // 999.999999us
        1_000_000.0,  // 1ms
        1_000_000.0001, // 1.0000000001ms
        999_999_999.999, // 999.999999999ms
        1_000_000_000.0, // 1s
        3.6e15,       // 1 hour (large value)
    ];

    for (i, nanos) in test_cases.iter().enumerate() {
        let times = Times::from_nanoseconds(*nanos);
        assert_debug_snapshot!(format!("times_{}_{}ns", i, nanos), &times);
    }
}

#[test]
fn test_timer_formatting_snapshots() {
    // Test different formatters
    let timer = Timer::with_formatter("Custom Format", |title, _, _| {
        format!("[CUSTOM] {}: 123.456ms", title)
    });
    assert_snapshot!("timer_custom_formatter", &timer.to_string());
}

// A helper to run one invocation of the test under a custom `Settings`
// that has a filter to replace all occurrences of, e.g., "123.456ms" with "<TIME>"
#[test]
fn test_timer_big_format_with_filters() {
    // Build a Settings object with a filter that replaces any “,<digits>μs,<digits>ns”
    // substring with “,Time”
    let settings = {
        let mut s = insta::Settings::clone_current();
        s.add_filter(r",\d+μs,\d+ns", ",Time");
        s
    };

    // Bind these settings for the duration of the closure
    settings.bind(|| {
        let timer = Timer::with_formatter("Big Format", big_format);
        timed_task(1);
        let output = timer.to_string();
        assert_snapshot!("timer_big_format", output);
        // On disk, the snapshot will now contain:
        //
        // ---------------------------------------------- 
        // |   Big Format   | Time = 1ms,Time | 
        // ----------------------------------------------
    });
}
