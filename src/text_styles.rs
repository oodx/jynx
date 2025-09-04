// Text Style Support - Italic, Underline, and Combined Formatting
// Extends color highlighting with rich typography for semantic emphasis

use std::collections::HashMap;

// ANSI escape codes for text formatting
pub const ITALIC: &str = "\x1B[3m";
pub const UNDERLINE: &str = "\x1B[4m";
pub const BOLD: &str = "\x1B[1m";
pub const DIM: &str = "\x1B[2m";
pub const STRIKETHROUGH: &str = "\x1B[9m";
pub const RESET: &str = "\x1B[0m";

// Combined style definitions
pub struct TextStyle {
    pub color: Option<String>,
    pub italic: bool,
    pub underline: bool,
    pub bold: bool,
    pub dim: bool,
    pub strikethrough: bool,
}

impl TextStyle {
    pub fn new() -> Self {
        TextStyle {
            color: None,
            italic: false,
            underline: false,
            bold: false,
            dim: false,
            strikethrough: false,
        }
    }
    
    pub fn color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
    
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
    
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }
    
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }
    
    // Generate ANSI escape sequence for this style
    pub fn to_ansi(&self) -> String {
        let mut codes = Vec::new();
        
        if self.bold {
            codes.push("1".to_string());
        }
        if self.dim {
            codes.push("2".to_string());
        }
        if self.italic {
            codes.push("3".to_string());
        }
        if self.underline {
            codes.push("4".to_string());
        }
        if self.strikethrough {
            codes.push("9".to_string());
        }
        
        // Add color code if specified
        if let Some(color) = &self.color {
            if let Some(color_code) = get_color_code(color) {
                codes.push(format!("38;5;{}", color_code));
            }
        }
        
        if codes.is_empty() {
            String::new()
        } else {
            format!("\x1B[{}m", codes.join(";"))
        }
    }
}

// Enhanced configuration for YAML themes with style support
#[derive(Debug, serde::Deserialize)]
pub struct StyledColorConfig {
    // Traditional color-only configuration (backward compatible)
    #[serde(flatten)]
    pub colorize: Option<HashMap<String, Vec<String>>>,
    
    // Enhanced styling configuration
    pub styles: Option<HashMap<String, StyleDefinition>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct StyleDefinition {
    pub keywords: Vec<String>,
    pub color: Option<String>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub bold: Option<bool>,
    pub dim: Option<bool>,
    pub strikethrough: Option<bool>,
}

impl StyleDefinition {
    pub fn to_text_style(&self) -> TextStyle {
        TextStyle {
            color: self.color.clone(),
            italic: self.italic.unwrap_or(false),
            underline: self.underline.unwrap_or(false),
            bold: self.bold.unwrap_or(false),
            dim: self.dim.unwrap_or(false),
            strikethrough: self.strikethrough.unwrap_or(false),
        }
    }
}

// Semantic style presets for common use cases
pub fn get_semantic_styles() -> HashMap<&'static str, TextStyle> {
    let mut styles = HashMap::new();
    
    // Documentation styles
    styles.insert("emphasis", TextStyle::new().italic().color("amber".to_string()));
    styles.insert("strong", TextStyle::new().bold().color("crimson".to_string()));
    styles.insert("code", TextStyle::new().color("azure".to_string()));
    styles.insert("link", TextStyle::new().underline().color("royal".to_string()));
    styles.insert("quote", TextStyle::new().italic().dim().color("slate".to_string()));
    
    // Status styles
    styles.insert("deprecated", TextStyle::new().strikethrough().dim().color("rust".to_string()));
    styles.insert("new_feature", TextStyle::new().bold().underline().color("emerald".to_string()));
    styles.insert("breaking_change", TextStyle::new().bold().color("crimson".to_string()));
    styles.insert("experimental", TextStyle::new().italic().color("orchid".to_string()));
    
    // Priority styles
    styles.insert("urgent", TextStyle::new().bold().underline().color("crimson".to_string()));
    styles.insert("important", TextStyle::new().bold().color("amber".to_string()));
    styles.insert("note", TextStyle::new().italic().color("azure".to_string()));
    styles.insert("tip", TextStyle::new().color("mint".to_string()));
    
    // Technical styles
    styles.insert("variable", TextStyle::new().italic().color("turquoise".to_string()));
    styles.insert("function", TextStyle::new().color("violet".to_string()));
    styles.insert("class", TextStyle::new().bold().color("royal".to_string()));
    styles.insert("keyword", TextStyle::new().bold().color("magenta".to_string()));
    styles.insert("string", TextStyle::new().color("forest".to_string()));
    styles.insert("comment", TextStyle::new().italic().dim().color("slate".to_string()));
    
    styles
}

// Helper function to get color code (references extended_colors.rs)
fn get_color_code(color: &str) -> Option<u8> {
    // This would reference the extended color palette
    match color {
        "crimson" => Some(196),
        "amber" => Some(220),
        "azure" => Some(33),
        "royal" => Some(21),
        "emerald" => Some(34),
        "orchid" => Some(170),
        "mint" => Some(121),
        "turquoise" => Some(45),
        "violet" => Some(129),
        "forest" => Some(22),
        "slate" => Some(244),
        "rust" => Some(166),
        _ => None,
    }
}

