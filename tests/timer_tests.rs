use jsavrs::time::time_values::TimeValues;
use jsavrs::time::timer::{AutoTimer, Timer};
use jsavrs::time::times::{Times, big_format};
use jsavrs::time::value_label::ValueLabel;
use std::thread;
use std::time::Duration;

// Test helper: funzione che consuma tempo in modo controllato
fn timed_task(duration_ms: u64) {
    thread::sleep(Duration::from_millis(duration_ms));
}

#[test]
#[allow(clippy::float_cmp)]
fn test_time_values_from_nanoseconds() {
    // Valore normale
    let tv = TimeValues::from_nanoseconds(1_500_000_000.0);
    assert!((tv.seconds() - 1.5).abs() < f64::EPSILON);
    assert!((tv.millis() - 1500.0).abs() < f64::EPSILON);
    assert!((tv.micro() - 1_500_000.0).abs() < f64::EPSILON);
    assert!((tv.nano() - 1_500_000_000.0).abs() < f64::EPSILON);

    // Edge case: zero
    let tv = TimeValues::from_nanoseconds(0.0);
    assert_eq!(tv.seconds(), 0.0);
    assert_eq!(tv.millis(), 0.0);
    assert_eq!(tv.micro(), 0.0);
    assert_eq!(tv.nano(), 0.0);

    // Edge case: negativo (dovrebbe gestire ma non supportato)
    let tv = TimeValues::from_nanoseconds(-100.0);
    assert!(tv.seconds().is_sign_negative());
}

#[test]
fn test_value_label_formatting() {
    // Test per diversi range temporali
    let test_cases = vec![
        (0.0, "ns", "0ns"),
        (1.5, "ns", "2ns"), // arrotondamento
        (999.0, "ns", "999ns"),
        (1_000.0, "ns", "1000ns"),
        (1.0, "us", "1μs,0ns"),
        (1.5, "us", "1μs,500ns"),
        (999.5, "us", "999μs,500ns"),
        (1.0, "ms", "1ms,0μs,0ns"),
        (1.5, "ms", "1ms,500μs,0ns"),
        (999.5, "ms", "999ms,500μs,0ns"),
        (1.0, "s", "1s,0ms,0μs,0ns"),
        (1.5, "s", "1s,500ms,0μs,0ns"),
        (123.456, "s", "123s,456ms,0μs,0ns"), // verifica precisione
    ];

    for (value, unit, expected) in test_cases {
        let vl = ValueLabel::new(value, unit);
        assert_eq!(vl.format_time(), expected, "Failed for {value} {unit}");
    }

    // Edge case: unità sconosciuta
    let vl = ValueLabel::new(1.23, "unknown");
    assert_eq!(vl.format_time(), "1.230 unknown");
}

#[test]
fn test_times_relevant_timeframe() {
    let test_cases = vec![
        (0.0, "ns"),             // 0 ns
        (0.999, "ns"),           // 0.999 ns
        (1.0, "ns"),             // 1 ns = 0.001 μs (minore di 1μs)
        (999.999, "ns"),         // 999.999 ns = 0.999999 μs (minore di 1μs)
        (1000.0, "us"),          // 1000 ns = 1 μs
        (1000.0001, "us"),       // 1000.0001 ns = 1.0000001 μs
        (999_999.999, "us"),     // 999999.999 ns = 999.999999 μs
        (1_000_000.0, "ms"),     // 1,000,000 ns = 1 ms
        (1_000_000.000_1, "ms"), // 1,000,000.0001 ns = 1.0000000001 ms
        (999_999_999.999, "ms"), // 999,999,999.999 ns = 999.999999999 ms
        (1_000_000_000.0, "s"),  // 1,000,000,000 ns = 1 s
    ];

    for (nanos, expected_unit) in test_cases {
        let times = Times::from_nanoseconds(nanos);
        let vl = times.get_relevant_timeframe();
        assert_eq!(
            vl.time_label(),
            expected_unit,
            "Failed for {nanos} ns: expected {expected_unit}, got {}",
            vl.time_label()
        );
    }
}
#[test]
fn test_timer_basic() {
    let timer = Timer::new("Basic Timer");
    timed_task(50);
    let elapsed = timer.elapsed();

    // Verifica che il tempo misurato sia ragionevole
    assert!(elapsed.as_millis() >= 50);
    assert!(elapsed.as_millis() < 500); // Con un margine generoso

    // Verifica la formattazione stringa
    let time_str = timer.to_string();
    assert!(time_str.contains("Basic Timer"));
    assert!(time_str.contains("Time = "));
}

#[test]
fn test_auto_timer() {
    // Per testare AutoTimer dovremmo catturare stdout
    // In questo test ci limitiamo a verificare che non crashi
    let _timer = AutoTimer::new("AutoTimer Test");
    timed_task(30);
}

#[test]
fn test_auto_timer_big() {
    // Per testare AutoTimer dovremmo catturare stdout
    // In questo test ci limitiamo a verificare che non crashi
    let _timer = AutoTimer::with_formatter("AutoTimer Test", big_format);
    timed_task(30);
}

#[should_panic(expected = "Cannot divide timer by zero")]
#[test]
fn test_timer_divide_by_zero() {
    let timer = Timer::new("Average Timer");
    timed_task(100);

    // Divisione che consuma il timer originale e ne restituisce uno nuovo
    let avg_timer = timer / 0;
    let _time_str = avg_timer.to_string();
}

