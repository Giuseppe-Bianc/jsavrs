# Phase 1: Data Model - Tracing System Data Structures

**Feature**: Centralized Tracing System Configuration  
**Date**: 2025-10-11  
**Status**: Complete

## Overview

This document defines the comprehensive data model for the jsavrs tracing system, including all enums, structs, and type definitions that implement the trace infrastructure. Following the project's constitution principle of using enum-based data models to reduce validation errors, all configuration options and trace metadata use strongly-typed enums rather than strings or magic values.

---

## Core Data Structures

### 1. Trace Configuration

#### TraceConfig

The central configuration structure controlling all tracing behavior.

```rust
/// Comprehensive configuration for the tracing subsystem
#[derive(Debug, Clone)]
pub struct TraceConfig {
    /// Is tracing enabled at all?
    pub enabled: bool,
    
    /// Minimum trace level to emit
    pub level: TraceLevel,
    
    /// Output destination configuration
    pub output: TraceOutput,
    
    /// Format for trace messages
    pub format: TraceFormat,
    
    /// Filtering rules for selective tracing
    pub filter: TraceFilter,
    
    /// Timing precision for span measurements
    pub timing_precision: TimingPrecision,
    
    /// Whether to include source locations in traces
    pub include_locations: bool,
    
    /// Whether to include thread IDs (future multi-threading)
    pub include_thread_ids: bool,
    
    /// Maximum span depth to prevent infinite recursion
    pub max_span_depth: usize,
}

impl Default for TraceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            level: TraceLevel::Info,
            output: TraceOutput::Stderr,
            format: TraceFormat::ErrorReporterStyle,
            filter: TraceFilter::AllPhases,
            timing_precision: TimingPrecision::Microseconds,
            include_locations: true,
            include_thread_ids: false,
            max_span_depth: 32,
        }
    }
}
```

**Validation Rules**:
- `max_span_depth` must be >= 1 and <= 256
- If `enabled` is false, all other settings are ignored
- `level` determines minimum severity for emission

**State Transitions**:
```
Created (Default) → Configured → Validated → Applied (Global Subscriber Set)
```

---

### 2. Trace Levels

#### TraceLevel Enum

Graduated severity levels matching standard observability practices.

```rust
/// Trace event severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TraceLevel {
    /// System errors only (compile failures, crashes)
    Error,
    
    /// Warnings (potential issues, deprecations)
    Warn,
    
    /// High-level phase information (default for --verbose)
    Info,
    
    /// Detailed operation traces (function-level)
    Debug,
    
    /// Maximum verbosity (inner loops, all decisions)
    Trace,
}

impl TraceLevel {
    /// Convert to tracing crate's Level type
    pub fn to_tracing_level(self) -> tracing::Level {
        match self {
            Self::Error => tracing::Level::ERROR,
            Self::Warn => tracing::Level::WARN,
            Self::Info => tracing::Level::INFO,
            Self::Debug => tracing::Level::DEBUG,
            Self::Trace => tracing::Level::TRACE,
        }
    }
    
    /// Parse from string (for future CLI enhancement)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "error" => Some(Self::Error),
            "warn" | "warning" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" => Some(Self::Debug),
            "trace" => Some(Self::Trace),
            _ => None,
        }
    }
}

impl std::fmt::Display for TraceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "ERROR"),
            Self::Warn => write!(f, "WARN"),
            Self::Info => write!(f, "INFO"),
            Self::Debug => write!(f, "DEBUG"),
            Self::Trace => write!(f, "TRACE"),
        }
    }
}
```

**Level Semantics**:
- **Error**: Compilation failures, internal errors, panic conditions
- **Warn**: Type promotion warnings, deprecation notices, potential issues
- **Info**: Phase entry/exit, high-level progress (default for `--verbose`)
- **Debug**: Individual function calls, decision points, data flow
- **Trace**: Inner loop iterations, every token/node processed

---

### 3. Output Destinations

