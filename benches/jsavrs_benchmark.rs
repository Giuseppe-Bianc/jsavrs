// rust
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use jsavrs::ir::generator::NIrGenerator;
use jsavrs::lexer::{Lexer, lexer_tokenize_with_errors};
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::semantic::type_checker::TypeChecker;
use std::hint::black_box;
use std::time::Duration;

pub fn benchmark_lexer(c: &mut Criterion) {
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
        ("simple", "var x: i64 = 42".to_string()),
        ("simple_long", "var x: i64 = 42\n".repeat(1000)),
        ("expression", "var y: i64 = (10 + 20) * (5 - 3) / 2".to_string()),
        ("expression_long", "var y: i64 = (10 + 20) * (5 - 3) / 2\n".repeat(1000)),
        (
            "complex_function",
            "fun fibonacci(n: i32): i32 { if n <= 1 { return n; } return fibonacci(n - 1) + fibonacci(n - 2); }"
                .to_string(),
        ),
        (
            "complex_function_long",
            "fun fibonacci(n: i32): i32 { if n <= 1 { return n; } return fibonacci(n - 1) + fibonacci(n - 2); }\n"
                .repeat(100),
        ),
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
}

pub fn benchmark_parser(c: &mut Criterion) {
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
        ("simple", "var x: i64 = 42".to_string()),
        ("simple_long", "var x: i64 = 42\n".repeat(1000)),
        ("expression", "var y: i64 = (10 + 20) * (5 - 3) / 2".to_string()),
        ("expression_long", "var y: i64 = (10 + 20) * (5 - 3) / 2\n".repeat(1000)),
        ("expressions", "var y: i64 = (10 + 20) * (5 - 3) / 2\nvar y: i64 = -42 + 5 ^ 3".to_string()),
        ("expressions_long", "var y: i64 = (10 + 20) * (5 - 3) / 2\nvar y: i64 = -42 + 5 ^ 3".repeat(1000)),
        ("function", "fun add(a: i32, b: i32): i32 { return a + b; }".to_string()),
        ("function_long", "fun add(a: i32, b: i32): i32 { return a + b; }\n".repeat(100)),
        ("control_flow", "fun test(n: i32): i32 { if n > 0 { while n > 0 { n = n - 1; } } else { for i in 0..10 { n = n + i; } } return n; }".to_string()),
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

pub fn benchmark_parser_nodes(c: &mut Criterion) {
    // Parser benchmarks for specific AST node types
    let mut node_group = c.benchmark_group("jsavrs-parser-nodes");
    node_group
        .significance_level(0.005)
        .sample_size(1000)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(10))
        .nresamples(200_000);

    let node_cases = [
        ("binary_expr", "var x: i64 = 10 + 20 * 30 - 40 / 50".to_string()),
        ("unary_expr", "var x: i64 = -42 + ~5 ^ 3".to_string()),
        ("function_call", "var x: i64 = fibonacci(10) + factorial(5)".to_string()),
        ("array_access", "var x: i64 = arr[0] + arr[i + 1]".to_string()),
        ("struct_access", "var x: i64 = obj.field + obj.nested.value".to_string()),
    ];

    for (name, input) in &node_cases {
        node_group.throughput(Throughput::Bytes(input.len() as u64));
        node_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                let mut lexer = Lexer::new("bench.vn", black_box(input.as_str()));
                let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
                let parser = JsavParser::new(tokens);
                let ast_and_errors = parser.parse();
                black_box(&ast_and_errors);
            })
        });
    }
    node_group.finish();
}

