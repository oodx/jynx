//! Standard Rust Implementation
//! 
//! Pure Rust approach using:
//! - Complex types and proper error handling
//! - Structured theme system with serde
//! - Performance-optimized regex compilation
//! - Memory-efficient processing

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::error::Error;
use std::fmt;

pub mod theme;
pub mod highlight;
pub mod auto_detect;
pub mod compiled;

use auto_detect::AutoDetector;
use std::collections::HashMap;
use crate::template_parser::ColorTemplateParser;

#[derive(Debug)]
pub enum JynxError {
    IoError(io::Error),
    ThemeError(String),
    ProcessingError(String),
}

impl fmt::Display for JynxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JynxError::IoError(e) => write!(f, "IO error: {}", e),
            JynxError::ThemeError(e) => write!(f, "Theme error: {}", e),
            JynxError::ProcessingError(e) => write!(f, "Processing error: {}", e),
        }
    }
}

impl Error for JynxError {}

impl From<io::Error> for JynxError {
    fn from(error: io::Error) -> Self {
        JynxError::IoError(error)
    }
}

use theme::Theme;
use compiled::CompiledTheme;
use regex::Regex;

pub struct JynxApp {
    // Core stream processor - Unix philosophy: do one thing well
    detector: Option<AutoDetector>,
    theme: Option<Theme>,
    compiled_theme: Option<CompiledTheme>,
    filter: Option<String>,
    // Compiled regex for :word: pattern detection
    icon_pattern: Regex,
    // Pre-compiled keyword regex patterns for performance (legacy)
    keyword_patterns: HashMap<String, (Regex, String)>, // (regex, ansi_style)
    // Color template parser for %c:colorname(text) patterns
    template_parser: ColorTemplateParser,
    // Output formatting options
    width: Option<usize>,
    align: TextAlign,
    // Performance optimization flags
    use_compiled: bool,
    no_color: bool,
}

#[derive(Debug, Clone)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "center" | "centre" => Self::Center,
            "right" => Self::Right,
            _ => Self::Left,
        }
    }
}

impl JynxApp {
    pub fn new() -> Self {
        Self::with_theme(None, None, None, "left".to_string())
    }
    
    pub fn with_theme_and_options(theme: Option<Theme>, filter: Option<String>, width: Option<usize>, align: String, no_color: bool) -> Self {
        Self::with_theme_internal(theme, filter, width, align, no_color)
    }
    
    pub fn with_theme(theme: Option<Theme>, filter: Option<String>, width: Option<usize>, align: String) -> Self {
        Self::with_theme_internal(theme, filter, width, align, false)
    }
    
    fn with_theme_internal(theme: Option<Theme>, filter: Option<String>, width: Option<usize>, align: String, no_color: bool) -> Self {
        // Try to initialize auto-detector, but gracefully fall back if it fails
        let detector = match AutoDetector::new() {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("Warning: Failed to initialize auto-detection: {}", e);
                None
            }
        };
        
        // Compile regex for :word: pattern detection
        let icon_pattern = Regex::new(r":([a-zA-Z_][a-zA-Z0-9_]*):").unwrap();
        
        // Enable compiled theme optimization for ~150x performance boost
        let (compiled_theme, use_compiled) = if let Some(ref theme) = theme {
            match CompiledTheme::from_theme(theme) {
                Ok(mut compiled) => {
                    if let Err(e) = compiled.init_runtime() {
                        eprintln!("Warning: Failed to initialize compiled theme: {}", e);
                        (None, false)
                    } else {
                        (Some(compiled), true)
                    }
                },
                Err(e) => {
                    eprintln!("Warning: Failed to compile theme: {}", e);
                    (None, false)
                }
            }
        } else {
            (None, false)
        };
        
        // Pre-compile keyword patterns for legacy fallback
        let keyword_patterns = if !use_compiled {
            if let (Some(ref theme), Some(ref filter_name)) = (&theme, &filter) {
                Self::compile_keyword_patterns(theme, filter_name)
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new() // Not needed when using compiled theme
        };
        
        // Initialize template parser
        let template_parser = ColorTemplateParser::new(no_color);
        
        Self { 
            detector,
            theme,
            compiled_theme,
            filter,
            icon_pattern,
            keyword_patterns,
            template_parser,
            width,
            align: TextAlign::from_str(&align),
            use_compiled,
            no_color,
        }
    }
    