#### TraceOutput Enum

Defines where trace data is written.

```rust
/// Destination for trace output
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceOutput {
    /// Standard output (stdout)
    Stdout,
    
    /// Standard error (stderr) - default for diagnostics
    Stderr,
    
    /// Write to specified file path
    File(std::path::PathBuf),
    
    /// Multiple simultaneous outputs
    Multi(Vec<TraceOutput>),
    
    /// Discard all output (for overhead measurement)
    Null,
}

impl TraceOutput {
    /// Create writer for this output destination
    pub fn create_writer(&self) -> Result<Box<dyn std::io::Write + Send>, TracingError> {
        match self {
            Self::Stdout => Ok(Box::new(std::io::stdout())),
            Self::Stderr => Ok(Box::new(std::io::stderr())),
            Self::File(path) => {
                let file = std::fs::File::create(path)
                    .map_err(|e| TracingError::OutputInitFailed {
                        destination: format!("{}", path.display()),
                        source: e,
                    })?;
                Ok(Box::new(file))
            }
            Self::Multi(outputs) => {
                // MultiWriter implementation that broadcasts to all destinations
                let writers = outputs
                    .iter()
                    .map(|out| out.create_writer())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Box::new(MultiWriter::new(writers)))
            }
            Self::Null => Ok(Box::new(std::io::sink())),
        }
    }
    
    /// Failover to stderr if this destination is unavailable
    pub fn with_stderr_failover(self) -> TraceOutput {
        match self {
            Self::Stdout | Self::Stderr => self,
            Self::File(_) => Self::Multi(vec![self, Self::Stderr]),
            Self::Multi(_) => self,
            Self::Null => self,
        }
    }
}
```

**Failover Behavior**:
- File creation failure → automatic stderr redirect + warning
- Multi-writer partial failure → continue with available writers
- Stderr/stdout always available (no failover needed)

---

### 4. Output Formats

#### TraceFormat Enum

Defines the visual presentation style for trace output.

```rust
/// Formatting style for trace output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceFormat {
    /// Match error_reporter.rs visual style (colored, structured)
    ErrorReporterStyle,
    
    /// Compact single-line format
    Compact,
    
    /// Detailed multi-line format with full context
    Detailed,
    
    /// Structured JSON for log aggregation
    Json,
    
    /// Plain text without ANSI colors (for piping)
    Plain,
}

impl TraceFormat {
    /// Should this format include ANSI color codes?
    pub fn uses_colors(self) -> bool {
        matches!(self, Self::ErrorReporterStyle | Self::Detailed)
    }
    
    /// Should this format use multi-line output?
    pub fn is_multiline(self) -> bool {
        matches!(self, Self::ErrorReporterStyle | Self::Detailed)
    }
}
```

**Format Examples**:

```
ErrorReporterStyle:
INFO lexer: Tokenizing input file
     │ file: /path/to/source.vn
     │ size: 1.2 KB
     │ duration: 125μs
     └─ 47 tokens generated

Compact:
INFO lexer file=/path/to/source.vn size=1.2KB duration=125μs tokens=47

Detailed:
2025-10-11T10:30:45.123456Z  INFO lexer: Tokenizing input file
    at src/lexer.rs:145
    in compilation_root
    fields:
        file: /path/to/source.vn
        size: 1.2 KB
        duration: 125μs
        tokens: 47

Json:
{"timestamp":"2025-10-11T10:30:45.123456Z","level":"INFO","target":"jsavrs::lexer","fields":{"message":"Tokenizing input file","file":"/path/to/source.vn","size":"1.2KB","duration_us":125,"tokens":47}}

Plain:
INFO lexer: Tokenizing input file [file=/path/to/source.vn size=1.2KB duration=125μs tokens=47]
```

---

### 5. Filtering Rules

#### TraceFilter Enum

Selective tracing of specific compilation phases or modules.

