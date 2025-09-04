# jynx Dual Implementation Experiment

## Overview

This project implements **jynx** (intelligent syntax highlighter) using two parallel approaches to evaluate the **RSB (Rebel String-Biased)** framework against standard Rust patterns.

## Experimental Structure

```
jynx/
├── Cargo.toml              # Dual binary configuration
├── src/
│   ├── bin/
│   │   ├── jynx-std.rs     # Standard Rust binary
│   │   └── jynx-rsb.rs     # RSB implementation binary
│   ├── std/                # Raw Rust modules
│   │   ├── mod.rs
│   │   ├── theme.rs
│   │   ├── highlight.rs
│   │   └── auto_detect.rs
│   └── rsb/                # RSB modules
│       ├── mod.rs
│       ├── theme.rs  
│       ├── highlight.rs
│       └── auto_detect.rs
├── docs/
│   ├── rsb-reference/      # RSB framework documentation
│   ├── THEME_SYSTEM.md     # Theme architecture specification
│   └── STATUS.md           # Implementation requirements
└── themes/
    └── example-theme.yml   # Theme configuration examples
```

## Implementation Approaches

### Standard Rust (`src/std/`)
- **Type System**: Full Rust types with proper error handling
- **Performance**: Direct regex compilation and memory optimization  
- **Architecture**: Structured enums, traits, and type safety
- **Dependencies**: Minimal - serde, regex, clap

### RSB Implementation (`src/rsb/`)
- **String-Biased**: String interfaces throughout (`fn process(input: &str) -> String`)
- **Unix Philosophy**: Stream processing with pipe-like operations
- **BashFX Patterns**: Function ordinality, systematic organization
- **Dependencies**: RSB framework macros and utilities

## Build Commands

```bash
# Standard Rust implementation
cargo build --bin jynx-std

# RSB implementation (requires rsb feature)
cargo build --bin jynx-rsb --features rsb

# Run implementations
./target/debug/jynx-std < input.txt
./target/debug/jynx-rsb < input.txt

# Performance comparison
hyperfine 'jynx-std < test.txt' 'jynx-rsb < test.txt'
```

## Core Features to Implement

### 0. Stream/Pipe Interface (CRITICAL)
**Unix Pipe Compatibility**: Must read from stdin and write to stdout for shell pipelines
```bash
cat file.txt | jynx --theme default --filter code | less
echo "The :keeper: found v1.2.3 at /path/file.rs" | jynx --width 80 --align center
```
**Standard**: BufReader/BufWriter with streaming line processing
**RSB**: Built-in stream processing with pipe! macro support

### 1. Auto-Detection Engine
**Standard**: Regex compilation with typed pattern matching
**RSB**: String-based pattern detection with simple interfaces

### 2. Theme System
**Standard**: Serde-based YAML parsing with structured config types
**RSB**: String-based theme loading with simple key-value access

### 3. Icon Mapping (`:word:` patterns)
**Standard**: HashMap-based lookups with option types
**RSB**: String operations with graceful degradation

### 4. Processing Pipeline (Stream-Based)
**Standard**: stdin → BufReader → line-by-line processing → BufWriter → stdout
**RSB**: stdin | auto_detect | apply_icons | apply_keywords | format_width | stdout

### 5. CLI Interface
**Standard**: Clap-based argument parsing with structured options
**RSB**: String-based argument handling with RSB patterns

## Success Metrics

### Performance Comparison
- **Compile Time**: RSB macros vs raw Rust compilation
- **Runtime Speed**: String abstractions vs direct type operations
- **Memory Usage**: RSB overhead vs manual memory management
- **Binary Size**: Framework dependencies vs minimal standard library

### Developer Experience
- **Lines of Code**: Implementation conciseness comparison
- **Readability**: Code clarity and maintainability
- **Implementation Speed**: Time to working prototype
- **Learning Curve**: Complexity for new contributors

### Feature Completeness
- **Auto-detection patterns**: Paths, URLs, versions, git hashes, etc.
- **Icon mapping system**: `:word:` → emoji/glyph + color
- **Theme inheritance**: Defaults + selective overrides
- **Output formatting**: `--width` and `--align` options

## RSB Learning Resources

The `docs/rsb-reference/` directory contains:
- `REBEL.md` - RSB philosophy and mental models
- `rsb-architecture.md` - Framework patterns and structure  
- `rsb-patterns.md` - Common implementation patterns
- `rsb-quick-reference-v2.md` - API reference and examples

## Lucas Engineering Strategy

**Recommended Approach:**
1. **Read RSB docs** - Understand string-biased philosophy and patterns
2. **Choose implementation** - Start with either std or rsb based on comfort level
3. **Build incrementally** - One feature at a time with working tests
4. **Compare as you go** - Implement same features in both approaches
5. **Measure everything** - Collect data on performance and developer experience

**Decision Framework:**
- If **performance is critical** → Focus on std implementation first
- If **rapid development** is priority → Focus on rsb implementation first  
- If **learning Rust** is goal → Compare both approaches actively

## Expected Outcomes

This experiment will provide concrete data on:
- RSB framework viability for real-world tools
- Performance trade-offs of string-biased approaches
- Developer productivity differences between approaches
- Architecture patterns that work best for syntax highlighting tools

The results will inform both jynx's final implementation and RSB's evolution as a Rust development framework.

---

**Status**: Experimental framework ready - Lucas can now choose implementation strategy and begin development! 🚀