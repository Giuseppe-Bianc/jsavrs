// benches/jsavrs_benchmark.rs
use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use jsavrs::ir::generator::NIrGenerator;
use jsavrs::ir::optimizer::{DeadCodeElimination, Phase};
use jsavrs::lexer::{Lexer, lexer_tokenize_with_errors};
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::semantic::type_checker::TypeChecker;
use std::hint::black_box;
use std::time::Duration;

/// Helper function to configure benchmark groups with standard settings
fn configure_benchmark_group(group: &mut BenchmarkGroup<WallTime>, warm_up: u64, measurement: u64) {
    group
        .significance_level(0.005)
        .sample_size(1000)
        .confidence_level(0.99)
        .warm_up_time(Duration::from_secs(warm_up))
        .measurement_time(Duration::from_secs(measurement))
        .nresamples(500_000);
}

pub fn benchmark_lexer(c: &mut Criterion) {
    let mut lex_group = c.benchmark_group("jsavrs-lexer");
    configure_benchmark_group(&mut lex_group, 5, 15);

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
    configure_benchmark_group(&mut parse_group, 5, 15);

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
    configure_benchmark_group(&mut node_group, 3, 10);

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
    configure_benchmark_group(&mut semantic_group, 3, 10);

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
    configure_benchmark_group(&mut ir_group, 3, 10);

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
    configure_benchmark_group(&mut pipeline_group, 5, 15);

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

/// Benchmark Dead Code Elimination optimization on small functions (<100 instructions)
pub fn benchmark_dce_small(c: &mut Criterion) {
    let mut dce_group = c.benchmark_group("jsavrs-dce-small");
    configure_benchmark_group(&mut dce_group, 5, 15);

    // Small function with unreachable code and dead variables
    let small_func = r#"
        fun test_small(x: i32): i32 {
            var dead1: i32 = 10;
            var dead2: i32 = 20;
            if false {
                var unreachable: i32 = 30;
                return unreachable;
            }
            return x;
        }
    "#;

    dce_group.bench_function("small_function", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new("bench.vn", black_box(small_func));
            let (tokens, _) = lexer_tokenize_with_errors(&mut lexer);
            let parser = JsavParser::new(tokens);
            let (ast, _) = parser.parse();
            let mut type_checker = TypeChecker::new();
            let _ = type_checker.check(&ast);
            let mut ir_generator = NIrGenerator::new();
            let (mut ir_module, _) = ir_generator.generate(ast, "bench.vn");

            let mut dce = DeadCodeElimination::default();
            dce.run(&mut ir_module);
            black_box(&ir_module);
        })
    });

    dce_group.finish();
}

/// Benchmark Dead Code Elimination optimization on medium functions (~1000 instructions)
pub fn benchmark_dce_medium(c: &mut Criterion) {
    let mut dce_group = c.benchmark_group("jsavrs-dce-medium");
    configure_benchmark_group(&mut dce_group, 5, 15);

    // Generate medium function with many dead variables (avoids deep nesting)
    let mut medium_func = String::from("fun test_medium(x: i32): i32 {\n");
    medium_func.push_str("    var result: i32 = x;\n");
    for i in 0..50 {
        medium_func.push_str(&format!("    var dead{}: i32 = {} + x;\n", i, i));
        medium_func.push_str(&format!("    var unused{}: i32 = dead{} + 1;\n", i, i));
    }
    medium_func.push_str("    return result;\n");
    medium_func.push_str("}\n");

    dce_group.bench_function("medium_function", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new("bench.vn", black_box(medium_func.as_str()));
            let (tokens, _) = lexer_tokenize_with_errors(&mut lexer);
            let parser = JsavParser::new(tokens);
            let (ast, _) = parser.parse();
            let mut type_checker = TypeChecker::new();
            let _ = type_checker.check(&ast);
            let mut ir_generator = NIrGenerator::new();
            let (mut ir_module, _) = ir_generator.generate(ast, "bench.vn");

            let mut dce = DeadCodeElimination::default();
            dce.run(&mut ir_module);
            black_box(&ir_module);
        })
    });

    dce_group.finish();
}
/// Benchmark Dead Code Elimination optimization on large functions (~10000 instructions)
pub fn benchmark_dce_large(c: &mut Criterion) {
    let mut dce_group = c.benchmark_group("jsavrs-dce-large");
    configure_benchmark_group(&mut dce_group, 5, 15);

    // Generate large function with many dead variables (avoids deep nesting)
    let mut large_func = String::from("fun test_large(x: i32): i32 {\n");
    large_func.push_str("    var result: i32 = x;\n");
    for i in 0..200 {
        large_func.push_str(&format!("    var dead{}: i32 = {} + x;\n", i, i));
        large_func.push_str(&format!("    var unused{}: i32 = dead{} + 1;\n", i, i));
    }
    large_func.push_str("    return result;\n");
    large_func.push_str("}\n");

    dce_group.bench_function("large_function", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new("bench.vn", black_box(large_func.as_str()));
            let (tokens, _) = lexer_tokenize_with_errors(&mut lexer);
            let parser = JsavParser::new(tokens);
            let (ast, _) = parser.parse();
            let mut type_checker = TypeChecker::new();
            let _ = type_checker.check(&ast);
            let mut ir_generator = NIrGenerator::new();
            let (mut ir_module, _) = ir_generator.generate(ast, "bench.vn");

            let mut dce = DeadCodeElimination::default();
            dce.run(&mut ir_module);
            black_box(&ir_module);
        })
    });

    dce_group.finish();
}

