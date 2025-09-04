# Jynx - High-Performance Syntax Highlighter 

## Product Vision
Jynx is a **pipe-able syntax highlighter** designed to replace bash-based colorization in knowledge management systems. It provides **blazing-fast** keyword highlighting using YAML configuration files, with a focus on **semantic coloring** for structured documentation.

## Problem Statement
Current bash-based colorization in KB system:
- **Performance bottleneck**: 7x slower than baseline (multiple sed passes)
- **Complex maintenance**: Inline regex logic in main application
- **Limited scalability**: Performance degrades with content size and keyword count
- **No reusability**: Logic locked into specific application

## Solution: Jynx Highlighter
**Single-pass Rust implementation** with YAML-driven configuration:
- ⚡ **Sub-millisecond processing** for typical KB content 
- 🎨 **Rich semantic highlighting** with 256-color support
- 📝 **YAML-based rules** for easy theme management
- 🔧 **Pipe-friendly CLI** for universal integration

---

## Core Requirements

### 1. **CLI Interface**
```bash
# Pipe-based usage (primary)
echo "content" | jynx --config theme.yml
cat file.txt | jynx -c theme.yml --filter todo

# File-based usage  
jynx --config theme.yml --input file.txt
jynx -c theme.yml -i file.txt --output colored.txt
```

### 2. **Configuration Format (YAML)**
```yaml
# theme.yml - KB-compatible format
filters:
  todo:
    colorize:
      red2: ["priority", "urgent", "critical", "blocked"]
      green2: ["done", "completed", "finished", "resolved"]  
      yellow2: ["status", "assignee", "deadline", "task"]
      
  troubleshoot:
    colorize:
      red2: ["symptom", "error", "failure", "critical"]
      orange: ["cause", "reason", "warning"]
      green2: ["solution", "fix", "resolution", "verified"]
      blue2: ["confirmed", "tested", "validated"]
```

### 3. **Pattern Matching**
Support **two keyword formats**:
- `keyword:` - Colon-terminated (e.g., `priority: high`)
- `[KEYWORD]` - Bracket-wrapped (e.g., `[URGENT]`, `[BLOCKED]`)

**Case-insensitive** matching for both patterns.

### 4. **Performance Targets**
- **< 1ms**: Process 1000 lines of typical KB content
- **< 10ms**: Process 10K lines with high keyword density
- **Memory efficient**: Streaming processing, minimal allocations
- **Linear scaling**: O(n) with content size, O(k) with keyword count

---

## Integration Requirements

### 1. **KB System Integration**
Replace existing `colorize_keywords_multi()` bash function:

**Before** (bash - slow):
```bash
colorize_keywords_multi "red2:priority,urgent" "green2:done,completed"
```

**After** (jynx - fast):
```bash
jynx --config ~/.local/etc/fx/kb/theme.yml --filter todo
```

### 2. **Backward Compatibility**
- **Theme files**: Must read existing KB theme.yml format
- **Color output**: Compatible with existing boxy integration
- **Filter contexts**: Support KB's 17 filter categories
- **Fallback**: Graceful degradation if jynx unavailable

### 3. **Theme System Bridge**
```bash
# KB integration point in _apply_colorization()
_apply_colorization() {
    local content="$1"
    local filter_type="$2"
    
    # Try jynx first (fast path)
    if command -v jynx >/dev/null 2>&1; then
        echo "$content" | jynx --config "$KB_THEME_FILE" --filter "$filter_type"
        return 0
    fi
    
    # Fallback to inlined bash colorization
    colorize_keywords_multi "$@"
}
```

---

## Technical Specifications

### 1. **Core Features**
- **YAML configuration parsing** (serde_yaml)
- **Regex engine** for pattern matching (regex crate)
- **ANSI color output** (256-color support)
- **Streaming I/O** for pipe compatibility
- **Error handling** with graceful fallback

### 2. **Configuration Schema**
```rust
#[derive(Deserialize)]
struct ThemeConfig {
    filters: HashMap<String, FilterConfig>,
}

#[derive(Deserialize)]  
struct FilterConfig {
    colorize: Option<HashMap<String, Vec<String>>>,
}
```