#[should_panic(expected = "Cannot divide timer by zero")]
#[test]
fn test_timer_divide_equal_by_zero() {
    let mut timer = Timer::new("Average Timer");
    timed_task(100);

    timer /= 0;
    let _time_str = timer.to_string();
}

#[test]
fn test_timer_division() {
    // Test per la divisione che crea un nuovo timer
    {
        let timer = Timer::new("Average Timer");
        timed_task(100);

        // Divisione che consuma il timer originale e ne restituisce uno nuovo
        let avg_timer = timer / 10;
        let time_str = avg_timer.to_string();
        assert!(time_str.contains("Average Timer"));
        assert!(time_str.contains("Time = "));
    }

    // Test per la divisione con assegnazione
    {
        let mut timer = Timer::new("Average Timer");
        timed_task(100);

        // Operatore di assegnazione che modifica il timer esistente
        timer /= 5;
        let time_str = timer.to_string();
        assert!(time_str.contains("Average Timer"));
        assert!(time_str.contains("Time = "));
    }
}

#[test]
fn test_time_it() {
    let mut timer = Timer::new("TimeIt Test");
    let result = timer.time_it(
        || timed_task(10),
        0.05, // Target di 50ms
    );

    // Estrai il numero di tentativi dalla stringa
    let tries_str = result
        .split_whitespace()
        .find(|s| s.parse::<u32>().is_ok()) // Cerca la prima parola che è un numero
        .expect("No number found in result string");

    let tries = tries_str.parse::<u32>().expect("Failed to parse tries as u32");

    // Verifica che abbia eseguito molte iterazioni
    assert!(tries >= 1, "Expected >=1 tries, got {tries}");
}

#[test]
fn test_formatters() {
    let timer = Timer::with_formatter("Custom Formatter", |title, _, time| format!("CUSTOM: {time} - {title}"));
    timed_task(20);

    let output = timer.to_string();
    assert!(output.starts_with("CUSTOM:"));
    assert!(output.contains("Custom Formatter"));
    assert!(output.contains("ms") || output.contains("us") || output.contains("ns"));
}

#[test]
fn test_big_format() {
    let timer = Timer::with_formatter("Big Format Test", big_format);
    timed_task(30);

    let output = timer.to_string();
    assert!(output.contains("Big Format Test"));
    assert!(output.contains("Time = "));
    assert!(output.contains("ms") || output.contains("us") || output.contains("ns"));
}

#[test]
#[allow(clippy::similar_names)]
fn test_edge_cases() {
    // Tempo molto piccolo (<1ns)
    let vl = ValueLabel::new(0.4, "ns");
    assert_eq!(vl.format_time(), "0ns"); // Arrotondamento

    // Tempo molto grande (>1000s)
    let tv = TimeValues::from_nanoseconds(3.6e15); // 1 ora
    let times = Times { values: tv, ..Times::from_nanoseconds(0.0) };
    let vl = times.get_relevant_timeframe();
    assert_eq!(vl.time_label(), "s");
    assert!(vl.time_val() > 3600.0);

    // Divisione per zero (dovrebbe essere prevenuta)
    let mut timer = Timer::new("Division by Zero");
    timed_task(10);
    timer /= 1; // Dovrebbe essere sicuro
}

#[test]
fn test_concurrent_timers() {
    // Verifica che i timer possano essere usati in contesti multi-thread
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let timer = Timer::new(&format!("Thread {i}"));
                timed_task(10 + i * 5);
                timer.to_string()
            })
        })
        .collect();

    for handle in handles {
        let time_str = handle.join().unwrap();
        assert!(!time_str.is_empty());
        assert!(time_str.contains("Thread"));
    }
}

#[test]
fn test_high_precision() {
    // Verifica la precisione per tempi molto brevi
    let timer = Timer::new("High Precision");
    for _ in 0..1000 {
        // Operazione vuota
    }
    let time_str = timer.to_string();

    // Dovrebbe essere nell'ordine dei microsecondi o nanosecondi
    assert!(time_str.contains("μs") || time_str.contains("us") || time_str.contains("ns"));
}

#[test]
fn test_long_running() {
    // Test per operazioni lunghe (>2 secondi)
    let timer = Timer::new("Long Running");
    timed_task(2500);
    let time_str = timer.to_string();

    // Dovrebbe usare i secondi come unità
    assert!(time_str.contains("s,"));
    assert!(time_str.contains('s'));
}

#[test]
fn test_time_it_short_operations() {
    let mut timer = Timer::new("Short Ops");
    let result = timer.time_it(
        || {
            // Aggiungiamo una piccola operazione per evitare ottimizzazioni
            let mut x = 0;
            for i in 0..100 {
                x += i;
            }
            std::hint::black_box(x);
        },
        0.01, // Aumentiamo il target time per avere più iterazioni
    );
    // Estrai il numero di tentativi dalla stringa
    let tries_str = result
        .split_whitespace()
        .find(|s| s.parse::<u32>().is_ok()) // Cerca la prima parola che è un numero
        .expect("No number found in result string");

    let tries = tries_str.parse::<u32>().expect("Failed to parse tries as u32");

    // Verifica che abbia eseguito molte iterazioni
    assert!(tries >= 100, "Expected >=100 tries, got {tries}");
}

// Custom formatter that panics
fn panic_formatter(_title: &str, _padding: usize, _time: &ValueLabel) -> String {
    panic!("test panic in formatter");
}

#[test]
#[should_panic(expected = "test panic in formatter")]
fn test_auto_timer_drop_panic() {
    // Create and immediately drop the AutoTimer
    {
        let _timer = AutoTimer::with_formatter("test", panic_formatter);
    }
}
