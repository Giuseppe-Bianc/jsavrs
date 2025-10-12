# Quick Start Guide - jsavrs Tracing System

**Feature**: Centralized Tracing System Configuration  
**Date**: 2025-10-11  
**Audience**: Developers using or extending the jsavrs compiler

## Overview

This guide provides practical examples for using the jsavrs tracing system in common scenarios. Whether you're debugging compilation issues, measuring performance, or integrating the compiler as a library, these examples will get you started quickly.

---

## Table of Contents

1. [Basic Usage](#basic-usage)
2. [Command-Line Interface](#command-line-interface)
3. [Library Integration](#library-integration)
4. [Filtering Traces](#filtering-traces)
5. [Custom Formatting](#custom-formatting)
6. [Performance Measurement](#performance-measurement)
7. [Troubleshooting](#troubleshooting)

---

## Basic Usage

### Enabling Tracing with --verbose

The simplest way to enable tracing is using the existing `--verbose` flag:

```bash
# Compile with basic tracing (INFO level)
jsavrs --input program.vn --verbose

# Example output:
# INFO lexer: Tokenizing input file
#      │ file: program.vn
#      │ size: 1.2 KB
#      │ duration: 125μs
#      └─ 47 tokens generated
# 
# INFO parser: Parsing statements
#      │ tokens: 47
#      │ duration: 380μs
#      └─ 12 statements parsed
# 
# INFO semantic: Type checking
#      │ statements: 12
#      │ duration: 215μs
#      └─ No errors found
```

### Disabling Tracing (Default)

By default, tracing is completely disabled with zero overhead:

```bash
# Standard compilation without tracing
jsavrs --input program.vn

# Output:
# total of bytes read: 1.2 KB
# 47 tokens found
# parsing done
# 12 statements found
# type checking done
# NIR generation done
```

---

## Command-Line Interface

### Current CLI Options

```bash
jsavrs --help

# Usage: jsavrs [OPTIONS] --input <INPUT>
# 
# Options:
#   -i, --input <INPUT>    Input source file path
#   -v, --verbose          Enable detailed tracing output
#   -h, --help             Print help information
#   -V, --version          Print version information
```

### Future CLI Enhancements (Phase 2+)

```bash
# Graduated trace levels (future)
jsavrs --input program.vn --trace-level debug

# Custom output destination (future)
jsavrs --input program.vn --verbose --trace-output trace.log

# Phase-specific tracing (future)
jsavrs --input program.vn --trace-phases lexer,parser

# JSON output for log aggregation (future)
jsavrs --input program.vn --trace-format json
```

---

## Library Integration

### Initializing Tracing in Your Application

If you're embedding jsavrs as a library, initialize tracing programmatically:

```rust
use jsavrs::tracing::{initialize_tracing, TraceConfig, TraceLevel, TraceFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple initialization with defaults
    jsavrs::tracing::initialize_tracing(true)?;
    
    // Your application code using jsavrs
    compile_my_program()?;
    
    Ok(())
}
```

### Advanced Configuration

```rust
use jsavrs::tracing::{
    TraceConfig, TraceLevel, TraceOutput, TraceFormat, TraceFilter, CompilationPhase
};

fn setup_advanced_tracing() -> Result<(), jsavrs::tracing::TracingError> {
    // Build custom configuration
    let config = TraceConfig::builder()
        .enabled(true)
        .level(TraceLevel::Debug)
        .output(TraceOutput::File("compiler_trace.log".into()))
        .format(TraceFormat::Detailed)
        .filter(TraceFilter::Phases(vec![
            CompilationPhase::Parser,
            CompilationPhase::Semantic,
        ]))
        .build()?;
    
    // Initialize with custom config
    jsavrs::tracing::initialize_with_config(config)?;
    
    Ok(())
}
```

### Error Handling

```rust
use jsavrs::tracing::{initialize_tracing, TracingError};

fn safe_tracing_init() {
    match initialize_tracing(true) {
        Ok(()) => {
            println!("Tracing initialized successfully");
        }
        Err(TracingError::SubscriberAlreadySet) => {
            // Tracing already initialized - this is fine
            eprintln!("Warning: Tracing was already initialized");
        }
        Err(e) => {
            // Other errors - log but continue
            eprintln!("Warning: Failed to initialize tracing: {}", e);
            eprintln!("Continuing without tracing...");
        }
    }
}
```

---

## Filtering Traces

### By Compilation Phase

```rust
use jsavrs::tracing::{TraceConfig, TraceFilter, CompilationPhase};

// Trace only lexer and parser
let config = TraceConfig::builder()
    .enabled(true)
    .filter(TraceFilter::Phases(vec![
        CompilationPhase::Lexer,
        CompilationPhase::Parser,
    ]))
    .build()?;
```

### By Module Path

```rust
// Trace only specific modules
let config = TraceConfig::builder()
    .enabled(true)
    .filter(TraceFilter::Modules(vec![
        "jsavrs::parser::jsav_parser".to_string(),
        "jsavrs::semantic::type_checker".to_string(),
    ]))
    .build()?;
```

### By Source File

```rust
use std::path::PathBuf;

// Trace only operations on specific files
let config = TraceConfig::builder()
    .enabled(true)
    .filter(TraceFilter::Files(vec![
        PathBuf::from("src/problematic_file.vn"),
    ]))
    .build()?;
```

### Custom Filter Directives

```rust
// Advanced filtering using tracing_subscriber syntax
let config = TraceConfig::builder()
    .enabled(true)
    .filter(TraceFilter::Custom(
        "jsavrs=debug,jsavrs::parser=trace".to_string()
    ))
    .build()?;
```

---

## Custom Formatting

### Available Formats

```rust
use jsavrs::tracing::{TraceConfig, TraceFormat};

// 1. Error Reporter Style (default)
// Visual hierarchy matching error output
let config = TraceConfig::builder()
    .format(TraceFormat::ErrorReporterStyle)
    .build()?;

// Example output:
// INFO parser: Parsing expression
//      │ type: binary_op
//      │ operator: +
//      │ duration: 45μs
//      └─ expression parsed successfully

// 2. Compact Format
// Single-line, grep-friendly
let config = TraceConfig::builder()
    .format(TraceFormat::Compact)
    .build()?;

// Example output:
// INFO parser type=binary_op operator=+ duration=45μs message="expression parsed"

// 3. Detailed Format
// Multi-line with full context
let config = TraceConfig::builder()
    .format(TraceFormat::Detailed)
    .build()?;

// Example output:
// 2025-10-11T10:30:45.123456Z  INFO parser: Parsing expression
//     at src/parser/jsav_parser.rs:234
//     in compilation_root::parser_phase
//     fields:
//         type: binary_op
//         operator: +
//         duration: 45μs

// 4. JSON Format
// Structured data for log aggregation
let config = TraceConfig::builder()
    .format(TraceFormat::Json)
    .build()?;

// Example output:
// {"timestamp":"2025-10-11T10:30:45.123456Z","level":"INFO","target":"jsavrs::parser","fields":{"message":"Parsing expression","type":"binary_op","operator":"+","duration_us":45}}

// 5. Plain Format
// No colors, for piping or redirecting
let config = TraceConfig::builder()
    .format(TraceFormat::Plain)
    .build()?;

// Example output:
// INFO parser: Parsing expression [type=binary_op operator=+ duration=45μs]
```

---

## Performance Measurement

### Benchmarking with Tracing Disabled

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jsavrs::compile_source;

fn benchmark_compilation(c: &mut Criterion) {
    let source = include_str!("../test_fixtures/large_program.vn");
    
    c.bench_function("compile_no_tracing", |b| {
        b.iter(|| {
            compile_source(black_box(source))
        });
    });
}

criterion_group!(benches, benchmark_compilation);
criterion_main!(benches);
```

### Measuring Tracing Overhead

```rust
fn benchmark_with_tracing(c: &mut Criterion) {
    let source = include_str!("../test_fixtures/large_program.vn");
    
    let mut group = c.benchmark_group("tracing_overhead");
    
    // Baseline
    group.bench_function("no_tracing", |b| {
        b.iter(|| compile_source(black_box(source)));
    });
    
    // With INFO level
    group.bench_function("info_level", |b| {
        jsavrs::tracing::initialize_with_level(TraceLevel::Info).unwrap();
        b.iter(|| compile_source(black_box(source)));
    });
    
    // With DEBUG level
    group.bench_function("debug_level", |b| {
        jsavrs::tracing::initialize_with_level(TraceLevel::Debug).unwrap();
        b.iter(|| compile_source(black_box(source)));
    });
    
    group.finish();
}
```

### Capturing Timing Information

```rust
use tracing::{info_span, info};

fn compile_with_timing() {
    let span = info_span!("full_compilation");
    let _guard = span.enter();
    
    // Compilation phases are automatically timed
    lexer_phase();    // Reports: duration: 125μs
    parser_phase();   // Reports: duration: 380μs
    semantic_phase(); // Reports: duration: 215μs
    
    info!("Total compilation completed");
    // Automatically includes total span duration
}
```

---

## Troubleshooting

### Tracing Not Showing Output

**Problem**: Running with `--verbose` but seeing no trace output.

**Solution**:
```rust
// Check if tracing is actually enabled
if !jsavrs::tracing::is_initialized() {
    eprintln!("Warning: Tracing not initialized");
}

// Verify configuration
let config = jsavrs::tracing::get_config();
assert!(config.enabled, "Tracing should be enabled");
assert!(config.level <= TraceLevel::Info, "Level should be Info or lower");
```

### Output Destination Unavailable

**Problem**: File output fails due to permissions or disk space.

**Behavior**: Automatic failover to stderr with warning:
```
Warning: Could not create trace log file, using stderr
INFO lexer: Tokenizing input file
...
```

**Manual Failover Configuration**:
```rust
let config = TraceConfig::builder()
    .output(
        TraceOutput::File("trace.log".into())
            .with_stderr_failover()
    )
    .build()?;
```

### Performance Degradation

**Problem**: Compilation noticeably slower with tracing enabled.

**Diagnosis**:
```bash
# Run benchmark to measure actual overhead
cargo bench -- tracing_overhead

# Expected results:
# no_tracing:    time: [95.2ms 96.1ms 97.0ms]
# info_level:    time: [99.8ms 100.5ms 101.2ms] (overhead: ~4.5%)
# debug_level:   time: [105.3ms 106.8ms 108.1ms] (overhead: ~11%)
```

**Solutions**:
- Use INFO level instead of DEBUG for normal use
- Filter to specific phases: `--trace-phases parser`
- Disable tracing for production builds

### Trace Output Cluttering Console

**Problem**: Too much trace output making errors hard to find.

**Solutions**:

```bash
# 1. Redirect traces to file, errors to console
jsavrs --input program.vn --verbose 2>trace.log

# 2. Filter to specific phases
jsavrs --input program.vn --trace-phases semantic,type_checker

# 3. Use grep to find specific information
jsavrs --input program.vn --verbose 2>&1 | grep ERROR

# 4. Use compact format for less visual noise
# (requires TraceFormat::Compact in config)
```

### Integration Test Failures

**Problem**: Tests fail when tracing is enabled.

**Solution**: Initialize tracing once per test process:
```rust
use std::sync::Once;

static INIT: Once = Once::new();

fn init_tracing_once() {
    INIT.call_once(|| {
        // This runs only once, even if called from multiple tests
        let _ = jsavrs::tracing::initialize_tracing(true);
    });
}

#[test]
fn test_with_tracing() {
    init_tracing_once();
    
    // Test code here
    // Tracing is available but initialized only once
}
```

---

## Best Practices

### 1. Development Workflow

```bash
# During active development - detailed traces
cargo run -- --input test.vn --verbose

# When debugging specific phase - filtered traces
cargo run -- --input test.vn --trace-phases parser

# Performance testing - measure overhead
cargo bench -- tracing_overhead

# CI/CD - no tracing unless test fails
cargo test  # tracing automatically enabled on failure
```

### 2. Production Deployment

```rust
// Production: tracing disabled by default
// Enable only when needed for diagnostics
fn main() {
    let args = Args::parse();
    
    // Only enable if explicitly requested
    if args.verbose {
        let _ = jsavrs::tracing::initialize_tracing(true);
    }
    
    // Compilation proceeds normally
    compile(&args.input)?;
}
```

### 3. Library Integration

```rust
// Allow library consumers to control tracing
pub fn compile_with_tracing(
    source: &str,
    enable_tracing: bool,
) -> Result<CompiledProgram, CompileError> {
    if enable_tracing && !jsavrs::tracing::is_initialized() {
        jsavrs::tracing::initialize_tracing(true)?;
    }
    
    // Compilation with optional tracing
    compile_internal(source)
}
```

### 4. Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use jsavrs::tracing::initialize_for_testing;
    
    #[test]
    fn test_parser_with_traces() {
        // Enable tracing for this test
        initialize_for_testing();
        
        let result = parse_source("let x = 42;");
        
        // Test assertions
        assert!(result.is_ok());
    }
}
```

---

## Example: Complete Compilation with Tracing

### Command Line

```bash
# Compile a program with full diagnostic tracing
jsavrs --input example.vn --verbose

# Expected output:
```

```
total of bytes read: 2.4 KB
INFO lexer: Tokenizing input file
     │ file: example.vn
     │ size: 2.4 KB
     │ duration: 245μs
     └─ 156 tokens generated

156 tokens found
INFO parser: Parsing statements
     │ tokens: 156
     │ duration: 820μs
     └─ 34 statements parsed

parsing done
34 statements found
INFO semantic: Type checking
     │ statements: 34
     │ duration: 445μs
     │
     ├─ function 'main' checked
     ├─ function 'calculate' checked
     └─ No type errors found

type checking done
INFO ir_generator: Generating intermediate representation
     │ statements: 34
     │ functions: 2
     │ duration: 1.2ms
     └─ IR generation complete

NIR generation done
```

### Programmatic

```rust
use jsavrs::{
    compile_source,
    tracing::{initialize_tracing, TraceConfig, TraceLevel, TraceFormat},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let config = TraceConfig::builder()
        .enabled(true)
        .level(TraceLevel::Info)
        .format(TraceFormat::ErrorReporterStyle)
        .build()?;
    
    jsavrs::tracing::initialize_with_config(config)?;
    
    // Read source file
    let source = std::fs::read_to_string("example.vn")?;
    
    // Compile with automatic tracing
    match compile_source(&source) {
        Ok(compiled) => {
            println!("Compilation successful!");
            Ok(())
        }
        Err(errors) => {
            eprintln!("Compilation failed with {} errors", errors.len());
            Err("Compilation failed".into())
        }
    }
}
```

---

## Further Reading

- [research.md](./research.md) - Technical implementation details
- [data-model.md](./data-model.md) - Complete data structure documentation
- [Tracing crate documentation](https://docs.rs/tracing/)
- [Console crate documentation](https://docs.rs/console/)

---

## Getting Help

If you encounter issues not covered in this guide:

1. Check the [Troubleshooting](#troubleshooting) section
2. Review trace output for warnings or errors
3. Enable DEBUG level for more detailed diagnostics
4. File an issue on GitHub with trace output attached

---

**Status**: ✅ **COMPLETE** - Ready for developer use
