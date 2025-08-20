use std::hint::black_box;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use jsavrs;
use jsavrs::lexer::{lexer_tokenize_with_errors, Lexer};
// Assicurati che il nome della crate sia corretto

pub fn benchmark_example(c: &mut Criterion) {
    let mut group = c.benchmark_group("jsavrs-parser");
    // Imposta un livello di significatività più basso (0.1 invece del default 0.05)
    // Questo rende il test più rigoroso nel rilevare differenze [[3]]
    // Aumenta la dimensione del campione a 500 (rispetto al default di 100) per maggiore precisione [[2]]
    group
        .significance_level(0.005)
        .sample_size(1000)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(15))
        .nresamples(200_000)
    ;

    group.bench_function("parse_simple", |b| {
        b.iter(|| {
            let input = black_box("let x = 42;");
            let mut lexer = Lexer::new("test.vn", input);
            let tokens = lexer_tokenize_with_errors(&mut lexer);
            black_box(tokens);
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark_example);
criterion_main!(benches);