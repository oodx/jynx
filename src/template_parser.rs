//! Color Template Parser
//!
//! Implements %c:colorname(text) templating system with bulletproof parsing
//! - NO NESTING ALLOWED: treats nested patterns as literal text
//! - Balanced parentheses: handles (), [], % signs, function calls
//! - Independent parsing: each template processed separately
//! - Graceful fallback: invalid patterns remain as literal text

use crate::extended_colors::get_extended_color_code;

/// Template parser for %c:colorname(text) patterns
pub struct ColorTemplateParser {
    /// No-color mode flag
    no_color: bool,
}

impl ColorTemplateParser {
    /// Create new parser with optional no-color mode
    pub fn new(no_color: bool) -> Self {
        Self {
            no_color,
        }
    }
    
    /// Process text with color templates
    pub fn process(&self, text: &str) -> String {
        if self.no_color {
            // In no-color mode, strip templates to plain text
            self.strip_templates(text)
        } else {
            // Apply color templates
            self.apply_templates(text)
        }
    }
    
    /// Apply color templates, converting %c:colorname(text) to colored text
    fn apply_templates(&self, text: &str) -> String {
        self.process_templates(text, false)
    }
    
    /// Strip all templates to plain text only
    fn strip_templates(&self, text: &str) -> String {
        self.process_templates(text, true)
    }
    
    /// Process templates with unified logic for both color and no-color modes
    fn process_templates(&self, text: &str, strip_only: bool) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            // Try to match a template at current position
            if let Some((template_end, processed_content)) = self.try_parse_template_at(&chars, i, strip_only) {
                result.push_str(&processed_content);
                i = template_end;
            } else {
                // No template match, add current character
                result.push(chars[i]);
                i += 1;
            }
        }
        
        result
    }
    
    /// Try to parse a template starting at the given position
    /// Returns (end_position, processed_content) on success
    fn try_parse_template_at(&self, chars: &[char], start: usize, strip_only: bool) -> Option<(usize, String)> {
        // Check if we have enough characters for a minimal template
        if start + 4 >= chars.len() {
            return None;
        }
        
        // Check for %c: prefix
        if chars[start] != '%' || chars[start + 1] != 'c' || chars[start + 2] != ':' {
            return None;
        }
        
        // Find the opening parenthesis and extract color name
        let mut color_name = String::new();
        let mut i = start + 3;
        
        // Extract color name until we find '('
        while i < chars.len() {
            let ch = chars[i];
            if ch == '(' {
                break;
            } else if ch.is_alphabetic() || ch == '_' || ch.is_numeric() {
                color_name.push(ch);
            } else {
                // Invalid character in color name
                return None;
            }
            i += 1;
        }
        
        // Check if we found the opening parenthesis
        if i >= chars.len() || chars[i] != '(' {
            return None;
        }
        
        // Find balanced content
        let content_start = i + 1; // After the '('
        let (content_end, content) = self.find_balanced_content_from_chars(chars, content_start)?;
        
        // Process the template
        if strip_only {
            Some((content_end + 1, content)) // +1 to skip the closing ')'
        } else {
            // Get color code
            let color_code = get_extended_color_code(&color_name);
            if color_code.is_empty() {
                // Unknown color, return None to keep as literal
                return None;
            }
            
            let colored_text = format!("{}{}\x1B[0m", color_code, content);
            Some((content_end + 1, colored_text)) // +1 to skip the closing ')'
        }
    }
    
    /// Find balanced parentheses content from character array
    fn find_balanced_content_from_chars(&self, chars: &[char], start: usize) -> Option<(usize, String)> {
        if start >= chars.len() {
            return None;
        }
        
        let mut depth = 1; // We start after the opening (
        let mut content = String::new();
        let mut pos = start;
        
        while pos < chars.len() && depth > 0 {
            let ch = chars[pos];
            
            match ch {
                '(' => {
                    depth += 1;
                    content.push(ch);
                },
                ')' => {
                    depth -= 1;
                    if depth > 0 {
                        content.push(ch);
                    }
                    // If depth == 0, we found our closing ), don't include it
                },
                _ => {
                    content.push(ch);
                }
            }
            
            pos += 1;
        }
        
        // Check if we found balanced parentheses
        if depth == 0 {
            Some((pos - 1, content)) // pos-1 because we incremented after finding the )
        } else {
            // Unbalanced parentheses, template is invalid
            None
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_template() {
        let parser = ColorTemplateParser::new(false);
        let result = parser.process("Status: %c:red(FAILED)");
        assert!(result.contains("FAILED"));
        assert!(result.contains("\x1B[38;5;9m")); // red color code
        assert!(result.contains("\x1B[0m"));       // reset code
    }
    
    #[test]
    fn test_no_color_mode() {
        let parser = ColorTemplateParser::new(true);
        let result = parser.process("Status: %c:red(FAILED) %c:green(OK)");
        assert_eq!(result, "Status: FAILED OK");
    }
    
    #[test]
    fn test_balanced_parentheses() {
        let parser = ColorTemplateParser::new(false);
        let result = parser.process("%c:red(())");
        assert!(result.contains("()"));
    }
    
    #[test]
    fn test_function_call_content() {
        let parser = ColorTemplateParser::new(true);
        let result = parser.process("%c:amber(function(param))");
        assert_eq!(result, "function(param)");
    }
    
    #[test]
    fn test_multiple_templates() {
        let parser = ColorTemplateParser::new(true);
        let result = parser.process("Status: %c:emerald(SUCCESS) - %c:crimson(3 errors)");
        assert_eq!(result, "Status: SUCCESS - 3 errors");
    }
    
    #[test]
    fn test_square_brackets() {
        let parser = ColorTemplateParser::new(true);
        let result = parser.process("%c:blue([value])");
        assert_eq!(result, "[value]");
    }
    
    #[test]
    fn test_percent_signs() {
        let parser = ColorTemplateParser::new(true);
        let result = parser.process("%c:green(%test%)");
        assert_eq!(result, "%test%");
    }
    
    #[test]
    fn test_mixed_parentheses() {
        let parser = ColorTemplateParser::new(true);
        let result = parser.process("%c:crimson(Error: (code 42))");
        assert_eq!(result, "Error: (code 42)");
    }
    
    #[test]
    fn test_percentage_values() {
        let parser = ColorTemplateParser::new(true);
        let result = parser.process("%c:emerald(100%)");
        assert_eq!(result, "100%");
    }
    
    #[test]
    fn test_unknown_color() {
        let parser = ColorTemplateParser::new(false);
        let result = parser.process("%c:unknowncolor(text)");
        assert_eq!(result, "%c:unknowncolor(text)"); // Should remain unchanged
    }
    
    #[test]
    fn test_unbalanced_parentheses() {
        let parser = ColorTemplateParser::new(false);
        let result = parser.process("%c:red(unbalanced");
        assert_eq!(result, "%c:red(unbalanced"); // Should remain unchanged
    }
    
    #[test]
    fn test_no_nesting() {
        let parser = ColorTemplateParser::new(true);
        // Inner template should be treated as literal text
        let result = parser.process("%c:red(text %c:blue(inner))");
        assert_eq!(result, "text %c:blue(inner)");
    }
}