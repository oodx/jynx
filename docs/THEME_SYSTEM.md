# Jynx Theme System Design

## Core Concepts

### **Inheritance Architecture**
- **Defaults + Overrides**: Start with global defaults, selectively override
- **Icon Mappings**: `:word:` patterns get enhanced with icons + colors
- **Layered Processing**: Keywords → styles, icons → enhanced rendering
- **Graceful Degradation**: Unknown mappings pass through unchanged

### **Theme Structure**
```yaml
# Global defaults (system-wide)
defaults:
  # Auto-detection patterns (zero-config intelligence)
  auto_detection:
    paths:
      pattern: "[~/]?[\\w\\-\\.\/]+\\.[a-z]{2,4}|\/[\\w\\-\\.\/_]+"
      color: "azure"
      underline: true
    versions:
      pattern: "\\d+\\.\\d+\\.\\d+(-\\w+)?"
      color: "emerald"
      bold: true
    numbers:
      pattern: "\\b\\d{1,3}(,\\d{3})*(\\.)\\d+)?"
      color: "cyan"
    urls:
      pattern: "https?://[\\w\\-\\.]+(:\\d+)?(\\/[^\\s]*)?"
      color: "royal"
      underline: true
    tags:
      pattern: "#\\w+"
      color: "violet"
    git_hashes:
      pattern: "\\b[a-f0-9]{7,40}\\b"
      color: "amber"
      italic: true
  
  filters:
    todo:
      icon_mappings:
        critical: { icon: "🔥", color: "red" }
        success: { icon: "✅", color: "green" }
      styles:
        high_priority:
          keywords: ["URGENT", "CRITICAL"]
          color: "orange"
          
# User theme (selective overrides)
theme:
  auto_detection:
    paths:
      color: "your_choice"     # Override default azure
    versions: none             # Disable version highlighting
    custom_emails:             # Add new auto-detection
      pattern: "\\b[\\w\\.-]+@[\\w\\.-]+\\.\\w+\\b"
      color: "purple"
      underline: true
      
  filters:
    todo:
      icon_mappings:
        critical: none                    # Disable default
        keeper: { icon: "🌑", color: "blue" }  # Add custom
      styles:
        high_priority: none              # Disable default style
```

## Theme Storage & Management

### **RSB Directory Structure**
```
~/.local/etc/rsb/jynx/
├── theme.yml              # Active theme (user editable)
├── theme-default.yml      # Protected default theme  
├── compiled/               # Compiled theme cache
│   ├── theme.bin          # Binary compiled theme
│   └── .theme_checksum    # File modification detection
└── custom/                # User custom themes
    ├── dark.yml
    ├── minimal.yml
    └── high-contrast.yml
```

### **Theme Compilation Strategy**

**Problem**: Loading YAML + compiling regexes on every run = slow
**Solution**: Pre-compile themes to optimized binary format

## Compilation Pipeline

### **1. Theme Compilation Trigger**
```rust
// Check if theme needs recompilation
pub fn needs_recompilation(theme_path: &Path) -> bool {
    let compiled_path = get_compiled_theme_path(theme_path);
    let checksum_path = get_checksum_path(theme_path);
    
    // No compiled version exists
    if !compiled_path.exists() {
        return true;
    }
    
    // Source is newer than compiled
    if is_source_newer(theme_path, &compiled_path) {
        return true;
    }
    
    // Checksum mismatch (file was edited)
    if !checksum_matches(theme_path, &checksum_path) {
        return true;
    }
    
    false
}
```

### **2. Smart Loading Strategy**
```rust
pub enum ThemeLoader {
    Compiled(CompiledTheme),    // Fast path - pre-compiled
    Runtime(RuntimeTheme),      // Slow path - on-demand compilation
}

impl ThemeLoader {
    pub fn load(theme_path: &Path) -> Result<Self, JynxError> {
        if needs_recompilation(theme_path) {
            // Recompile and save
            let runtime_theme = RuntimeTheme::from_yaml(theme_path)?;
            let compiled_theme = runtime_theme.compile()?;
            compiled_theme.save_to_disk()?;
            Ok(ThemeLoader::Compiled(compiled_theme))
        } else {
            // Load pre-compiled binary
            let compiled_theme = CompiledTheme::load_from_disk(theme_path)?;
            Ok(ThemeLoader::Compiled(compiled_theme))
        }
    }
    
    pub fn process_text(&self, text: &str, filter: &str) -> String {
        match self {
            ThemeLoader::Compiled(theme) => theme.highlight_fast(text, filter),
            ThemeLoader::Runtime(theme) => theme.highlight_slow(text, filter),
        }
    }
}
```

