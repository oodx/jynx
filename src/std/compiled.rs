//! Compiled theme system for high-performance text processing
//! 
//! Pre-compiles regex patterns and stores them in optimized binary format

use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use regex::{Regex, RegexSet};
use crate::std::theme::{Theme, Filter, IconMapping, StyleGroup, AutoDetectionPattern};
use crate::extended_colors::get_extended_color_code;

/// Compiled theme with pre-optimized regex patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledTheme {
    // Metadata for cache validation
    pub version: String,
    pub source_checksum: u64,
    pub compiled_at: SystemTime,
    
    // Compiled auto-detection patterns
    pub auto_detection: Vec<CompiledAutoPattern>,
    
    // Compiled filters
    pub filters: HashMap<String, CompiledFilter>,
}

/// Compiled auto-detection pattern with regex and styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledAutoPattern {
    pub name: String,
    pub pattern_str: String, // Store pattern string for serialization
    #[serde(skip)]
    pub regex: Option<Regex>, // Runtime compiled regex
    pub ansi_style: String,
    pub icon: Option<String>,
}

/// Compiled filter with optimized pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledFilter {
    // Icon mappings (direct lookup)
    pub icon_mappings: HashMap<String, CompiledIconMapping>,
    
    // Keyword patterns (optimized for bulk matching)
    pub keyword_patterns: Vec<CompiledKeywordPattern>,
    
    // Fast lookup structures
    pub pattern_set_str: Vec<String>, // Store pattern strings for serialization
    #[serde(skip)]
    pub pattern_set: Option<RegexSet>, // Runtime compiled regex set
}

/// Compiled icon mapping with pre-formatted output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledIconMapping {
    pub icon: String,
    pub color_ansi: String,
    pub formatted_template: String, // Pre-built template: "{color}{icon} {word}{reset}"
}

/// Compiled keyword pattern with regex and styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledKeywordPattern {
    pub pattern_str: String, // Store for serialization
    #[serde(skip)]
    pub regex: Option<Regex>, // Runtime compiled regex
    pub ansi_style: String,
    pub keywords: Vec<String>, // Original keywords for reference
}

impl CompiledTheme {
    /// Compile a runtime theme into optimized form
    pub fn from_theme(theme: &Theme) -> Result<Self, Box<dyn std::error::Error>> {
        let mut compiled_theme = CompiledTheme {
            version: env!("CARGO_PKG_VERSION").to_string(),
            source_checksum: Self::calculate_theme_checksum(theme),
            compiled_at: SystemTime::now(),
            auto_detection: Vec::new(),
            filters: HashMap::new(),
        };
        
        // Compile auto-detection patterns
        for (name, pattern) in &theme.auto_detection {
            let compiled_pattern = CompiledAutoPattern::from_auto_pattern(name, pattern)?;
            compiled_theme.auto_detection.push(compiled_pattern);
        }
        
        // Compile filters
        for (filter_name, filter) in &theme.filters {
            let compiled_filter = CompiledFilter::from_filter(filter)?;
            compiled_theme.filters.insert(filter_name.clone(), compiled_filter);
        }
        
        Ok(compiled_theme)
    }
    
    /// Initialize runtime regex compilation after deserialization
    pub fn init_runtime(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Compile auto-detection patterns
        for pattern in &mut self.auto_detection {
            pattern.compile_regex()?;
        }
        
        // Compile filter patterns
        for filter in self.filters.values_mut() {
            filter.compile_patterns()?;
        }
        
        Ok(())
    }
    
    /// Calculate checksum for theme change detection
    fn calculate_theme_checksum(theme: &Theme) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Hash theme structure (simplified)
        theme.metadata.name.hash(&mut hasher);
        theme.metadata.version.hash(&mut hasher);
        
        // Hash auto-detection patterns
        for (name, pattern) in &theme.auto_detection {
            name.hash(&mut hasher);
            pattern.pattern.hash(&mut hasher);
            pattern.color.hash(&mut hasher);
        }
        
        // Hash filters
        for (filter_name, filter) in &theme.filters {
            filter_name.hash(&mut hasher);
            
            // Hash icon mappings
            for (icon_key, mapping) in &filter.icon_mappings {
                icon_key.hash(&mut hasher);
                mapping.icon.hash(&mut hasher);
                mapping.color.hash(&mut hasher);
            }
            
            // Hash styles
            for (style_key, style) in &filter.styles {
                style_key.hash(&mut hasher);
                style.keywords.hash(&mut hasher);
                style.color.hash(&mut hasher);
            }
        }
        
