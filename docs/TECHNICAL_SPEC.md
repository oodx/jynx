# Jynx Technical Specification

## Architecture Overview

```
┌─────────────┐    ┌──────────────┐    ┌─────────────────┐    ┌────────────┐
│   Input     │───▶│ YAML Config  │───▶│ Regex Compiler  │───▶│  Colorizer │
│  (stdin)    │    │   Parser     │    │   & Matcher     │    │  Engine    │
└─────────────┘    └──────────────┘    └─────────────────┘    └────────────┘
                                                                      │
┌─────────────┐    ┌──────────────┐    ┌─────────────────┐           │
│   Output    │◀───│ ANSI Color   │◀───│ Pattern Match   │◀──────────┘
│  (stdout)   │    │  Formatter   │    │   Processor     │
└─────────────┘    └──────────────┘    └─────────────────┘
```

## Core Components

### 1. **Configuration Engine**
**Purpose**: Parse KB-compatible YAML theme files

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct JynxConfig {
    pub filters: HashMap<String, FilterConfig>,
}

#[derive(Debug, Deserialize)]
pub struct FilterConfig {
    pub colorize: Option<HashMap<String, Vec<String>>>,
    // Future: pattern, case_sensitive, priority
}

impl JynxConfig {
    pub fn load_from_file(path: &Path) -> Result<Self, JynxError> {
        let content = std::fs::read_to_string(path)?;
        let config: JynxConfig = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    pub fn get_filter(&self, filter_name: &str) -> Option<&FilterConfig> {
        self.filters.get(filter_name)
    }
}
```

### 2. **Pattern Matcher**
**Purpose**: High-performance regex-based keyword highlighting

```rust
use regex::Regex;

pub struct PatternMatcher {
    patterns: Vec<CompiledPattern>,
}

#[derive(Debug)]
struct CompiledPattern {
    regex: Regex,
    color_code: u8,
    pattern_type: PatternType,
}

#[derive(Debug)]
enum PatternType {
    ColonTerminated,  // "keyword:"
    BracketWrapped,   // "[KEYWORD]"
}

impl PatternMatcher {
    pub fn from_config(filter_config: &FilterConfig) -> Result<Self, JynxError> {
        let mut patterns = Vec::new();
        
        if let Some(colorize) = &filter_config.colorize {
            for (color_name, keywords) in colorize {
                let color_code = parse_color_code(color_name)?;
                
                for keyword in keywords {
                    // Pattern 1: keyword: format (case insensitive)
                    let colon_pattern = format!(r"(?i)\b({}):(?=\s|$)", regex::escape(keyword));
                    patterns.push(CompiledPattern {
                        regex: Regex::new(&colon_pattern)?,
                        color_code,
                        pattern_type: PatternType::ColonTerminated,
                    });
                    
                    // Pattern 2: [keyword] format (case insensitive)  
                    let bracket_pattern = format!(r"(?i)\[({})\\]", regex::escape(keyword));
                    patterns.push(CompiledPattern {
                        regex: Regex::new(&bracket_pattern)?,
                        color_code,
                        pattern_type: PatternType::BracketWrapped,
                    });
                }
            }
        }
        
        // Sort patterns by keyword length (longest first) to prevent partial matches
        patterns.sort_by(|a, b| {
            b.regex.as_str().len().cmp(&a.regex.as_str().len())
        });
        
        Ok(PatternMatcher { patterns })
    }
    
    pub fn colorize_line(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Single pass through all patterns (critical for performance)
        for pattern in &self.patterns {
            result = pattern.regex.replace_all(&result, |caps: &regex::Captures| {
                let matched = &caps[1];
                match pattern.pattern_type {
                    PatternType::ColonTerminated => {
                        format!("\x1b[38;5;{}m{}\x1b[0m:", pattern.color_code, matched)
                    }
                    PatternType::BracketWrapped => {
                        format!("[\x1b[38;5;{}m{}\x1b[0m]", pattern.color_code, matched)  
                    }
                }
            }).to_string();
        }
        
        result
    }
}
```

### 3. **Color Code Engine**
**Purpose**: Map KB color names to ANSI 256-color codes

```rust
use std::collections::HashMap;
use once_cell::sync::Lazy;

// Pre-computed color mapping for maximum performance
static COLOR_MAP: Lazy<HashMap<&'static str, u8>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Standard colors
    map.insert("red", 9);
    map.insert("green", 10); 
    map.insert("yellow", 11);
    map.insert("blue", 12);
    map.insert("magenta", 13);
    map.insert("cyan", 14);
    map.insert("white", 15);
    map.insert("gray", 244);
    map.insert("purple", 13);
    
    // Extended 256-color palette
    map.insert("red2", 196);
    map.insert("green2", 46);
    map.insert("yellow2", 226);
    map.insert("blue2", 81);
    map.insert("cyan2", 51);
    map.insert("purple2", 213);
    map.insert("orange", 208);
    map.insert("deep", 22);
    map.insert("deep_green", 28);
    map.insert("white2", 255);
    map.insert("grey", 244);
    map.insert("grey2", 247);
    map.insert("grey3", 250);
    