### **3. Binary Compilation Format**
```rust
use serde::{Serialize, Deserialize};
use regex::RegexSet;

#[derive(Serialize, Deserialize)]
pub struct CompiledTheme {
    // Metadata
    pub version: String,
    pub checksum: u64,
    pub compiled_at: SystemTime,
    
    // Pre-compiled patterns per filter
    pub filters: HashMap<String, CompiledFilter>,
}

#[derive(Serialize, Deserialize)]  
pub struct CompiledFilter {
    // Pre-compiled regex set (serialized)
    pub regex_patterns: Vec<String>,
    pub color_codes: Vec<u8>,
    pub style_flags: Vec<StyleFlags>,
    
    // Performance optimizations
    pub pattern_priorities: Vec<usize>,  // Longest-first ordering
    pub fast_lookup: HashMap<String, usize>,  // Direct keyword -> pattern mapping
}

// Bit flags for efficient style storage
#[derive(Serialize, Deserialize)]
pub struct StyleFlags {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
    pub color: Option<u8>,
}
```

## Theme Management CLI

### **Jynx Theme Commands**
```bash
# Theme management
jynx theme list                    # List available themes
jynx theme active                  # Show current theme
jynx theme set <name>             # Switch to theme
jynx theme edit                   # Open current theme in $EDITOR
jynx theme create <name>          # Create new custom theme
jynx theme compile               # Force recompilation
jynx theme validate              # Validate theme syntax
jynx theme benchmark             # Performance test current theme

# Output formatting
jynx --width 80 --align center    # Fixed width output with alignment
jynx -w 80 -a left               # Short form flags

# Theme development
jynx theme watch                 # Auto-recompile on changes (dev mode)
jynx theme reset                 # Reset to default theme
jynx theme export <name>         # Export theme to stdout
jynx theme import <file>         # Import theme from file
```

### **Implementation**
```rust
pub fn handle_theme_command(args: &ThemeArgs) -> Result<(), JynxError> {
    match args.subcommand {
        ThemeCommand::Edit => {
            let theme_path = get_active_theme_path()?;
            let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            
            // Open in editor
            Command::new(editor)
                .arg(&theme_path)
                .status()?;
            
            // Auto-recompile after edit
            println!("🔄 Recompiling theme...");
            force_recompile(&theme_path)?;
            println!("✅ Theme compiled successfully!");
        },
        
        ThemeCommand::Set { name } => {
            let theme_path = find_theme_by_name(&name)?;
            set_active_theme(theme_path)?;
            println!("🎨 Switched to theme: {}", name);
        },
        
        ThemeCommand::Benchmark => {
            benchmark_current_theme()?;
        },
        
        // ... other commands
    }
}
```

## Performance Optimizations

### **1. Compiled Regex Sets**
```rust
impl CompiledFilter {
    pub fn highlight_optimized(&self, text: &str) -> String {
        // Use pre-compiled RegexSet for maximum performance
        let matches = self.regex_set.matches(text);
        
        if matches.iter().count() == 0 {
            return text.to_string();  // Fast path: no matches
        }
        
        // Apply styles using pre-computed mappings
        self.apply_compiled_styles(text, matches)
    }
    
    fn apply_compiled_styles(&self, text: &str, matches: SetMatches) -> String {
        let mut result = text.to_string();
        
        // Process matches in priority order (pre-sorted)
        for pattern_id in matches.iter() {
            let color_code = self.color_codes[pattern_id];
            let style_flags = &self.style_flags[pattern_id];
            let pattern = &self.regex_patterns[pattern_id];
            
            // Apply styling with single regex operation
            result = apply_style_fast(&result, pattern, color_code, style_flags);
        }
        
        result
    }
}
```

### **2. Incremental Compilation**
```rust
pub struct ThemeCompiler {
    cache: HashMap<String, CompiledFilter>,
}

impl ThemeCompiler {
    pub fn compile_incremental(&mut self, theme: &RuntimeTheme) -> Result<CompiledTheme, JynxError> {
        let mut compiled_filters = HashMap::new();
        
        for (filter_name, filter_config) in &theme.filters {
            // Check if this filter changed since last compilation
            if let Some(cached) = self.cache.get(filter_name) {
                if !filter_config_changed(filter_config, cached) {
                    compiled_filters.insert(filter_name.clone(), cached.clone());
                    continue;
                }
            }
            
            // Compile only changed filters
            let compiled_filter = self.compile_filter(filter_config)?;
            self.cache.insert(filter_name.clone(), compiled_filter.clone());
            compiled_filters.insert(filter_name.clone(), compiled_filter);
        }
        
        Ok(CompiledTheme {
            version: env!("CARGO_PKG_VERSION").to_string(),
            checksum: calculate_theme_checksum(theme),
            compiled_at: SystemTime::now(),
            filters: compiled_filters,
        })
    }
}
```

## Theme Hot-Reloading (Development Mode)