        hasher.finish()
    }
    
    /// High-performance text processing using compiled patterns
    pub fn process_text(&self, text: &str, filter_name: &str) -> String {
        let mut result = text.to_string();
        
        // Apply auto-detection first
        for pattern in &self.auto_detection {
            if let Some(ref regex) = pattern.regex {
                result = regex.replace_all(&result, |caps: &regex::Captures| {
                    let matched = caps.get(1).map_or(caps.get(0).unwrap().as_str(), |m| m.as_str());
                    if let Some(ref icon) = pattern.icon {
                        format!("{} {}{}{}", icon, pattern.ansi_style, matched, "\x1b[0m")
                    } else {
                        format!("{}{}{}", pattern.ansi_style, matched, "\x1b[0m")
                    }
                }).to_string();
            }
        }
        
        // Apply filter-specific processing
        if let Some(filter) = self.filters.get(filter_name) {
            // Apply icon mappings first
            let icon_regex = Regex::new(r":([a-zA-Z_][a-zA-Z0-9_]*):").unwrap();
            result = icon_regex.replace_all(&result, |caps: &regex::Captures| {
                let word = &caps[1];
                if let Some(mapping) = filter.icon_mappings.get(word) {
                    // Use pre-compiled template
                    mapping.formatted_template.replace("{word}", word)
                } else {
                    caps[0].to_string()
                }
            }).to_string();
            
            // Apply keyword highlighting
            for pattern in &filter.keyword_patterns {
                if let Some(ref regex) = pattern.regex {
                    result = regex.replace_all(&result, |caps: &regex::Captures| {
                        let matched = &caps[0];
                        format!("{}{}{}", pattern.ansi_style, matched, "\x1b[0m")
                    }).to_string();
                }
            }
        }
        
        result
    }
}

impl CompiledAutoPattern {
    fn from_auto_pattern(name: &str, pattern: &AutoDetectionPattern) -> Result<Self, Box<dyn std::error::Error>> {
        let ansi_style = pattern.to_ansi();
        
        // Determine if this pattern should have an icon (based on auto-detection type)
        let icon = match name {
            "paths" => Some("📁".to_string()),
            "versions" => Some("🏷️".to_string()),
            "urls" => Some("🔗".to_string()),
            _ => None,
        };
        
        Ok(CompiledAutoPattern {
            name: name.to_string(),
            pattern_str: pattern.pattern.clone(),
            regex: None, // Will be compiled at runtime
            ansi_style,
            icon,
        })
    }
    
    fn compile_regex(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Wrap pattern to capture the match
        let capture_pattern = if !self.pattern_str.contains('(') {
            format!("({})", self.pattern_str)
        } else {
            self.pattern_str.clone()
        };
        
        self.regex = Some(Regex::new(&capture_pattern)?);
        Ok(())
    }
}

impl CompiledFilter {
    fn from_filter(filter: &Filter) -> Result<Self, Box<dyn std::error::Error>> {
        let mut compiled_filter = CompiledFilter {
            icon_mappings: HashMap::new(),
            keyword_patterns: Vec::new(),
            pattern_set_str: Vec::new(),
            pattern_set: None,
        };
        
        // Compile icon mappings
        for (key, mapping) in &filter.icon_mappings {
            let compiled_mapping = CompiledIconMapping::from_icon_mapping(mapping);
            compiled_filter.icon_mappings.insert(key.clone(), compiled_mapping);
        }
        
        // Compile keyword patterns
        for (_style_name, style) in &filter.styles {
            let compiled_pattern = CompiledKeywordPattern::from_style_group(style)?;
            compiled_filter.keyword_patterns.push(compiled_pattern);
        }
        
        Ok(compiled_filter)
    }
    
    fn compile_patterns(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Compile individual regex patterns
        for pattern in &mut self.keyword_patterns {
            pattern.compile_regex()?;
        }
        
        // Build pattern set strings
        self.pattern_set_str = self.keyword_patterns
            .iter()
            .map(|p| p.pattern_str.clone())
            .collect();
        
        // Compile regex set for bulk matching optimization
        if !self.pattern_set_str.is_empty() {
            self.pattern_set = Some(RegexSet::new(&self.pattern_set_str)?);
        }
        
        Ok(())
    }
}

impl CompiledIconMapping {
    fn from_icon_mapping(mapping: &IconMapping) -> Self {
        let color_ansi = get_extended_color_code(&mapping.color);
        let formatted_template = format!("{}{} {{word}}\x1b[0m", color_ansi, mapping.icon);
        
        CompiledIconMapping {
            icon: mapping.icon.clone(),
            color_ansi: color_ansi.to_string(),
            formatted_template,
        }
    }
}

impl CompiledKeywordPattern {
    fn from_style_group(style: &StyleGroup) -> Result<Self, Box<dyn std::error::Error>> {
        let ansi_style = style.to_ansi();
        
        // Create unified pattern for all keywords in this style group
        let escaped_keywords: Vec<String> = style.keywords
            .iter()
            .map(|k| {
                if k.contains(":") || k.contains(" ") {
                    // Literal matching for phrases
                    format!("(?i){}", regex::escape(k))
                } else {
                    // Word boundary matching for single words
                    format!(r"(?i)\b{}\b", regex::escape(k))
                }
            })
            .collect();
        
        let pattern_str = format!("({})", escaped_keywords.join("|"));
        
        Ok(CompiledKeywordPattern {
            pattern_str,
            regex: None, // Will be compiled at runtime
            ansi_style,
            keywords: style.keywords.clone(),
        })
    }
    
    fn compile_regex(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.regex = Some(Regex::new(&self.pattern_str)?);
        Ok(())
    }
}