```rust
/// Filtering rules for selective tracing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceFilter {
    /// Trace all compilation phases
    AllPhases,
    
    /// Trace only specified phases
    Phases(Vec<CompilationPhase>),
    
    /// Trace only specified modules (by target path)
    Modules(Vec<String>),
    
    /// Trace only operations on specified files
    Files(Vec<std::path::PathBuf>),
    
    /// Custom filter predicate (for advanced users)
    Custom(String), // EnvFilter directive format
}

impl TraceFilter {
    /// Convert to tracing_subscriber::EnvFilter directive
    pub fn to_env_filter(&self) -> String {
        match self {
            Self::AllPhases => "jsavrs=trace".to_string(),
            Self::Phases(phases) => {
                phases
                    .iter()
                    .map(|p| p.target_filter())
                    .collect::<Vec<_>>()
                    .join(",")
            }
            Self::Modules(modules) => {
                modules
                    .iter()
                    .map(|m| format!("{}=trace", m))
                    .collect::<Vec<_>>()
                    .join(",")
            }
            Self::Files(_) => {
                // File filtering handled at span creation time
                "jsavrs=trace".to_string()
            }
            Self::Custom(directive) => directive.clone(),
        }
    }
}
```

---

### 6. Compilation Phases

#### CompilationPhase Enum

Strongly-typed representation of major compiler phases.

```rust
/// Major compilation phases for selective tracing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompilationPhase {
    /// Lexical analysis (tokenization)
    Lexer,
    
    /// Syntactic analysis (parsing)
    Parser,
    
    /// Semantic analysis (type checking)
    Semantic,
    
    /// Intermediate representation generation
    IrGeneration,
    
    /// Assembly code generation
    AsmGeneration,
    
    /// Optimization passes
    Optimization,
}

impl CompilationPhase {
    /// Get the target module path for filtering
    pub fn target_filter(self) -> String {
        match self {
            Self::Lexer => "jsavrs::lexer=trace".to_string(),
            Self::Parser => "jsavrs::parser=trace".to_string(),
            Self::Semantic => "jsavrs::semantic=trace".to_string(),
            Self::IrGeneration => "jsavrs::ir=trace".to_string(),
            Self::AsmGeneration => "jsavrs::asm=trace".to_string(),
            Self::Optimization => "jsavrs::ir::optimization=trace".to_string(),
        }
    }
    
    /// Get human-readable phase name
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Lexer => "Lexical Analysis",
            Self::Parser => "Syntactic Analysis",
            Self::Semantic => "Semantic Analysis",
            Self::IrGeneration => "IR Generation",
            Self::AsmGeneration => "Assembly Generation",
            Self::Optimization => "Optimization",
        }
    }
}

impl std::fmt::Display for CompilationPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
```

---

### 7. Timing Precision

#### TimingPrecision Enum

Control the granularity of timing measurements.

```rust
/// Precision for timing measurements in trace spans
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimingPrecision {
    /// Milliseconds (1ms resolution)
    Milliseconds,
    
    /// Microseconds (1μs resolution) - default
    Microseconds,
    
    /// Nanoseconds (1ns resolution) - for microbenchmarks
    Nanoseconds,
}

impl TimingPrecision {
    /// Format a duration with this precision
    pub fn format(self, duration: std::time::Duration) -> String {
        match self {
            Self::Milliseconds => format!("{}ms", duration.as_millis()),
            Self::Microseconds => format!("{}μs", duration.as_micros()),
            Self::Nanoseconds => format!("{}ns", duration.as_nanos()),
        }
    }
    
    /// Get the divisor for converting from nanoseconds
    pub fn divisor(self) -> u128 {
        match self {
            Self::Milliseconds => 1_000_000,
            Self::Microseconds => 1_000,
            Self::Nanoseconds => 1,
        }
    }
}
```

---

### 8. Error Types

#### TracingError Enum

Errors that can occur during tracing system initialization or operation.

