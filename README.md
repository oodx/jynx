# jynx 🎨

**Intelligent syntax highlighter with auto-detection, icon mapping, and theme inheritance**

jynx is a **stream-first** syntax highlighter built in Rust that brings visual intelligence to your terminal. It automatically detects patterns, maps semantic labels to visual icons, and applies rich theming - all while maintaining perfect Unix pipeline compatibility.

## ✨ Key Features

### 🤖 **Auto-Detection Intelligence**
- **Zero-config highlighting** for URLs, version numbers, file paths
- **Visual icons** with Unicode fallback support
- **Semantic enhancement** without manual configuration

### 🎯 **Icon Mapping System**
- **`:word:` patterns** get enhanced with emoji/glyphs + colors
- **Semantic labeling**: `:critical:` → 🔥, `:success:` → ✅, `:keeper:` → 🌑
- **Graceful degradation** - unknown patterns pass through unchanged

### 🎨 **Rich Theme System**
- **YAML-based configuration** with inheritance support
- **Selective overrides** - inherit defaults, customize what matters
- **Multiple filters** - context-aware highlighting (logs, todo, code, docs)
- **90+ semantic colors** including extended debug colors (silly, magic, trace, think)

### 🚰 **Stream-First Design**
- **Perfect Unix citizen** - reads stdin, writes stdout
- **Pipeline compatible**: `cat file.txt | jynx --theme dark | less -R`
- **Real-time processing** - line-by-line with immediate output
- **Fixed-width output** with alignment options

## 🚀 Quick Start

```bash
# Install from release binary
./bin/deploy.sh

# Basic usage - auto-detection
echo "Check https://api.github.com v1.2.3 in /home/user/config.yml" | jynx

# With theme and filter  
cat logs.txt | jynx --theme themes/example-theme.yml --filter logs

# Fixed width output
echo "Deploy :success: complete" | jynx --width 80 --align center

# Real-world pipeline usage
tail -f app.log | jynx --theme dark --filter logs | grep ERROR
```

## 📖 Examples

### Auto-Detection in Action
```bash
$ echo "Deploy v2.1.0 to https://prod.example.com at /etc/config.json" | jynx
Deploy 🏷️ v2.1.0 to 🔗 https://prod.example.com at 📁 /etc/config.json
```

### Icon Mapping System
```bash  
$ echo "Status: :critical: database error, :success: tests passed" | jynx --theme example
Status: 🔥 critical database error, ✅ success tests passed
```

### Theme-Aware Filtering
```bash
$ echo "ERROR: Connection failed, INFO: Retry scheduled" | jynx --theme example --filter logs
ERROR: Connection failed (bold red)
INFO: Retry scheduled (blue)
```

## 🛠️ Development

### Project Structure
```
jynx/
├── bin/              # Support scripts
│   ├── deploy.sh     # Production deployment  
│   └── ux.sh         # Feature demonstration
├── docs/             # Documentation
├── src/              # Rust source code
├── themes/           # Example theme files
└── examples/         # Usage examples
```

### Building
```bash
cargo build --release
./bin/ux.sh  # See all features in action
```

## 🎭 Demo

Run the UX demonstration to see jynx's full capabilities:

```bash
./bin/ux.sh
```

This showcases:
- Auto-detection intelligence  
- Icon mapping system
- Theme filtering
- Width/alignment formatting
- Real-world usage patterns

## 📚 Documentation

- [Theme System](docs/THEME_SYSTEM.md) - Complete theming guide
- [Stream Interface](docs/STREAM_INTERFACE.md) - Unix pipeline requirements  
- [Technical Spec](docs/TECHNICAL_SPEC.md) - Implementation details
- [Status](docs/STATUS.md) - Development progress

## 🤝 Philosophy

**jynx follows the Unix philosophy**: do one thing well. It's a **stream processor** that adds visual intelligence to text, designed to be:

- **Composable** - Works perfectly in pipelines
- **Fast** - Rust performance with streaming I/O  
- **Smart** - Zero-config intelligence with full customization
- **Beautiful** - Makes terminal output visually informative

---

**jynx**: *Where intelligence meets beauty in the terminal* ✨🚰