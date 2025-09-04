//! Theme system for jynx
//! 
//! Handles loading and parsing YAML theme files with icon mappings and inheritance

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use crate::extended_colors::get_extended_color_code;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoDetectionPattern {
    pub pattern: String,
    pub color: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub dim: bool,
    #[serde(default)]
    pub strikethrough: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IconMapping {
    pub icon: String,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StyleGroup {
    pub keywords: Vec<String>,
    pub color: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub dim: bool,
    #[serde(default)]
    pub strikethrough: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Filter {
    #[serde(default)]
    pub icon_mappings: HashMap<String, IconMapping>,
    pub styles: HashMap<String, StyleGroup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompilationSettings {
    pub optimize_for: String,
    pub pattern_limit: usize,
    pub enable_fast_lookup: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Theme {
    pub metadata: ThemeMetadata,
    #[serde(default)]
    pub defaults: Option<ThemeDefaults>,
    #[serde(default)]
    pub auto_detection: HashMap<String, AutoDetectionPattern>,
    #[serde(default)]
    pub compilation: Option<CompilationSettings>,
    pub filters: HashMap<String, Filter>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThemeDefaults {
    #[serde(default)]
    pub auto_detection: HashMap<String, AutoDetectionPattern>,
    #[serde(default)]
    pub filters: HashMap<String, Filter>,
}

/// Special value to disable inheritance
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InheritanceValue<T> {
    Value(T),
    Disabled(String), // "none" string to disable
}

impl<T> InheritanceValue<T> {
    pub fn is_disabled(&self) -> bool {
        match self {
            InheritanceValue::Disabled(s) => s == "none",
            _ => false,
        }
    }
    
    pub fn value(&self) -> Option<&T> {
        match self {
            InheritanceValue::Value(v) => Some(v),
            _ => None,
        }
    }
}

impl Theme {
    /// Get XDG+ theme directory path
    pub fn xdg_theme_dir() -> PathBuf {
        if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".local/etc/rsb/jynx/themes")
        } else {
            PathBuf::from(".local/etc/rsb/jynx/themes")
        }
    }
    
    /// Resolve theme name to actual file path with XDG+ fallback hierarchy
    /// - `rebel` → `~/.local/etc/rsb/jynx/themes/theme_rebel.yml`
    /// - `./my_theme.yml` → relative path as-is
    /// - `/abs/path.yml` → absolute path as-is
    pub fn resolve_theme_path(theme_name: &str) -> Option<PathBuf> {
        // Handle relative and absolute paths directly
        if theme_name.starts_with("./") || theme_name.starts_with("/") || theme_name.ends_with(".yml") {
            let path = PathBuf::from(theme_name);
            return if path.exists() { Some(path) } else { None };
        }
        
        // Handle theme name resolution with fallback hierarchy
        let theme_filename = format!("theme_{}.yml", theme_name);
        
        // 1. XDG+ location first
        let xdg_path = Self::xdg_theme_dir().join(&theme_filename);
        if xdg_path.exists() {
            return Some(xdg_path);
        }
        
        // 2. Local ./themes/ directory
        let local_path = PathBuf::from("themes").join(&theme_filename);
        if local_path.exists() {
            return Some(local_path);
        }
        
        // 3. Try direct filename in XDG+
        let direct_xdg_path = Self::xdg_theme_dir().join(theme_name);
        if direct_xdg_path.exists() {
            return Some(direct_xdg_path);
        }
        
        // 4. Try direct filename in local themes
        let direct_local_path = PathBuf::from("themes").join(theme_name);
        if direct_local_path.exists() {
            return Some(direct_local_path);
        }
        
        None
    }
    
    /// Load theme with smart resolution
    pub fn load_theme(theme_name: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        match theme_name {
            Some(name) => {
                if let Some(path) = Self::resolve_theme_path(name) {
                    Self::load_from_file(path)
                } else {
                    Err(format!("Theme '{}' not found in XDG+ or local themes", name).into())
                }
            },
            None => {
                // Try default theme from XDG+ first
                if let Some(path) = Self::resolve_theme_path("default") {
                    Self::load_from_file(path)
                } else {
                    // Fallback to embedded default
                    Ok(Self::default())
                }
            }
        }
    }
    
    /// Load theme from YAML file with inheritance support
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut theme: Theme = serde_yaml::from_str(&content)?;
        
        // Apply inheritance if defaults are present
        theme.apply_inheritance();
        
        Ok(theme)
    }
    
    /// Apply theme inheritance: defaults + user overrides
    fn apply_inheritance(&mut self) {
        if let Some(defaults) = &self.defaults.clone() {
            // Merge auto_detection patterns
            for (key, default_pattern) in &defaults.auto_detection {
                if !self.auto_detection.contains_key(key) {
                    self.auto_detection.insert(key.clone(), default_pattern.clone());
                }
            }
            
            // Merge filters with selective override support
            for (filter_name, default_filter) in &defaults.filters {
                if let Some(user_filter) = self.filters.get_mut(filter_name) {
                    // Merge icon mappings (user overrides defaults)
                    for (icon_key, default_icon) in &default_filter.icon_mappings {
                        if !user_filter.icon_mappings.contains_key(icon_key) {
                            user_filter.icon_mappings.insert(icon_key.clone(), default_icon.clone());
                        }
                    }
                    
                    // Merge styles (user overrides defaults)
                    for (style_key, default_style) in &default_filter.styles {
                        if !user_filter.styles.contains_key(style_key) {
                            user_filter.styles.insert(style_key.clone(), default_style.clone());
                        }
                    }
                } else {
                    // No user filter exists, use defaults entirely
                    self.filters.insert(filter_name.clone(), default_filter.clone());
                }
            }
        }
    }
    
    /// Get default theme if no file is provided
    pub fn default() -> Self {
        Theme {
            metadata: ThemeMetadata {
                name: "jynx-minimal".to_string(),
                version: "1.0.0".to_string(),
                description: "Minimal default theme with auto-detection only".to_string(),
            },
            defaults: None,
            auto_detection: HashMap::new(),
            compilation: None,
            filters: HashMap::new(),
        }
    }
    
    /// Get icon mapping for a word pattern
    pub fn get_icon_mapping(&self, filter_name: &str, word: &str) -> Option<&IconMapping> {
        self.filters
            .get(filter_name)?
            .icon_mappings
            .get(word)
    }
    
    /// Get all keywords for a specific filter
    pub fn get_filter_keywords(&self, filter_name: &str) -> Vec<&str> {
        if let Some(filter) = self.filters.get(filter_name) {
            filter.styles.values()
                .flat_map(|style| style.keywords.iter().map(|s| s.as_str()))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// List all available themes (XDG+ and local)
    pub fn list_themes() -> Result<Vec<(String, PathBuf, String)>, Box<dyn std::error::Error>> {
        let mut themes = Vec::new();
        
        // Check XDG+ themes
        let xdg_dir = Self::xdg_theme_dir();
        if xdg_dir.exists() {
            for entry in fs::read_dir(&xdg_dir)? {
                let entry = entry?;
                let path = entry.path();
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        if filename_str.ends_with(".yml") {
                            let theme_name = if filename_str.starts_with("theme_") {
                                filename_str.strip_prefix("theme_").unwrap().strip_suffix(".yml").unwrap()
                            } else {
                                filename_str.strip_suffix(".yml").unwrap()
                            };
                            themes.push((theme_name.to_string(), path.clone(), "XDG+".to_string()));
                        }
                    }
                }
            }
        }
        
        // Check local themes
        let local_dir = PathBuf::from("themes");
        if local_dir.exists() {
            for entry in fs::read_dir(&local_dir)? {
                let entry = entry?;
                let path = entry.path();
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        if filename_str.ends_with(".yml") {
                            let theme_name = if filename_str.starts_with("theme_") {
                                filename_str.strip_prefix("theme_").unwrap().strip_suffix(".yml").unwrap()
                            } else {
                                filename_str.strip_suffix(".yml").unwrap()
                            };
                            // Only add if not already found in XDG+
                            if !themes.iter().any(|(name, _, _)| name == theme_name) {
                                themes.push((theme_name.to_string(), path.clone(), "local".to_string()));
                            }
                        }
                    }
                }
            }
        }
        
        themes.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(themes)
    }
    
    /// Create a new theme by copying default theme to current location
    pub fn create_theme(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let target_path = PathBuf::from(format!("{}.yml", name));
        
        // Load default theme or use embedded default
        let theme = Self::load_theme(Some("default")).unwrap_or_else(|_| Self::default());
        
        // Serialize theme to YAML
        let yaml_content = serde_yaml::to_string(&theme)?;
        
        fs::write(&target_path, yaml_content)?;
        Ok(target_path)
    }
    
    /// Import theme from current location to XDG+
    pub fn import_theme(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let source_path = PathBuf::from(format!("{}.yml", name));
        if !source_path.exists() {
            return Err(format!("Theme file '{}' not found in current directory", source_path.display()).into());
        }
        
        let target_dir = Self::xdg_theme_dir();
        fs::create_dir_all(&target_dir)?;
        
        let target_path = target_dir.join(format!("theme_{}.yml", name));
        fs::copy(&source_path, &target_path)?;
        
        Ok(target_path)
    }
    
    /// Export theme from XDG+ to current location
    pub fn export_theme(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let source_path = Self::xdg_theme_dir().join(format!("theme_{}.yml", name));
        if !source_path.exists() {
            return Err(format!("Theme '{}' not found in XDG+ themes", name).into());
        }
        
        let target_path = PathBuf::from(format!("{}.yml", name));
        fs::copy(&source_path, &target_path)?;
        
        Ok(target_path)
    }
    
    /// Edit theme in $EDITOR
    pub fn edit_theme(name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let theme_path = Self::resolve_theme_path(name)
            .ok_or_else(|| format!("Theme '{}' not found", name))?;
        
        let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
        
        let mut child = std::process::Command::new(&editor)
            .arg(&theme_path)
            .spawn()?;
        
        let status = child.wait()?;
        if !status.success() {
            return Err(format!("Editor '{}' exited with error", editor).into());
        }
        
        Ok(())
    }
}

/// ANSI style codes
pub struct AnsiCodes;
impl AnsiCodes {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const ITALIC: &'static str = "\x1b[3m";
    pub const UNDERLINE: &'static str = "\x1b[4m";
    pub const STRIKETHROUGH: &'static str = "\x1b[9m";
}

impl StyleGroup {
    /// Convert style group to ANSI escape sequence
    pub fn to_ansi(&self) -> String {
        let mut ansi = String::new();
        
        // Add color first
        ansi.push_str(get_extended_color_code(&self.color));
        
        // Add text styles
        if self.bold {
            ansi.push_str(AnsiCodes::BOLD);
        }
        if self.dim {
            ansi.push_str(AnsiCodes::DIM);
        }
        if self.italic {
            ansi.push_str(AnsiCodes::ITALIC);
        }
        if self.underline {
            ansi.push_str(AnsiCodes::UNDERLINE);
        }
        if self.strikethrough {
            ansi.push_str(AnsiCodes::STRIKETHROUGH);
        }
        
        ansi
    }
}

impl AutoDetectionPattern {
    /// Convert auto-detection pattern to ANSI escape sequence
    pub fn to_ansi(&self) -> String {
        let mut ansi = String::new();
        
        // Add color first
        ansi.push_str(get_extended_color_code(&self.color));
        
        // Add text styles
        if self.bold {
            ansi.push_str(AnsiCodes::BOLD);
        }
        if self.dim {
            ansi.push_str(AnsiCodes::DIM);
        }
        if self.italic {
            ansi.push_str(AnsiCodes::ITALIC);
        }
        if self.underline {
            ansi.push_str(AnsiCodes::UNDERLINE);
        }
        if self.strikethrough {
            ansi.push_str(AnsiCodes::STRIKETHROUGH);
        }
        
        ansi
    }
}

impl IconMapping {
    /// Get formatted icon with color following the spec: ":word:" -> "🔥 word"
    /// Icon is prefixed OUTSIDE color codes to avoid ANSI wrapping issues
    pub fn formatted_icon(&self, word: &str) -> String {
        format!("{} {}{}{}", 
            self.icon,
            get_extended_color_code(&self.color), 
            word,
            AnsiCodes::RESET
        )
    }
}