```rust
/// Errors that occur in the tracing subsystem
#[derive(Debug, thiserror::Error)]
pub enum TracingError {
    /// Failed to initialize output destination
    #[error("Failed to initialize trace output '{destination}': {source}")]
    OutputInitFailed {
        destination: String,
        #[source]
        source: std::io::Error,
    },
    
    /// Failed to set global subscriber (already set)
    #[error("Global tracing subscriber already initialized")]
    SubscriberAlreadySet,
    
    /// Invalid configuration parameter
    #[error("Invalid tracing configuration: {message}")]
    InvalidConfig {
        message: String,
    },
    
    /// Failed to create custom formatter
    #[error("Failed to create trace formatter: {reason}")]
    FormatterCreationFailed {
        reason: String,
    },
    
    /// Filter parsing error
    #[error("Invalid trace filter directive '{directive}': {source}")]
    InvalidFilter {
        directive: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl TracingError {
    /// Convert to CompileError for integration with existing error handling
    pub fn to_compile_error(self) -> crate::error::compile_error::CompileError {
        crate::error::compile_error::CompileError::IoError(
            std::io::Error::new(std::io::ErrorKind::Other, self.to_string())
        )
    }
}
```

---

## Span Metadata

### SpanMetadata

Structured metadata attached to trace spans.

```rust
/// Metadata associated with a trace span
#[derive(Debug, Clone)]
pub struct SpanMetadata {
    /// Span identifier (unique within trace context)
    pub id: SpanId,
    
    /// Span name (function or operation name)
    pub name: String,
    
    /// Compilation phase this span belongs to
    pub phase: Option<CompilationPhase>,
    
    /// Source file being processed (if applicable)
    pub file: Option<std::path::PathBuf>,
    
    /// Parent span identifier (for hierarchical context)
    pub parent_id: Option<SpanId>,
    
    /// Start timestamp
    pub start_time: std::time::Instant,
    
    /// Structured fields (key-value pairs)
    pub fields: std::collections::HashMap<String, FieldValue>,
}

/// Unique identifier for a span
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanId(u64);

impl SpanId {
    /// Generate a new unique span identifier
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// Values that can be stored in span fields
#[derive(Debug, Clone)]
pub enum FieldValue {
    String(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Duration(std::time::Duration),
}
```

---

## Builder Pattern

### TraceConfigBuilder

Fluent API for constructing `TraceConfig` instances.

```rust
/// Builder for TraceConfig with validation
pub struct TraceConfigBuilder {
    config: TraceConfig,
    errors: Vec<String>,
}

impl TraceConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: TraceConfig::default(),
            errors: Vec::new(),
        }
    }
    
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.config.enabled = enabled;
        self
    }
    
    pub fn level(mut self, level: TraceLevel) -> Self {
        self.config.level = level;
        self
    }
    
    pub fn output(mut self, output: TraceOutput) -> Self {
        self.config.output = output;
        self
    }
    
    pub fn format(mut self, format: TraceFormat) -> Self {
        self.config.format = format;
        self
    }
    
    pub fn filter(mut self, filter: TraceFilter) -> Self {
        self.config.filter = filter;
        self
    }
    
    pub fn max_span_depth(mut self, depth: usize) -> Self {
        if depth == 0 || depth > 256 {
            self.errors.push(format!(
                "max_span_depth must be between 1 and 256, got {}",
                depth
            ));
        } else {
            self.config.max_span_depth = depth;
        }
        self
    }
    
    /// Build the configuration, returning error if validation failed
    pub fn build(self) -> Result<TraceConfig, TracingError> {
        if !self.errors.is_empty() {
            return Err(TracingError::InvalidConfig {
                message: self.errors.join("; "),
            });
        }
        Ok(self.config)
    }
}
```

---

## Relationships

### Entity Relationship Diagram