// Example YAML configuration with style support
pub const EXAMPLE_STYLED_THEME: &str = r#"
filters:
  todo:
    # Traditional colorize (backward compatible)
    colorize:
      red2: ["priority", "urgent", "critical"]
      green2: ["done", "completed", "finished"]
      yellow2: ["status", "assignee", "deadline"]
    
    # Enhanced styles (new feature)
    styles:
      urgent_items:
        keywords: ["URGENT", "CRITICAL", "ASAP"]
        color: "crimson"
        bold: true
        underline: true
      
      completed_items:
        keywords: ["DONE", "COMPLETE", "FINISHED"]
        color: "emerald"
        bold: true
        
      notes:
        keywords: ["NOTE", "INFO", "FYI"]
        color: "azure"
        italic: true
        
      deprecated:
        keywords: ["DEPRECATED", "OBSOLETE", "LEGACY"]
        color: "rust"
        strikethrough: true
        dim: true

  troubleshoot:
    styles:
      error_emphasis:
        keywords: ["ERROR", "FATAL", "CRASH"]
        color: "crimson"
        bold: true
        underline: true
        
      solution_highlight:
        keywords: ["SOLUTION", "FIX", "RESOLUTION"]
        color: "emerald"
        bold: true
        
      investigation:
        keywords: ["investigate", "analyze", "debug"]
        color: "azure"
        italic: true
        
      assumptions:
        keywords: ["assume", "probably", "might be"]
        color: "amber"
        italic: true
        dim: true

  documentation:
    styles:
      headings:
        keywords: ["OVERVIEW", "SUMMARY", "CONCLUSION"]
        color: "royal"
        bold: true
        underline: true
        
      emphasis:
        keywords: ["important", "critical", "essential"]
        color: "amber"
        italic: true
        
      code_references:
        keywords: ["function", "class", "method", "variable"]
        color: "turquoise"
        italic: true
        
      external_links:
        keywords: ["http://", "https://", "www."]
        color: "royal"
        underline: true
"#;

// Usage examples for boxy integration
pub fn generate_boxy_style_examples() {
    println!("=== Boxy Style Integration Examples ===\n");
    
    // Example 1: Documentation with mixed styles
    let doc_content = "OVERVIEW: This is an important feature\n\
                      NOTE: The function parse_args() handles validation\n\
                      DEPRECATED: Old method is obsolete\n\
                      See https://docs.example.com for details";
    
    println!("Documentation with mixed styles:");
    println!("{}\n", apply_documentation_styles(doc_content));
    
    // Example 2: TODO items with priority styling
    let todo_content = "task: Implement authentication\n\
                       URGENT: Fix security vulnerability\n\
                       DONE: Update documentation\n\
                       NOTE: Consider performance impact";
    
    println!("TODO items with priority styling:");
    println!("{}\n", apply_todo_styles(todo_content));
    
    // Example 3: Code with syntax highlighting
    let code_content = "function validateUser(username) {\n\
                       // Check if user exists\n\
                       const user = database.find(username);\n\
                       return user !== null;\n\
                       }";
    
    println!("Code with syntax highlighting:");
    println!("{}", apply_code_styles(code_content));
}

// Example style application functions
fn apply_documentation_styles(content: &str) -> String {
    let styles = get_semantic_styles();
    let mut result = content.to_string();
    
    // Apply heading style to OVERVIEW
    if let Some(style) = styles.get("strong") {
        result = result.replace("OVERVIEW:", &format!("{}OVERVIEW:{}", style.to_ansi(), RESET));
    }
    
    // Apply emphasis to "important"
    if let Some(style) = styles.get("emphasis") {
        result = result.replace("important", &format!("{}important{}", style.to_ansi(), RESET));
    }
    
    // Apply code style to function names
    if let Some(style) = styles.get("code") {
        result = result.replace("parse_args()", &format!("{}parse_args(){}", style.to_ansi(), RESET));
    }
    
    // Apply deprecated style
    if let Some(style) = styles.get("deprecated") {
        result = result.replace("DEPRECATED:", &format!("{}DEPRECATED:{}", style.to_ansi(), RESET));
    }
    
    // Apply link style to URLs
    if let Some(style) = styles.get("link") {
        result = result.replace("https://docs.example.com", 
                               &format!("{}https://docs.example.com{}", style.to_ansi(), RESET));
    }
    
    result
}

fn apply_todo_styles(content: &str) -> String {
    let styles = get_semantic_styles();
    let mut result = content.to_string();
    
    // Apply urgent style
    if let Some(style) = styles.get("urgent") {
        result = result.replace("URGENT:", &format!("{}URGENT:{}", style.to_ansi(), RESET));
    }
    
    // Apply note style
    if let Some(style) = styles.get("note") {
        result = result.replace("NOTE:", &format!("{}NOTE:{}", style.to_ansi(), RESET));
    }
    
    result
}

fn apply_code_styles(content: &str) -> String {
    let styles = get_semantic_styles();
    let mut result = content.to_string();
    
    // Apply function style
    if let Some(style) = styles.get("function") {
        result = result.replace("function", &format!("{}function{}", style.to_ansi(), RESET));
    }
    
    // Apply comment style
    if let Some(style) = styles.get("comment") {
        result = result.replace("// Check if user exists", 
                               &format!("{}// Check if user exists{}", style.to_ansi(), RESET));
    }
    
    result
}