    map
});

pub fn parse_color_code(color_name: &str) -> Result<u8, JynxError> {
    COLOR_MAP.get(color_name)
        .copied()
        .ok_or_else(|| JynxError::InvalidColor(color_name.to_string()))
}
```

### 4. **Streaming Processor**
**Purpose**: Memory-efficient line-by-line processing

```rust
use std::io::{BufRead, BufReader, Write};

pub struct StreamProcessor {
    matcher: PatternMatcher,
    no_color: bool,
}

impl StreamProcessor {
    pub fn new(config: &FilterConfig, no_color: bool) -> Result<Self, JynxError> {
        let matcher = PatternMatcher::from_config(config)?;
        Ok(StreamProcessor { matcher, no_color })
    }
    
    pub fn process_stream<R: BufRead, W: Write>(
        &self, 
        reader: R, 
        mut writer: W
    ) -> Result<(), JynxError> {
        for line_result in reader.lines() {
            let line = line_result?;
            
            let output = if self.no_color {
                line  // Passthrough mode
            } else {
                self.matcher.colorize_line(&line)
            };
            
            writeln!(writer, "{}", output)?;
        }
        
        Ok(())
    }
}
```

## CLI Application Structure

### **Main Entry Point**
```rust
use clap::{Arg, Command};
use std::io::{self, BufReader};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("jynx")
        .version(env!("CARGO_PKG_VERSION"))
        .about("High-performance syntax highlighter for structured text")
        .arg(Arg::new("config")
            .short('c')
            .long("config") 
            .value_name("FILE")
            .help("YAML theme configuration file")
            .required(true))
        .arg(Arg::new("filter")
            .short('f')
            .long("filter")
            .value_name("NAME")
            .help("Filter category (todo, troubleshoot, etc.)")
            .required(true))
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Input file (default: stdin)"))
        .arg(Arg::new("output")
            .short('o') 
            .long("output")
            .value_name("FILE")
            .help("Output file (default: stdout)"))
        .arg(Arg::new("no-color")
            .long("no-color")
            .help("Disable colorization")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("validate")
            .long("validate")
            .help("Validate config file syntax")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("list-filters")
            .long("list-filters")
            .help("Show available filters in config")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    let config = JynxConfig::load_from_file(Path::new(config_path))?;
    
    if matches.get_flag("validate") {
        println!("✅ Configuration valid: {}", config_path);
        return Ok(());
    }
    
    if matches.get_flag("list-filters") {
        for filter_name in config.filters.keys() {
            println!("{}", filter_name);
        }
        return Ok(());
    }
    
    let filter_name = matches.get_one::<String>("filter").unwrap();
    let filter_config = config.get_filter(filter_name)
        .ok_or_else(|| format!("Filter '{}' not found in config", filter_name))?;
    
    let no_color = matches.get_flag("no-color");
    let processor = StreamProcessor::new(filter_config, no_color)?;
    
    // Input handling
    let reader: Box<dyn BufRead> = match matches.get_one::<String>("input") {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };
    
    // Output handling
    let writer: Box<dyn Write> = match matches.get_one::<String>("output") {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };
    
    processor.process_stream(reader, writer)?;
    
    Ok(())
}
```

## Performance Optimizations

### 1. **Regex Compilation Strategy**
```rust
// Compile once, use many times
pub struct OptimizedMatcher {
    // Pre-compiled regex set for maximum performance
    regex_set: RegexSet,
    color_mapping: Vec<u8>,
    replacement_templates: Vec<String>,
}

impl OptimizedMatcher {
    pub fn new(patterns: Vec<(String, u8)>) -> Result<Self, JynxError> {
        let (regex_patterns, color_codes): (Vec<_>, Vec<_>) = patterns.into_iter().unzip();
        
        let regex_set = RegexSet::new(&regex_patterns)?;
        
        Ok(OptimizedMatcher {
            regex_set,
            color_mapping: color_codes,
            replacement_templates: create_templates(&regex_patterns),
        })
    }
    
    // Single pass matching using RegexSet for optimal performance
    pub fn highlight(&self, text: &str) -> String {
        let matches = self.regex_set.matches(text);
        
        if matches.iter().count() == 0 {
            return text.to_string();  // Fast path for no matches
        }
        
        // Apply colorization based on matched patterns
        self.apply_colorization(text, matches)
    }
}
```

### 2. **Memory Management**
```rust
// Minimize allocations using string slices and in-place modifications
pub struct MemoryEfficientProcessor {
    buffer: String,
    output_buffer: String,
}

impl MemoryEfficientProcessor {
    pub fn process_line(&mut self, line: &str) -> &str {
        self.buffer.clear();
        self.buffer.push_str(line);
        
        self.output_buffer.clear();
        self.highlight_into(&self.buffer, &mut self.output_buffer);
        
        &self.output_buffer
    }
    