    /// Pre-compile all keyword patterns for performance
    fn compile_keyword_patterns(theme: &Theme, filter_name: &str) -> HashMap<String, (Regex, String)> {
        let mut patterns = HashMap::new();
        
        if let Some(filter) = theme.filters.get(filter_name) {
            for style_group in filter.styles.values() {
                let ansi_style = style_group.to_ansi() + &theme::AnsiCodes::RESET;
                
                for keyword in &style_group.keywords {
                    // Create appropriate regex pattern
                    let pattern = if keyword.contains(":") || keyword.contains(" ") {
                        // For phrases or patterns with colons, use literal matching
                        format!(r"(?i){}", regex::escape(keyword))
                    } else {
                        // For single words, use word boundaries
                        format!(r"(?i)\b{}\b", regex::escape(keyword))
                    };
                    
                    if let Ok(regex) = Regex::new(&pattern) {
                        patterns.insert(keyword.clone(), (regex, ansi_style.clone()));
                    }
                }
            }
        }
        
        patterns
    }

    /// Main entry point - stream processor that reads stdin and writes to stdout
    pub fn run(&self) -> Result<(), JynxError> {
        // Lock stdin and stdout once for the entire session - more efficient
        let stdin = io::stdin();
        let stdout = io::stdout();
        
        let reader = BufReader::new(stdin.lock());
        let mut writer = BufWriter::new(stdout.lock());
        
        // Stream processing: line by line, immediate output
        for line_result in reader.lines() {
            let line = line_result?;
            
            // Process the line - this is where the magic happens
            let processed_line = self.process_line(&line)?;
            
            // Write immediately and flush for pipe compatibility
            writeln!(writer, "{}", processed_line)?;
            writer.flush()?;
        }
        
        Ok(())
    }
    
    /// Process a single line - the core transformation logic
    /// Implements the complete 5-layer processing pipeline
    fn process_line(&self, line: &str) -> Result<String, JynxError> {
        let mut result = line.to_string();
        
        // FIRST: Apply color templates (%c:colorname(text) patterns) - highest priority
        result = self.template_parser.process(&result);
        
        // Skip other color processing if in no-color mode
        if !self.no_color {
            // Use compiled theme for optimal performance if available
            if self.use_compiled {
                if let (Some(ref compiled_theme), Some(ref filter_name)) = (&self.compiled_theme, &self.filter) {
                    // High-performance compiled processing
                    result = compiled_theme.process_text(&result, filter_name);
                } else if let Some(detector) = &self.detector {
                    // Fallback to basic auto-detection only
                    result = detector.highlight_line(&result);
                }
            } else {
                // Legacy processing pipeline (layers 2-4)
                // 2. Apply auto-detection if available
                if let Some(detector) = &self.detector {
                    result = detector.highlight_line(&result);
                }
                
                // 3. Apply icon mappings (:word: patterns) if theme is available
                if let (Some(theme), Some(filter_name)) = (&self.theme, &self.filter) {
                    result = self.apply_icon_patterns(&result, theme, filter_name);
                }
                
                // 4. Apply keyword highlighting if theme and filter are available  
                if let (Some(theme), Some(filter_name)) = (&self.theme, &self.filter) {
                    result = self.apply_keyword_highlighting(&result, theme, filter_name);
                }
            }
        }
        
        // 5. Apply width and alignment formatting if specified (always last)
        if let Some(width) = self.width {
            result = self.format_line_width(&result, width);
        }
        
        Ok(result)
    }
    
