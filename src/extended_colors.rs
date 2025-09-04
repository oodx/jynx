// Extended Color Palette - Rich variation for semantic highlighting
// Builds on ref_colors.rs with expanded palette for diverse use cases

pub fn get_extended_color_code(color: &str) -> &'static str {
    match color {
        // === CORE PALETTE (from ref_colors.rs) ===
        "red" => "\x1B[38;5;9m",
        "red2" => "\x1B[38;5;197m", 
        "deep" => "\x1B[38;5;61m",
        "deep_green" => "\x1B[38;5;60m",
        "orange" => "\x1B[38;5;214m",
        "yellow" => "\x1B[33m",
        "green" => "\x1B[38;5;10m",
        "green2" => "\x1B[32m",
        "blue" => "\x1B[36m",
        "blue2" => "\x1B[38;5;39m",
        "cyan" => "\x1B[38;5;14m",
        "magenta" => "\x1B[35m",
        "purple" => "\x1B[38;5;213m",
        "purple2" => "\x1B[38;5;141m",
        "white" => "\x1B[38;5;247m",
        "white2" => "\x1B[38;5;15m",
        "grey" => "\x1B[38;5;242m",
        "grey2" => "\x1B[38;5;240m",
        "grey3" => "\x1B[38;5;237m",
        
        // === EXTENDED RED SPECTRUM ===
        "crimson" => "\x1B[38;5;196m",        // Pure red - critical alerts
        "ruby" => "\x1B[38;5;160m",           // Dark red - errors
        "coral" => "\x1B[38;5;203m",          // Red-orange - warnings
        "salmon" => "\x1B[38;5;209m",         // Light red-orange - notices
        "rose" => "\x1B[38;5;217m",           // Pink-red - highlights
        "brick" => "\x1B[38;5;124m",          // Dark brick red - severe
        
        // === EXTENDED ORANGE SPECTRUM ===
        "amber" => "\x1B[38;5;220m",          // Golden orange - attention
        "tangerine" => "\x1B[38;5;208m",      // Bright orange - active
        "peach" => "\x1B[38;5;216m",          // Light orange - soft alerts
        "rust" => "\x1B[38;5;166m",           // Dark orange - deprecation
        "bronze" => "\x1B[38;5;130m",         // Brown-orange - legacy
        "gold" => "\x1B[38;5;178m",           // Golden - achievements
        
        // === EXTENDED YELLOW SPECTRUM ===
        "lemon" => "\x1B[38;5;226m",          // Bright yellow - warnings
        "mustard" => "\x1B[38;5;184m",        // Muted yellow - caution  
        "sand" => "\x1B[38;5;223m",           // Beige-yellow - neutral
        "cream" => "\x1B[38;5;230m",          // Light yellow - info
        "khaki" => "\x1B[38;5;143m",          // Olive-yellow - pending
        
        // === EXTENDED GREEN SPECTRUM ===
        "lime" => "\x1B[38;5;46m",            // Bright green - success
        "emerald" => "\x1B[38;5;34m",         // Pure green - completed
        "forest" => "\x1B[38;5;22m",          // Dark green - stable
        "mint" => "\x1B[38;5;121m",           // Light green - fresh
        "sage" => "\x1B[38;5;108m",           // Muted green - accepted
        "jade" => "\x1B[38;5;35m",            // Blue-green - verified
        "olive" => "\x1B[38;5;58m",           // Brown-green - archived
        
        // === EXTENDED BLUE SPECTRUM ===
        "azure" => "\x1B[38;5;33m",           // Sky blue - information
        "navy" => "\x1B[38;5;17m",            // Dark blue - system
        "royal" => "\x1B[38;5;21m",           // Royal blue - primary
        "ice" => "\x1B[38;5;159m",            // Light blue - secondary
        "steel" => "\x1B[38;5;67m",           // Grey-blue - infrastructure
        "teal" => "\x1B[38;5;30m",            // Blue-green - data
        "indigo" => "\x1B[38;5;54m",          // Deep blue - configuration
        
        // === EXTENDED PURPLE SPECTRUM ===
        "violet" => "\x1B[38;5;129m",         // Blue-purple - special
        "plum" => "\x1B[38;5;96m",            // Dark purple - reserved
        "lavender" => "\x1B[38;5;183m",       // Light purple - optional
        "orchid" => "\x1B[38;5;170m",         // Pink-purple - enhanced
        "mauve" => "\x1B[38;5;139m",          // Muted purple - metadata
        "amethyst" => "\x1B[38;5;98m",        // Deep purple - advanced
        
        // === EXTENDED CYAN SPECTRUM ===
        "aqua" => "\x1B[38;5;51m",            // Bright cyan - active data
        "turquoise" => "\x1B[38;5;45m",       // Blue-cyan - processing
        "sky" => "\x1B[38;5;117m",            // Light cyan - status
        "ocean" => "\x1B[38;5;31m",           // Deep cyan - persistence
        
        // === MONOCHROME SPECTRUM ===
        "black" => "\x1B[38;5;16m",           // Pure black - disabled
        "charcoal" => "\x1B[38;5;235m",       // Dark grey - inactive
        "slate" => "\x1B[38;5;244m",          // Medium grey - secondary
        "silver" => "\x1B[38;5;250m",         // Light grey - tertiary
        "pearl" => "\x1B[38;5;253m",          // Very light grey - background
        "snow" => "\x1B[38;5;255m",           // Pure white - emphasis
        
        // === SEMANTIC GROUPINGS ===
        
        // Error/Alert semantic colors
        "error" => "\x1B[38;5;196m",          // Critical error
        "warning" => "\x1B[38;5;220m",        // Warning state
        "danger" => "\x1B[38;5;160m",         // Dangerous operation
        "alert" => "\x1B[38;5;208m",          // Alert state
        
        // Success/Positive semantic colors
        "success" => "\x1B[38;5;46m",         // Success state
        "complete" => "\x1B[38;5;34m",        // Completion
        "verified" => "\x1B[38;5;35m",        // Verification
        "approved" => "\x1B[38;5;121m",       // Approval
        
        // Info/Neutral semantic colors  
        "info" => "\x1B[38;5;33m",            // Information
        "note" => "\x1B[38;5;159m",           // Note/annotation
        "hint" => "\x1B[38;5;117m",           // Hint/tip
        "debug" => "\x1B[38;5;67m",           // Debug information
        
        // Process/State semantic colors
        "pending" => "\x1B[38;5;184m",        // Pending state
        "progress" => "\x1B[38;5;214m",       // In progress
        "blocked" => "\x1B[38;5;197m",        // Blocked state
        "queued" => "\x1B[38;5;143m",         // Queued state
        "active" => "\x1B[38;5;51m",          // Active state
        "inactive" => "\x1B[38;5;240m",       // Inactive state
        
        // Extended BashFX stderr semantic colors (debugging personality)
        "silly" => "\x1B[38;5;201m",          // Bright magenta - ridiculous debugging/invalid conditions
        "magic" => "\x1B[38;5;93m",           // Purple variation - "how did this even work?" moments
        "trace" => "\x1B[38;5;242m",          // Medium grey - tracing state progression/function output
        "think" => "\x1B[38;5;15m",           // Bright white - tracing function calls only
        
        // Priority semantic colors
        "critical" => "\x1B[38;5;196m",       // Critical priority
        "high" => "\x1B[38;5;208m",           // High priority
        "medium" => "\x1B[38;5;220m",         // Medium priority  
        "low" => "\x1B[38;5;250m",            // Low priority
        "trivial" => "\x1B[38;5;237m",        // Trivial priority
        
        // === ADVANCED VARIATIONS ===
        
        // Bright variants (high contrast)
        "bright_red" => "\x1B[38;5;9m",
        "bright_green" => "\x1B[38;5;10m",
        "bright_yellow" => "\x1B[38;5;11m",
        "bright_blue" => "\x1B[38;5;12m",
        "bright_magenta" => "\x1B[38;5;13m",
        "bright_cyan" => "\x1B[38;5;14m",
        
        // Dim variants (low contrast)
        "dim_red" => "\x1B[38;5;52m",
        "dim_green" => "\x1B[38;5;22m",
        "dim_yellow" => "\x1B[38;5;58m",
        "dim_blue" => "\x1B[38;5;17m",
        "dim_magenta" => "\x1B[38;5;54m",
        "dim_cyan" => "\x1B[38;5;23m",
        
        // Pastel variants (soft colors)
        "pastel_red" => "\x1B[38;5;217m",
        "pastel_green" => "\x1B[38;5;157m",
        "pastel_yellow" => "\x1B[38;5;230m",
        "pastel_blue" => "\x1B[38;5;159m",
        "pastel_purple" => "\x1B[38;5;183m",
        "pastel_orange" => "\x1B[38;5;223m",
        
        // Default fallback
        _ => "",
    }
}