    fn highlight_into(&self, input: &str, output: &mut String) {
        // In-place highlighting to minimize string allocations
    }
}
```

### 3. **Benchmark Framework**
```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn benchmark_colorization(c: &mut Criterion) {
        let config = create_test_config();
        let matcher = PatternMatcher::from_config(&config.filters["todo"]).unwrap();
        
        let test_content = include_str!("../test_data/kb_content_1000_lines.txt");
        
        c.bench_function("colorize_1000_lines", |b| {
            b.iter(|| {
                for line in test_content.lines() {
                    black_box(matcher.colorize_line(black_box(line)));
                }
            });
        });
    }
    
    criterion_group!(benches, benchmark_colorization);
    criterion_main!(benches);
}
```

## Error Handling

### **Error Types**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JynxError {
    #[error("Invalid color name: {0}")]
    InvalidColor(String),
    
    #[error("Filter '{0}' not found in configuration")]
    FilterNotFound(String),
    
    #[error("Invalid regex pattern: {0}")]
    RegexError(#[from] regex::Error),
    
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration validation failed: {0}")]
    ConfigValidation(String),
}

impl JynxError {
    pub fn exit_code(&self) -> i32 {
        match self {
            JynxError::InvalidColor(_) => 2,
            JynxError::FilterNotFound(_) => 3,
            JynxError::RegexError(_) => 4,
            JynxError::YamlError(_) => 5,
            JynxError::IoError(_) => 6,
            JynxError::ConfigValidation(_) => 7,
        }
    }
}
```

## Testing Strategy

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keyword_colon_pattern() {
        let config = create_todo_config();
        let matcher = PatternMatcher::from_config(&config).unwrap();
        
        let input = "task: Fix authentication bug";
        let output = matcher.colorize_line(input);
        
        assert!(output.contains("\x1b[38;5;226m"));  // Yellow color code
        assert!(output.contains("task"));
    }
    
    #[test]
    fn test_bracket_pattern() {
        let config = create_todo_config();
        let matcher = PatternMatcher::from_config(&config).unwrap();
        
        let input = "[URGENT] needs immediate attention";
        let output = matcher.colorize_line(input);
        
        assert!(output.contains("\x1b[38;5;196m"));  // Red2 color code
        assert!(output.contains("URGENT"));
    }
    
    #[test]
    fn test_case_insensitive_matching() {
        let config = create_todo_config();
        let matcher = PatternMatcher::from_config(&config).unwrap();
        
        let inputs = ["priority: high", "Priority: HIGH", "PRIORITY: medium"];
        
        for input in &inputs {
            let output = matcher.colorize_line(input);
            assert!(output.contains("\x1b[38;5;196m"), "Failed for: {}", input);
        }
    }
}
```

### **Integration Tests**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::process::Command;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_kb_theme_compatibility() {
        // Test with actual KB theme.yml file
        let theme_content = include_str!("../test_data/kb_theme.yml");
        let temp_theme = create_temp_file(theme_content);
        
        let test_input = "task: Fix bug\npriority: urgent\nstatus: in-progress";
        
        let output = Command::new("./target/release/jynx")
            .args(&["-c", temp_theme.path().to_str().unwrap(), "-f", "todo"])
            .input(test_input)
            .output()
            .expect("Failed to execute jynx");
        
        assert!(output.status.success());
        
        let result = String::from_utf8(output.stdout).unwrap();
        assert!(result.contains("\x1b[38;5;226m"));  // task: colored
        assert!(result.contains("\x1b[38;5;196m"));  // priority: colored
    }
}
```

## Deployment Integration

### **KB System Integration Point**
```bash
# Update _apply_colorization() in kb.sh
_apply_colorization() {
    local content="$1"
    local filter_type="$2"
    
    # Performance-first approach: try jynx
    if command -v jynx >/dev/null 2>&1; then
        echo "$content" | jynx --config "$KB_THEME_FILE" --filter "$filter_type" 2>/dev/null && return 0
    fi
    
    # Fallback to bash implementation
    colorize_keywords_multi "$@"
}
```

### **Performance Validation Script**
```bash
#!/bin/bash
# benchmark_jynx_vs_bash.sh

echo "=== Jynx vs Bash Performance Comparison ==="

# Generate test content
generate_test_content() {
    for i in {1..1000}; do
        echo "task: Fix issue $i"
        echo "priority: high status: in-progress"
        echo "[URGENT] deadline: 2025-02-15"
        echo "done: partial implementation"
    done
}

test_content=$(generate_test_content)

echo "Testing 1000 lines with multiple keywords..."

echo -n "Bash implementation: "
time echo "$test_content" | colorize_keywords_multi "red2:priority,urgent" "green2:done" "yellow2:task,status" > /dev/null

echo -n "Jynx implementation: "  
time echo "$test_content" | jynx -c theme.yml -f todo > /dev/null

echo "=== Performance improvement should be 100x+ ==="
```

This technical specification provides the complete architecture for implementing jynx as a high-performance replacement for the KB system's bash-based colorization, with comprehensive performance optimization and integration strategies.