### **Watch Mode Implementation**
```rust
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn start_theme_watch_mode() -> Result<(), JynxError> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1))?;
    let theme_dir = get_theme_directory()?;
    
    watcher.watch(&theme_dir, RecursiveMode::Recursive)?;
    
    println!("👀 Watching for theme changes in: {}", theme_dir.display());
    println!("Press Ctrl+C to stop");
    
    loop {
        match rx.recv() {
            Ok(event) => {
                if let Some(path) = extract_theme_path(&event) {
                    println!("📝 Theme changed: {}", path.display());
                    
                    match recompile_theme(&path) {
                        Ok(_) => println!("✅ Theme recompiled successfully"),
                        Err(e) => eprintln!("❌ Compilation failed: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {}", e),
        }
    }
}
```

## Example Theme Configuration

### **Enhanced theme.yml with icon mappings**
```yaml
# ~/.local/etc/rsb/jynx/theme.yml
metadata:
  name: "jynx-enhanced"
  version: "1.1.0"
  description: "Theme with icon mapping and inheritance support"
  
# Compilation hints for performance optimization
compilation:
  optimize_for: "speed"      # vs "size" vs "balanced"
  pattern_limit: 100         # Max patterns per filter
  enable_fast_lookup: true   # Build keyword->pattern hash map
  
filters:
  todo:
    # Icon mappings for :word: patterns
    icon_mappings:
      critical: { icon: "🔥", color: "red" }      # Full override
      urgent: { icon: "⚡" }                      # Icon only, inherits style color
      keeper: { icon: "🌑", color: "blue" }
      success: { icon: "✅", color: "green" }
      
    # Enhanced styles (keyword highlighting)
    styles:
      urgent_highlight:
        keywords: ["URGENT", "CRITICAL", "ASAP", "EMERGENCY"]
        color: "orange"
        bold: true
        underline: true
        priority: "high"
        
      completion_emphasis:
        keywords: ["DONE", "COMPLETE", "FINISHED", "RESOLVED"]
        color: "emerald" 
        bold: true
        priority: "medium"
        
      soft_notes:
        keywords: ["note", "info", "fyi", "reminder"]
        color: "azure"
        italic: true
        priority: "low"
```

## Performance Benchmarks

### **Expected Performance Gains**
```
Theme Loading Performance:
- Cold start (YAML + compilation): ~50ms
- Warm start (pre-compiled binary): ~0.5ms  
- Hot reload (file watch): ~10ms

Text Processing Performance:  
- Compiled theme: ~0.1ms per 1K lines
- Runtime theme: ~5ms per 1K lines
- 50x improvement with compilation!

Memory Usage:
- Compiled theme: ~2MB for complex themes
- Runtime theme: ~8MB (includes YAML parser overhead)
```

## Processing Flow Examples

### **Complete Processing Pipeline**
```
Input Text: "The :keeper: reviewed URGENT tasks v1.2.3 at /home/user/project.rs with :success: status"

Step 1 - Auto-Detection (Zero-Config Intelligence):
  - Detect "v1.2.3" → versions pattern → emerald + bold
  - Detect "/home/user/project.rs" → paths pattern → azure + underline

Step 2 - Icon Pattern Detection:
  - Detect `:keeper:` → lookup icon_mappings.keeper → 🌑 + blue
  - Detect `:success:` → lookup icon_mappings.success → ✅ + green

Step 3 - Keyword Style Application:  
  - Match "URGENT" → styles.urgent_highlight → orange + bold + underline

Step 4 - Layered Rendering:
  - "v1.2.3" → "v1.2.3" (emerald, bold)
  - "/home/user/project.rs" → "/home/user/project.rs" (azure, underline)
  - ":keeper:" → "🌑 keeper" (blue) 
  - "URGENT" → "URGENT" (orange, bold, underline)
  - ":success:" → "✅ success" (green)

Final Output: "The 🌑 keeper (blue) reviewed URGENT (orange,bold,underline) tasks v1.2.3 (emerald,bold) at /home/user/project.rs (azure,underline) with ✅ success (green) status"
```

### **Inheritance Override Example**
```yaml
# User theme with selective overrides
theme:
  filters:
    todo:
      icon_mappings:
        critical: none                           # Disable default 🔥 critical
        keeper: { icon: "🌚", color: "purple" } # Override default 🌑 blue
      styles:
        urgent_highlight: none                   # Disable default styling
```

### **CLI Output Formatting**
```bash
# Input with mixed content
echo "Status: :online: system :processing: data" | jynx -w 60 -a center

# Processed content  
"🟢 online (green) system 🔄 processing (aqua) data"

# Final formatted output (60 chars, centered)
"     🟢 online system 🔄 processing data     "
```

This theme system gives you **blazing performance** through smart compilation, **visual semantic enhancement** through icon mappings, and **flexible inheritance** for DRY configuration! 🚀