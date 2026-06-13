# buttre Engine Observability

This document describes the observability features built into the buttre-engine, including structured logging, tracing, and debugging capabilities.

## Structured Logging with Tracing

The buttre-engine uses the [`tracing`](https://docs.rs/tracing) framework for structured logging. This provides powerful debugging and monitoring capabilities.

### Quick Start

Initialize tracing in your application:

```rust
use buttre_engine::init_tracing;

fn main() {
    // Initialize tracing subscriber
    init_tracing();
    
    // Your code here
}
```

### Log Levels

Control log verbosity using the `RUST_LOG` environment variable:

| Level | Description | Use Case |
|-------|-------------|----------|
| `error` | Critical errors only | Production |
| `warn` | Warnings and errors | Production |
| `info` | Informational messages | Production/Development |
| `debug` | Debug information | Development |
| `trace` | Detailed execution traces | Deep debugging |

### Examples

#### Show All Logs
```bash
RUST_LOG=trace cargo run --example tracing_demo
```

#### Show Only Engine Logs
```bash
RUST_LOG=buttre_engine=debug cargo run
```

#### Show Specific Stage Logs
```bash
# Only transformation stage
RUST_LOG=buttre_engine::pipeline::stages::stage4_transform=trace cargo run

# Only tone stage
RUST_LOG=buttre_engine::pipeline::stages::stage5_tone=trace cargo run
```

#### Production Settings
```bash
# Recommended for production
RUST_LOG=buttre_engine=info cargo run
```

## Instrumented Components

### Pipeline Executor

The main pipeline executor provides trace-level logging for:
- Input character processing
- Stage transitions (Continue/PassThrough/Output)
- PassThrough confirmations
- Warning when all stages return Continue

**Example trace output:**
```
TRACE process{syllable=thu}: Processing input character: 's'
DEBUG process{syllable=thu}: Stage returned Output with 1 actions
```

### Stage 3: Validation

Logs validation decisions:
- Non-alphabetic character warnings
- Strict mode validation results
- Valid/invalid syllable detection

**Example debug output:**
```
DEBUG process{stage="validation" strict=true}: Invalid syllable 'xyz' in strict mode, passing through
TRACE process{stage="validation" strict=true}: Valid syllable 'thu' in strict mode
```

### Stage 4: Transform

Tracks character transformations:
- Transformation detection and application
- Context rule blocking
- Sequence length and results

**Example debug output:**
```
DEBUG process{stage="transform"}: Applying transformation: 2 chars → 'â' (sequence_len=2)
TRACE process{stage="transform"}: No transformation found, appending 'h' as-is
DEBUG process{stage="transform"}: Context rule blocked transformation, appending 'w' as-is
```

### Stage 5: Tone

Logs tone mark application:
- Tone key detection
- Vowel position calculation
- Tone application results

**Example debug output:**
```
DEBUG process{stage="tone"}: Detected tone key 's' → Acute
DEBUG process{stage="tone"}: Applying tone to vowel 'u' at position 2 → 'ú'
TRACE process{stage="tone"}: Not a tone key, continuing
```

## Span Hierarchy

Tracing uses spans to create execution context:

```
process{syllable=thu}
├── process{stage="validation" input='s' syllable=thu strict=false}
├── process{stage="transform" input='s' syllable=thu}
└── process{stage="tone" input='s' syllable=thus}
```

Each span includes:
- `syllable`: Current syllable buffer state
- `stage`: Pipeline stage name (for stage spans)
- `input`: Input character being processed
- Stage-specific fields (e.g., `strict` for validation)

## Debugging Scenarios

### Scenario 1: Why Didn't My Transformation Apply?

Enable trace logging for the transform stage:

```bash
RUST_LOG=buttre_engine::pipeline::stages::stage4_transform=trace cargo run
```

Look for:
- `"No transformation found"` - The sequence doesn't match any rule
- `"Context rule blocked transformation"` - A context rule prevented it
- `"Applying transformation"` - It worked!

### Scenario 2: Why Is Tone on Wrong Vowel?

Enable debug logging for the tone stage:

```bash
RUST_LOG=buttre_engine::pipeline::stages::stage5_tone=debug cargo run
```

Look for:
- `"Applying tone to vowel 'X' at position Y"` - Shows which vowel was selected
- `"No vowel found in syllable buffer"` - No vowel to apply tone to

### Scenario 3: Why Is Input Being Passed Through?

Enable debug logging for all stages:

```bash
RUST_LOG=buttre_engine=debug cargo run
```

Look for:
- `"Stage returned PassThrough"` - A stage rejected the input
- `"Invalid syllable ... in strict mode"` - Validation rejected it
- `"Non-alphabetic, non-numeric character"` - Character type issue

### Scenario 4: Performance Issues

Enable trace logging to see execution flow:

```bash
RUST_LOG=buttre_engine::pipeline::executor=trace cargo run
```

Count the number of stages executed per character to identify bottlenecks.

## Integration with Applications

### Desktop Application (buttre-platform)

Add tracing initialization in the platform layer:

```rust
// In buttre-platform startup
fn main() {
    // Initialize tracing early
    buttre_engine::init_tracing();
    
    // Platform initialization
    // ...
}
```

### Testing

Tracing works automatically in tests. To see logs during tests:

```bash
RUST_LOG=debug cargo test -- --nocapture
```

## Custom Tracing Subscribers

For advanced use cases, you can configure your own tracing subscriber instead of using `init_tracing()`:

```rust
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    // Custom configuration
    tracing_subscriber::registry()
        .with(fmt::layer()
            .with_target(true)
            .with_level(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true))
        .with(EnvFilter::from_default_env())
        .init();
    
    // Your code here
}
```

## Performance Considerations

- **Disabled at compile time**: With `--release` builds, most tracing overhead is eliminated if not used
- **Dynamic filtering**: Use `RUST_LOG` to filter logs at runtime without recompiling
- **Minimal overhead**: Trace/debug logs have negligible performance impact when filtered out

## Future Enhancements

Planned observability features:

- **Metrics Collection**: Track transformation counts, error rates, latency percentiles
- **Performance Monitoring**: Real-time performance dashboards
- **Error Tracking**: Structured error reporting with context
- **OpenTelemetry Integration**: Export traces to monitoring systems

## References

- [Tracing Documentation](https://docs.rs/tracing)
- [Tracing Subscriber Guide](https://docs.rs/tracing-subscriber)
- [RUST_LOG Syntax](https://docs.rs/env_logger/#enabling-logging)