```
TraceConfig (1) ──┬── (1) TraceLevel
                  ├── (1) TraceOutput
                  ├── (1) TraceFormat
                  ├── (1) TraceFilter
                  └── (1) TimingPrecision

TraceFilter (1) ── (*) CompilationPhase

SpanMetadata (1) ──┬── (1) SpanId
                   ├── (0..1) CompilationPhase
                   ├── (0..1) parent SpanId
                   └── (*) FieldValue

TracingError ──> CompileError (conversion)
```

---

## State Transitions

### Tracing System Lifecycle

```
[Uninitialized] 
    │
    ├─ initialize_tracing(config) with enabled=false
    │  └─> [Disabled] (zero overhead, immediate return)
    │
    └─ initialize_tracing(config) with enabled=true
       ├─ config validation
       │  └─ failure → [Degraded] (warning logged, continues)
       ├─ subscriber creation
       │  └─ failure → [Degraded]
       ├─ set_global_default
       │  └─ failure → [Degraded]
       └─> [Active] (traces emitted according to config)

[Active]
    │
    ├─ compilation completes
    │  └─> [Flushing] → [Shutdown]
    │
    └─ signal received (Ctrl+C)
       └─> [Flushing] (ensure buffered traces written) → [Shutdown]
```

### Span Lifecycle

```
[Created] 
    │ (tracing::info_span!("name"))
    │
[Entered]
    │ (let _guard = span.enter())
    │ (trace events emitted within this context)
    │
[Exited]
    │ (guard dropped, timing recorded)
    │
[Closed]
    │ (span dropped, resources released)
```

---

## Validation Rules Summary

### TraceConfig Validation
- `max_span_depth`: 1 ≤ depth ≤ 256
- If `enabled = false`, all other settings ignored
- `output` must be createable or failover to stderr

### TraceLevel Validation
- Must be one of: Error, Warn, Info, Debug, Trace
- Orderable: Error < Warn < Info < Debug < Trace
- Used for filtering: events below configured level discarded

### TraceFilter Validation
- Phase filter: Must reference valid `CompilationPhase` variants
- Module filter: Must be valid Rust module paths
- File filter: Paths need not exist (may be future files)
- Custom filter: Must parse as valid `EnvFilter` directive

### SpanMetadata Validation
- `name` must be non-empty
- `parent_id` must reference existing span or be None
- `start_time` must be in the past (sanity check)
- `fields` keys must be valid Rust identifiers

---

## Performance Characteristics

### Memory Usage
- **TraceConfig**: ~200 bytes (small, typically stack-allocated)
- **SpanMetadata**: ~150 bytes + field data (heap-allocated)
- **Active span stack**: ~32 spans × 150 bytes = ~5KB per thread
- **Total overhead**: < 50KB for typical compilation

### Timing Overhead
- **Disabled tracing**: 0ns (compile-time eliminated)
- **Span creation**: ~50ns (allocation + metadata)
- **Span enter/exit**: ~20ns (atomic ops)
- **Event emission**: ~100ns (formatting + I/O)
- **Total per phase**: < 1μs (negligible vs. actual work)

---

## Future Extensions

### Potential Enhancements (Not in Scope for Phase 1)

1. **Dynamic Configuration**: Runtime level adjustment via signal or IPC
2. **Distributed Tracing**: OpenTelemetry integration for multi-process builds
3. **Flamegraph Generation**: Automatic performance visualization
4. **Trace Sampling**: Probabilistic emission for production use
5. **Compression**: gzip trace output for large compilations
6. **Rotation**: Automatic log file rotation and archival

---

## Conclusion

This data model provides a comprehensive, type-safe foundation for the jsavrs tracing system. By using enums throughout for configuration and metadata, we eliminate entire classes of validation errors and provide excellent IDE support through exhaustive pattern matching. The design supports both immediate requirements (human-readable diagnostic output) and future enhancements (JSON export, filtering, multi-threading) without requiring breaking changes.

**Status**: ✅ **COMPLETE** - Ready for implementation in Phase 2