### 3. **Processing Pipeline**
1. **Parse YAML** configuration
2. **Compile regex patterns** for specified filter
3. **Stream process** input line-by-line
4. **Apply colorization** using single-pass regex matching
5. **Output** ANSI-colored text

### 4. **Optimization Strategy**
- **Pre-compile regex patterns** at startup
- **Single-pass processing** (no multiple sed calls)  
- **Efficient color code lookup** (HashMap)
- **Minimal string allocations** (streaming)

---

## Command Line Interface

### **Required Flags**
```bash
-c, --config <FILE>     YAML theme configuration file
-f, --filter <NAME>     Filter category (todo, troubleshoot, etc.)
-i, --input <FILE>      Input file (default: stdin)
-o, --output <FILE>     Output file (default: stdout)
```

### **Optional Flags**
```bash
-h, --help              Show help message
-V, --version           Show version information
--no-color              Disable colorization (passthrough mode)
--list-filters          Show available filters in config
--validate              Validate config file syntax
```

### **Usage Examples**
```bash
# Basic usage
echo "task: Fix bug" | jynx -c kb-theme.yml -f todo

# File processing  
jynx -c theme.yml -f troubleshoot -i input.txt -o colored.txt

# Validation
jynx --validate -c theme.yml

# Integration testing
jynx -c theme.yml -f todo --no-color  # Should passthrough uncolored
```

---

## Performance Requirements

### **Benchmarks vs Current System**
| Content Size | Current (bash) | Target (jynx) | Improvement |
|--------------|----------------|---------------|-------------|
| 100 lines    | ~100ms        | < 1ms         | 100x faster |
| 1K lines     | ~750ms        | < 5ms         | 150x faster |
| 10K lines    | ~7s           | < 50ms        | 140x faster |

### **Memory Usage**
- **Streaming processing**: Constant memory usage regardless of input size
- **Configuration caching**: Load YAML once, reuse for multiple invocations
- **Regex compilation**: Pre-compile patterns, cache during execution

---

## Integration Milestones

### **Phase 1: Core Implementation**
- ✅ Basic CLI with YAML config parsing
- ✅ Single filter colorization  
- ✅ Both keyword patterns (`keyword:` and `[KEYWORD]`)
- ✅ Performance benchmarking vs bash implementation

### **Phase 2: KB Integration** 
- ✅ Theme file compatibility with existing KB format
- ✅ All 17 filter categories supported
- ✅ Fallback integration in kb.sh
- ✅ Performance validation in production

### **Phase 3: Enhancement**
- ✅ Advanced pattern matching options
- ✅ Color scheme validation
- ✅ Configuration hot-reloading
- ✅ Detailed performance metrics

---

## Success Criteria

### **Performance** 
- [ ] **10x faster** than bash implementation minimum
- [ ] **< 10ms** processing time for typical KB content
- [ ] **Linear scaling** with content size

### **Functionality**
- [ ] **100% compatibility** with existing KB theme files
- [ ] **Zero regression** in colorization quality
- [ ] **Graceful fallback** when jynx unavailable

### **Integration**
- [ ] **Drop-in replacement** for bash colorization
- [ ] **No breaking changes** to KB user experience  
- [ ] **Seamless deployment** with existing KB workflow

### **Quality**
- [ ] **Comprehensive test suite** covering all filter types
- [ ] **Error handling** for malformed configs
- [ ] **Memory safety** and stability under load

---

## Future Enhancements

### **Advanced Features**
- **Custom regex patterns** beyond keyword: and [keyword] 
- **Multi-line highlighting** for complex structures
- **Performance profiling** built-in diagnostics
- **Live config reloading** for theme development

### **Ecosystem Integration**
- **Boxy integration**: Native keyword highlighting in boxy v0.6
- **Other tools**: Generic syntax highlighter for structured text
- **Theme sharing**: Community theme repository

---

*This specification defines jynx as the high-performance foundation for semantic highlighting in knowledge management systems, with the KB system as the primary integration target.*