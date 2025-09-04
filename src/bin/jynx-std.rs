#!/usr/bin/env rust
//
// jynx-std - Standard Rust Implementation
// Intelligent syntax highlighter with auto-detection and theme system
//

use jynx::std::*;
use jynx::std::theme::Theme;
use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(name = "jynx")]
#[command(about = "Intelligent syntax highlighter with auto-detection and theme management")]
#[command(version)]
struct Cli {
    /// Theme name or path to load
    #[arg(short, long)]
    theme: Option<String>,
    
    /// Filter to apply from theme
    #[arg(short, long)]
    filter: Option<String>,
    
    /// Fixed width for output formatting
    #[arg(short, long)]
    width: Option<usize>,
    
    /// Text alignment: left, center, right
    #[arg(short, long, default_value = "left")]
    align: String,
    
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
    
    /// Disable colorization (passthrough mode)
    #[arg(long)]
    no_color: bool,
    
    /// Commands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Theme management commands
    Theme {
        #[command(subcommand)]
        action: ThemeAction,
    },
}

#[derive(Subcommand)]
enum ThemeAction {
    /// List available themes
    List,
    /// Create new theme in current directory
    Create { name: String },
    /// Import theme from current directory to XDG+
    Import { name: String },
    /// Export theme from XDG+ to current directory
    Export { name: String },
    /// Edit theme in $EDITOR
    Edit { name: String },
}

fn main() {
    let cli = Cli::parse();
    
    // Handle subcommands
    if let Some(Commands::Theme { action }) = &cli.command {
        match handle_theme_command(action) {
            Ok(()) => return,
            Err(e) => {
                eprintln!("Theme command error: {}", e);
                process::exit(1);
            }
        }
    }
    
    // Load theme using smart resolution
    let theme = match Theme::load_theme(cli.theme.as_deref()) {
        Ok(theme) => {
            if cli.debug {
                eprintln!("Loaded theme: {} v{}", theme.metadata.name, theme.metadata.version);
            }
            Some(theme)
        },
        Err(e) => {
            if cli.theme.is_some() {
                eprintln!("Warning: {}", e);
                eprintln!("Falling back to auto-detection only");
            }
            None
        }
    };
    
    let app = JynxApp::with_theme_and_options(theme, cli.filter, cli.width, cli.align, cli.no_color);
    
    // Graceful error handling - if anything fails, we become 'cat'
    if let Err(e) = app.run() {
        eprintln!("jynx error: {}", e);
        process::exit(1);
    }
}

fn handle_theme_command(action: &ThemeAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ThemeAction::List => {
            let themes = Theme::list_themes()?;
            if themes.is_empty() {
                println!("No themes found");
            } else {
                println!("Available themes:");
                for (name, path, source) in themes {
                    println!("  {} ({}) - {}", name, source, path.display());
                }
            }
        },
        ThemeAction::Create { name } => {
            let path = Theme::create_theme(name)?;
            println!("Created theme '{}' at {}", name, path.display());
        },
        ThemeAction::Import { name } => {
            let path = Theme::import_theme(name)?;
            println!("Imported theme '{}' to {}", name, path.display());
        },
        ThemeAction::Export { name } => {
            let path = Theme::export_theme(name)?;
            println!("Exported theme '{}' to {}", name, path.display());
        },
        ThemeAction::Edit { name } => {
            Theme::edit_theme(name)?;
            println!("Edited theme '{}'", name);
        },
    }
    Ok(())
}