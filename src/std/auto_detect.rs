//! Auto-detection patterns
//! 
//! Recognizes common patterns like paths, versions, URLs automatically

use regex::Regex;

/// ANSI color codes for basic highlighting
pub struct Colors;
impl Colors {
    pub const AZURE: &'static str = "\x1b[36m";      // Cyan for paths
    pub const EMERALD: &'static str = "\x1b[32m";    // Green for versions  
    pub const ROYAL: &'static str = "\x1b[34m";      // Blue for URLs
    pub const RESET: &'static str = "\x1b[0m";       // Reset
    pub const UNDERLINE: &'static str = "\x1b[4m";   // Underline
    pub const BOLD: &'static str = "\x1b[1m";        // Bold
}

/// Unicode emoji icons for visual enhancement
pub struct Icons;
impl Icons {
    pub const PATH: &'static str = "📁";      // Folder icon for paths
    pub const VERSION: &'static str = "🏷️";   // Tag icon for versions
    pub const URL: &'static str = "🔗";       // Link icon for URLs
    
    // Fallback text icons if Unicode isn't supported
    pub const PATH_FALLBACK: &'static str = "[PATH]";
    pub const VERSION_FALLBACK: &'static str = "[VER]";
    pub const URL_FALLBACK: &'static str = "[URL]";
}

/// Check if terminal supports Unicode (basic heuristic)
fn supports_unicode() -> bool {
    // Check if LANG or LC_ALL contains UTF-8
    std::env::var("LANG")
        .or_else(|_| std::env::var("LC_ALL"))
        .map(|s| s.contains("UTF-8") || s.contains("utf8"))
        .unwrap_or(false)
}

/// Core auto-detection patterns - start with just 3 for MVP
pub struct AutoDetector {
    // Ordered list: (name, regex, style, icon)
    patterns: Vec<(String, Regex, String, String)>,
}

impl AutoDetector {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut patterns = Vec::new();
        let use_unicode = supports_unicode();
        
        // Apply in order: URLs first (most specific), then versions, then paths
        
        // URL pattern: http:// or https:// (complete URLs)
        patterns.push((
            "urls".to_string(),
            Regex::new(r"(https?://[^\s]+)")?,
            format!("{}{}", Colors::UNDERLINE, Colors::ROYAL),
            if use_unicode { Icons::URL.to_string() } else { Icons::URL_FALLBACK.to_string() },
        ));
        
        // Version pattern: 1.2.3 or 2.0.0-alpha  
        patterns.push((
            "versions".to_string(),
            Regex::new(r"\bv?(\d+\.\d+\.\d+(-\w+)?)\b")?,
            format!("{}{}", Colors::BOLD, Colors::EMERALD),
            if use_unicode { Icons::VERSION.to_string() } else { Icons::VERSION_FALLBACK.to_string() },
        ));
        
        // Path pattern: filesystem paths (simple and robust)
        patterns.push((
            "paths".to_string(),
            Regex::new(r"\b([~/][^\s]+\.[a-z]{2,4})\b")?,
            format!("{}{}", Colors::UNDERLINE, Colors::AZURE),
            if use_unicode { Icons::PATH.to_string() } else { Icons::PATH_FALLBACK.to_string() },
        ));
        
        Ok(AutoDetector { patterns })
    }
    
    /// Apply auto-detection to a line and return highlighted version
    pub fn highlight_line(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Apply each pattern in sequence with icons
        for (_name, regex, style, icon) in &self.patterns {
            result = regex.replace_all(&result, |caps: &regex::Captures| {
                format!("{} {}{}{}", icon, style, &caps[1], Colors::RESET)
            }).to_string();
        }
        
        result
    }
}