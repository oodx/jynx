# jynx Stream/Pipe Interface Design

## Unix Pipeline Requirements

jynx must be **stream-first** for seamless shell integration:

```bash
# Core streaming patterns
cat access.log | jynx --filter logs | grep ERROR
echo "Deploy v1.2.3 to /prod/server" | jynx --theme deploy --width 100
curl -s api.com/data | jynx --filter json | less -R

# Pipeline composition  
tail -f app.log | jynx --theme monitoring | tee processed.log | notify-send
find . -name "*.rs" -exec cat {} \; | jynx --filter code --theme rust-dark
```

## Interface Specification

### Input Handling
- **stdin**: Primary input source (required)
- **File args**: Optional file inputs (`jynx file1.txt file2.txt`)
- **No input**: Read from stdin, timeout after 100ms for interactive detection

### Output Requirements
- **stdout**: Processed/highlighted content (pipeable)
- **stderr**: Status messages, errors, debug info (doesn't pollute pipe)
- **Exit codes**: 0=success, 1=error, 2=usage error

### Performance Constraints
- **Streaming**: Process line-by-line, don't buffer entire input
- **Memory**: Constant memory usage regardless of input size
- **Latency**: First line out within 10ms of first line in

## Implementation Approaches

### Standard Rust Stream Processing
```rust
use std::io::{BufRead, BufReader, BufWriter, Write};

fn process_stream() -> Result<(), Error> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    
    let reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    
    for line in reader.lines() {
        let line = line?;
        let highlighted = highlight_line(&line)?;
        writeln!(writer, "{}", highlighted)?;
        writer.flush()?; // Immediate output for pipes
    }
    Ok(())
}
```

### RSB Stream Processing
```rust
use rsb::prelude::*;

fn main() {
    // RSB streaming with pipe! macro
    stdin!()
        .lines()
        .auto_detect_patterns()
        .apply_icon_mappings()
        .apply_keyword_styles()  
        .format_width(80)
        .to_stdout();
}
```

## Shell Integration Examples

### Log Processing
```bash
# Real-time log monitoring with highlighting
tail -f /var/log/nginx/access.log | \
    jynx --filter logs --theme dark | \
    grep --color=never ERROR    # jynx handles coloring
```

### Development Workflow  
```bash
# Code review with syntax highlighting
git show HEAD | jynx --filter code --theme github-dark | less -R

# Search with context highlighting
rg "TODO|FIXME" --type rust | jynx --filter code --highlight-matches
```

### Data Processing
```bash
# CSV processing with semantic highlighting
curl -s api.com/users.csv | \
    jynx --filter csv --theme data | \
    column -t -s ','
```

## Stream Performance Requirements

### Buffering Strategy
- **Line buffering**: Process and output each line immediately  
- **No look-ahead**: Can't depend on seeing future lines
- **Memory bounds**: Max 1MB working memory regardless of input size

### Latency Requirements
- **Interactive**: < 10ms first line latency
- **Throughput**: > 10K lines/sec for large files
- **Memory**: Constant memory usage (no input size scaling)

### Error Handling
- **Partial failure**: Continue processing on recoverable errors
- **Graceful degradation**: Fall back to pass-through if theme fails
- **Signal handling**: Clean shutdown on SIGINT/SIGTERM

## Lucas Implementation Notes

**Critical Design Decision**: jynx is fundamentally a **stream processor**, not a file processor.

**Standard Rust Approach:**
- Use `BufReader::lines()` for line-by-line processing
- Flush output immediately for pipe compatibility  
- Handle broken pipe errors gracefully (SIGPIPE)

**RSB Approach:**
- Leverage RSB's built-in streaming patterns
- Use `pipe!()` macro for Unix-like processing chains
- Stream operations should compose naturally

**Testing Strategy:**
```bash
# Stream performance test
yes "test line with :critical: status v1.2.3" | head -10000 | time jynx-std
yes "test line with :critical: status v1.2.3" | head -10000 | time jynx-rsb

# Pipeline compatibility test  
echo "data" | jynx | cat | wc -l  # Should be 1
```

This stream-first design makes jynx a **composable Unix tool**, not just a syntax highlighter! 🚰