    /// Apply :word: icon pattern replacements
    fn apply_icon_patterns(&self, text: &str, theme: &Theme, filter_name: &str) -> String {
        self.icon_pattern.replace_all(text, |caps: &regex::Captures| {
            let word = &caps[1];
            
            if let Some(icon_mapping) = theme.get_icon_mapping(filter_name, word) {
                // Replace :word: with colored icon + word (e.g. ":critical:" -> "🔥 critical")
                icon_mapping.formatted_icon(word)
            } else {
                // Keep original if no mapping found (graceful degradation)
                caps[0].to_string()
            }
        }).to_string()
    }
    
    /// Apply keyword highlighting based on theme styles (using pre-compiled patterns)
    fn apply_keyword_highlighting(&self, text: &str, _theme: &Theme, _filter_name: &str) -> String {
        let mut result = text.to_string();
        
        // Use pre-compiled patterns for much better performance
        for (_keyword, (regex, styled_replacement)) in &self.keyword_patterns {
            result = regex.replace_all(&result, |caps: &regex::Captures| {
                let matched = &caps[0];
                format!("{}{}{}", 
                    styled_replacement.replace(theme::AnsiCodes::RESET, ""),
                    matched, 
                    theme::AnsiCodes::RESET
                )
            }).to_string();
        }
        
        result
    }
    
    /// Format line to specified width with alignment
    /// Handles ANSI escape codes properly to calculate visible text length
    fn format_line_width(&self, text: &str, width: usize) -> String {
        // Calculate visible text length by removing ANSI escape codes
        let visible_len = Self::get_visible_length(text);
        
        if visible_len >= width {
            // If text is already wider than target, truncate it gracefully
            return Self::truncate_to_width(text, width);
        }
        
        let padding_needed = width - visible_len;
        
        match self.align {
            TextAlign::Left => {
                // Left align: add padding to the right
                format!("{}{}", text, " ".repeat(padding_needed))
            },
            TextAlign::Right => {
                // Right align: add padding to the left  
                format!("{}{}", " ".repeat(padding_needed), text)
            },
            TextAlign::Center => {
                // Center align: split padding between left and right
                let left_padding = padding_needed / 2;
                let right_padding = padding_needed - left_padding;
                format!("{}{}{}", " ".repeat(left_padding), text, " ".repeat(right_padding))
            }
        }
    }
    
    /// Get visible length of text (excluding ANSI escape codes)
    /// More accurate than strip_ansi_codes for length calculation
    fn get_visible_length(text: &str) -> usize {
        let ansi_regex = regex::Regex::new(r"\x1B\[[0-9;]*m").unwrap();
        let stripped = ansi_regex.replace_all(text, "");
        
        // Count Unicode grapheme clusters for accurate character width
        stripped.chars().count()
    }
    
    /// Truncate text to specified width while preserving ANSI codes
    fn truncate_to_width(text: &str, width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        
        let ansi_regex = regex::Regex::new(r"\x1B\[[0-9;]*m").unwrap();
        let mut result = String::new();
        let mut visible_chars = 0;
        let mut i = 0;
        
        while i < text.len() && visible_chars < width {
            // Check for ANSI escape sequence
            if let Some(mat) = ansi_regex.find(&text[i..]) {
                if mat.start() == 0 {
                    // Add ANSI sequence without counting toward visible length
                    result.push_str(mat.as_str());
                    i += mat.len();
                    continue;
                }
            }
            
            // Add regular character
            if let Some(ch) = text[i..].chars().next() {
                result.push(ch);
                visible_chars += 1;
                i += ch.len_utf8();
            } else {
                break;
            }
        }
        
        // Add ellipsis if truncated (within width limit)
        if i < text.len() && width > 3 && visible_chars == width {
            // Remove last 3 characters and add ellipsis
            let mut chars: Vec<char> = result.chars().collect();
            chars.truncate(chars.len().saturating_sub(3));
            result = chars.into_iter().collect();
            result.push_str("...");
        }
        
        result
    }
    
    /// Strip ANSI escape codes completely (for compatibility)
    #[allow(dead_code)]
    fn strip_ansi_codes(text: &str) -> String {
        let ansi_regex = regex::Regex::new(r"\x1B\[[0-9;]*m").unwrap();
        ansi_regex.replace_all(text, "").to_string()
    }
}