pub fn benchmark_semantic_analysis(c: &mut Criterion) {
    // Semantic analysis benchmarks
    let mut semantic_group = c.benchmark_group("jsavrs-semantic");
    semantic_group
        .significance_level(0.005)
        .sample_size(500)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(10))
        .nresamples(100_000);

    let semantic_cases = [
        ("simple_types", "var x: i64 = 42\nvar y: f64 = 3.14\nvar z: bool = true".to_string()),
        (
            "function_types",
            "fun add(a: i32, b: i32): i32 { return a + b; }\nfun mul(a: f64, b: f64): f64 { return a * b; }"
                .to_string(),
        ),
        ("complex_types", "fun complex(a: i32[], b: {x: i32, y: i32}): i32 { return a[0] + b.x + b.y; }".to_string()),
        ("type_errors", "var x: i64 = 3.14\nvar y: bool = 42\nvar z: string = 10".to_string()),
    ];

    for (name, input) in &semantic_cases {
        semantic_group.throughput(Throughput::Bytes(input.len() as u64));
        semantic_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                let mut lexer = Lexer::new("bench.vn", black_box(input.as_str()));
                let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
                let parser = JsavParser::new(tokens);
                let (ast, _parse_errors) = parser.parse();
                let mut type_checker = TypeChecker::new();
                let semantic_errors = type_checker.check(&ast);
                black_box(&semantic_errors);
            })
        });
    }
    semantic_group.finish();
}

pub fn benchmark_ir_generation(c: &mut Criterion) {
    // IR generation benchmarks
    let mut ir_group = c.benchmark_group("jsavrs-ir-generation");
    ir_group
        .significance_level(0.005)
        .sample_size(500)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(10))
        .nresamples(100_000);

    let ir_cases = [
        ("simple_expr", "var x: i64 = 42\nvar y: i64 = x + 10".to_string()),
        (
            "functions",
            "fun add(a: i32, b: i32): i32 { return a + b; }\nfun main() { var result: i32 = add(5, 10); }".to_string(),
        ),
        (
            "control_flow",
            "fun loop_test(n: i32): i32 { var sum: i32 = 0; for i in 0..n { sum = sum + i; } return sum; }".to_string(),
        ),
    ];

    for (name, input) in &ir_cases {
        ir_group.throughput(Throughput::Bytes(input.len() as u64));
        ir_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                let mut lexer = Lexer::new("bench.vn", black_box(input.as_str()));
                let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
                let parser = JsavParser::new(tokens);
                let (ast, _parse_errors) = parser.parse();
                let mut ir_generator = NIrGenerator::new();
                let (ir_module, _ir_errors) = ir_generator.generate(ast, "bench.vn");
                black_box(&ir_module);
            })
        });
    }
    ir_group.finish();
}

pub fn benchmark_end_to_end(c: &mut Criterion) {
    // End-to-end compilation pipeline benchmarks
    let mut pipeline_group = c.benchmark_group("jsavrs-end-to-end");
    pipeline_group
        .significance_level(0.005)
        .sample_size(200)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(15))
        .nresamples(50_000);

    let pipeline_cases = [
        ("simple", "var x: i64 = 42".to_string()),
        ("function", "fun add(a: i32, b: i32): i32 { return a + b; }".to_string()),
        (
            "complex",
            "fun fibonacci(n: i32): i32 { if n <= 1 { return n; } return fibonacci(n - 1) + fibonacci(n - 2); }"
                .to_string(),
        ),
        ("long", "fun add(a: i32, b: i32): i32 { return a + b; }\n".repeat(100)),
    ];

    for (name, input) in &pipeline_cases {
        pipeline_group.throughput(Throughput::Bytes(input.len() as u64));
        pipeline_group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| {
                let mut lexer = Lexer::new("bench.vn", black_box(input.as_str()));
                let (tokens, _lex_errors) = lexer_tokenize_with_errors(&mut lexer);
                let parser = JsavParser::new(tokens);
                let (ast, _parse_errors) = parser.parse();
                let mut type_checker = TypeChecker::new();
                let _semantic_errors = type_checker.check(&ast);
                let mut ir_generator = NIrGenerator::new();
                let (ir_module, _ir_errors) = ir_generator.generate(ast, "bench.vn");
                black_box(&ir_module);
            })
        });
    }
    pipeline_group.finish();
}

criterion_group!(
    benches,
    benchmark_lexer,
    benchmark_parser,
    benchmark_parser_nodes,
    benchmark_semantic_analysis,
    benchmark_ir_generation,
    benchmark_end_to_end
);
criterion_main!(benches);
