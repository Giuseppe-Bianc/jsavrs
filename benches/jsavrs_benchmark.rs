// rust
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use jsavrs::lexer::{Lexer, lexer_tokenize_with_errors};
use jsavrs::parser::jsav_parser::JsavParser;
use std::hint::black_box;
use std::time::Duration;

pub fn benchmark_example(c: &mut Criterion) {
    // Lexer-only con throughput (byte/s)
    let mut lex_group = c.benchmark_group("jsavrs-lexer");
    lex_group
        .significance_level(0.005)
        .sample_size(1000)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(15))
        .nresamples(500_000);

    let lex_cases = [
        ("simple", "var x: i32 = 42".to_string()),
        ("simple_long", "var x: i32 = 42\n".repeat(1000)),
        ("expression", "var y: i32 = (10 + 20) * (5 - 3) / 2".to_string()),
        ("expression_long", "var y: i32 = (10 + 20) * (5 - 3) / 2\n".repeat(1000)),
    ];

    for (name, input) in &lex_cases {
        lex_group.throughput(Throughput::Bytes(input.len() as u64));
        lex_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                let mut lexer = Lexer::new("bench.vn", black_box(input.as_str()));
                let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
                black_box(&tokens);
            })
        });
    }
    lex_group.finish();

    // Lexer + Parser (end-to-end) con throughput (byte/s)
    let mut parse_group = c.benchmark_group("jsavrs-parser");
    parse_group
        .significance_level(0.005)
        .sample_size(1000)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(15))
        .nresamples(500_000);

    let parse_cases = [
        ("simple", "var x: i32 = 42".to_string()),
        ("simple_long", "var x: i32 = 42\n".repeat(1000)),
        ("expression", "var y: i32 = (10 + 20) * (5 - 3) / 2".to_string()),
        ("expression_long", "var y: i32 = (10 + 20) * (5 - 3) / 2\n".repeat(1000)),
    ];

    for (name, input) in &parse_cases {
        parse_group.throughput(Throughput::Bytes(input.len() as u64));
        parse_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                let mut lexer = Lexer::new("bench.vn", black_box(input.as_str()));
                let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
                let parser = JsavParser::new(tokens);
                let ast_and_errors = parser.parse();
                black_box(&ast_and_errors);
            })
        });
    }
    parse_group.finish();
}

criterion_group!(benches, benchmark_example);
criterion_main!(benches);