// Helper function to get color categories for theme generation
pub fn get_color_categories() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("red_spectrum", vec!["red", "red2", "crimson", "ruby", "coral", "salmon", "rose", "brick"]),
        ("orange_spectrum", vec!["orange", "amber", "tangerine", "peach", "rust", "bronze", "gold"]),
        ("yellow_spectrum", vec!["yellow", "lemon", "mustard", "sand", "cream", "khaki"]),
        ("green_spectrum", vec!["green", "green2", "lime", "emerald", "forest", "mint", "sage", "jade", "olive"]),
        ("blue_spectrum", vec!["blue", "blue2", "azure", "navy", "royal", "ice", "steel", "teal", "indigo"]),
        ("purple_spectrum", vec!["purple", "purple2", "violet", "plum", "lavender", "orchid", "mauve", "amethyst"]),
        ("cyan_spectrum", vec!["cyan", "aqua", "turquoise", "sky", "ocean"]),
        ("monochrome", vec!["black", "charcoal", "slate", "grey", "grey2", "grey3", "silver", "pearl", "white", "white2", "snow"]),
        ("semantic_alerts", vec!["error", "warning", "danger", "alert"]),
        ("semantic_success", vec!["success", "complete", "verified", "approved"]),
        ("semantic_info", vec!["info", "note", "hint", "debug"]),
        ("semantic_states", vec!["pending", "progress", "blocked", "queued", "active", "inactive"]),
        ("semantic_debug", vec!["silly", "magic", "trace", "think"]),
        ("semantic_priority", vec!["critical", "high", "medium", "low", "trivial"]),
    ]
}

// Usage example for KB theme generation
pub fn generate_kb_theme_suggestions() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        // High contrast combinations for critical information
        ("error_theme", vec!["crimson", "ruby", "coral"]),
        ("success_theme", vec!["emerald", "lime", "mint"]),
        ("warning_theme", vec!["amber", "lemon", "gold"]),
        
        // Balanced combinations for general use
        ("balanced_warm", vec!["coral", "amber", "sage"]),
        ("balanced_cool", vec!["azure", "mint", "lavender"]),
        ("balanced_neutral", vec!["slate", "sand", "ice"]),
        
        // Specialized combinations for different domains
        ("technical_theme", vec!["steel", "emerald", "amber"]),
        ("creative_theme", vec!["orchid", "turquoise", "gold"]),
        ("minimal_theme", vec!["charcoal", "silver", "azure"]),
        
        // Accessibility-optimized combinations
        ("high_contrast", vec!["snow", "crimson", "emerald"]),
        ("colorblind_safe", vec!["azure", "amber", "charcoal"]),
    ]
}