/// Benchmark Dead Code Elimination optimization on modules with multiple functions
pub fn benchmark_dce_module(c: &mut Criterion) {
    let mut dce_group = c.benchmark_group("jsavrs-dce-module");
    configure_benchmark_group(&mut dce_group, 5, 15);

    // Module with 10 functions, each with dead code (simplified to avoid stack overflow)
    let mut module_code = String::new();
    for func_idx in 0..10 {
        module_code.push_str(&format!("fun test_func{}(x: i32): i32 {{\n", func_idx));
        module_code.push_str("    var result: i32 = x;\n");
        for i in 0..10 {
            module_code.push_str(&format!("    var dead{}: i32 = {} + x;\n", i, i));
        }
        module_code.push_str("    return result;\n");
        module_code.push_str("}\n\n");
    }

    dce_group.bench_function("module_with_multiple_functions", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new("bench.vn", black_box(module_code.as_str()));
            let (tokens, _) = lexer_tokenize_with_errors(&mut lexer);
            let parser = JsavParser::new(tokens);
            let (ast, _) = parser.parse();
            let mut type_checker = TypeChecker::new();
            let _ = type_checker.check(&ast);
            let mut ir_generator = NIrGenerator::new();
            let (mut ir_module, _) = ir_generator.generate(ast, "bench.vn");

            let mut dce = DeadCodeElimination::default();
            dce.run(&mut ir_module);
            black_box(&ir_module);
        })
    });

    dce_group.finish();
}
/// Benchmark worst-case iteration count (deeply nested code)
pub fn benchmark_dce_worst_case(c: &mut Criterion) {
    let mut dce_group = c.benchmark_group("jsavrs-dce-worst-case");
    configure_benchmark_group(&mut dce_group, 5, 15);

    // Deep nesting with cascading dead code (requires multiple iterations)
    let mut worst_case = String::from("fun test_worst_case(x: i32): i32 {\n");
    for i in 0..20 {
        worst_case.push_str(&format!("    var v{}: i32 = x + {};\n", i, i));
        if i > 0 {
            worst_case.push_str(&format!("    var chain{}: i32 = v{};\n", i, i - 1));
        }
    }
    worst_case.push_str("    return x;\n");
    worst_case.push_str("}\n");

    dce_group.bench_function("worst_case_deep_nesting", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new("bench.vn", black_box(worst_case.as_str()));
            let (tokens, _) = lexer_tokenize_with_errors(&mut lexer);
            let parser = JsavParser::new(tokens);
            let (ast, _) = parser.parse();
            let mut type_checker = TypeChecker::new();
            let _ = type_checker.check(&ast);
            let mut ir_generator = NIrGenerator::new();
            let (mut ir_module, _) = ir_generator.generate(ast, "bench.vn");

            let mut dce = DeadCodeElimination::default();
            dce.run(&mut ir_module);
            black_box(&ir_module);
        })
    });

    dce_group.finish();
}

criterion_group!(
    benches,
    benchmark_lexer,
    benchmark_parser,
    benchmark_parser_nodes,
    benchmark_semantic_analysis,
    benchmark_ir_generation,
    benchmark_end_to_end,
    benchmark_dce_small,
    benchmark_dce_medium,
    benchmark_dce_large,
    benchmark_dce_module,
    benchmark_dce_worst_case
);
criterion_main!(benches);
