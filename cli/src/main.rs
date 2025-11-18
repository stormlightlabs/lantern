/// TODO: Add --no-bg flag to present command to allow users to disable background color
use clap::{Parser, Subcommand};
use lantern_core::{parser::parse_slides_with_meta, term::Terminal as SlideTerminal, theme::ThemeRegistry};
use lantern_ui::App;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, path::PathBuf};
use tracing::Level;

/// A modern terminal-based presentation tool
#[derive(Parser, Debug)]
#[command(name = "slides")]
#[command(version, about, long_about = None)]
struct ArgParser {
    /// Set logging level (error, warn, info, debug, trace)
    #[arg(short, long, global = true, default_value = "info")]
    log_level: Level,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Present slides in interactive TUI mode
    Present {
        /// Path to the markdown file
        file: PathBuf,
        /// Theme to use for presentation
        #[arg(short, long)]
        theme: Option<String>,
    },

    /// Print slides to stdout with formatting
    Print {
        /// Path to the markdown file
        file: PathBuf,
        /// Maximum width for output (in characters)
        #[arg(short, long, default_value = "80")]
        width: usize,
        /// Theme to use for coloring
        #[arg(short, long)]
        theme: Option<String>,
    },

    /// Initialize a new slide deck with example content
    Init {
        /// Directory to create the deck in
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Name of the deck file
        #[arg(short, long, default_value = "slides.md")]
        name: String,
    },

    /// Check slides for errors and lint issues
    Check {
        /// Path to the markdown file
        file: PathBuf,
        /// Enable strict mode with additional checks
        #[arg(short, long)]
        strict: bool,
        /// Validate file as a theme instead of slides
        #[arg(short, long)]
        theme: bool,
    },
}

fn main() {
    let cli = ArgParser::parse();

    if let Ok(log_path) = std::env::var("LANTERN_LOG_FILE") {
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)
            .unwrap_or_else(|e| panic!("Failed to create log file at {}: {}", log_path, e));

        tracing_subscriber::fmt()
            .with_max_level(cli.log_level)
            .with_writer(std::sync::Mutex::new(log_file))
            .with_ansi(false)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(cli.log_level)
            .with_writer(std::io::sink)
            .with_ansi(false)
            .init();
    }

    match cli.command {
        Commands::Present { file, theme } => {
            if let Err(e) = run_present(&file, theme) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Print { file, width, theme } => {
            if let Err(e) = run_print(&file, width, theme) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Init { path, name } => {
            tracing::info!("Initializing new deck: {} in {}", name, path.display());
            eprintln!("Init command not yet implemented");
        }
        Commands::Check { file, strict, theme } => {
            if let Err(e) = run_check(&file, strict, theme) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn run_present(file: &PathBuf, theme_arg: Option<String>) -> io::Result<()> {
    tracing::info!("Presenting slides from: {}", file.display());

    let markdown = std::fs::read_to_string(file)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read file {}: {}", file.display(), e)))?;

    let (meta, slides) = parse_slides_with_meta(&markdown)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Parse error: {}", e)))?;

    if slides.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No slides found in file"));
    }

    let theme_name = theme_arg.clone().unwrap_or_else(|| meta.theme.clone());
    tracing::info!(
        "Theme selection: CLI arg={:?}, frontmatter={}, final={}",
        theme_arg,
        meta.theme,
        theme_name
    );

    let theme = ThemeRegistry::get(&theme_name);

    let filename = file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut slide_terminal = SlideTerminal::setup()?;

    let result = (|| -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        let mut app = App::new(slides, theme, filename, meta);
        app.run(&mut terminal)?;

        Ok(())
    })();

    slide_terminal.restore()?;

    result
}

fn run_check(file: &PathBuf, strict: bool, is_theme: bool) -> io::Result<()> {
    use lantern_core::validator::{validate_slides, validate_theme_file};
    use owo_colors::OwoColorize;

    if is_theme {
        tracing::info!("Validating theme file: {}", file.display());
        let result = validate_theme_file(file);

        if result.is_valid() {
            println!("{} Theme is valid", "✓".green().bold());
        } else {
            println!("{} Theme validation failed", "✗".red().bold());
        }

        for error in &result.errors {
            println!("  {} {}", "Error:".red().bold(), error);
        }

        for warning in &result.warnings {
            println!("  {} {}", "Warning:".yellow().bold(), warning);
        }

        if !result.is_valid() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Theme validation failed"));
        }
    } else {
        tracing::info!("Validating slides: {}", file.display());
        if strict {
            tracing::debug!("Strict mode enabled");
        }

        let result = validate_slides(file, strict);

        if result.is_valid() && !result.has_issues() {
            println!("{} Slides are valid", "✓".green().bold());
        } else if result.is_valid() {
            println!("{} Slides are valid (with warnings)", "✓".yellow().bold());
        } else {
            println!("{} Slide validation failed", "✗".red().bold());
        }

        for error in &result.errors {
            println!("  {} {}", "Error:".red().bold(), error);
        }

        for warning in &result.warnings {
            println!("  {} {}", "Warning:".yellow().bold(), warning);
        }

        if !result.is_valid() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Slide validation failed"));
        }
    }

    Ok(())
}

fn run_print(file: &PathBuf, width: usize, theme_arg: Option<String>) -> io::Result<()> {
    tracing::info!("Printing slides from: {} (width: {})", file.display(), width);

    let markdown = std::fs::read_to_string(file)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read file {}: {}", file.display(), e)))?;

    let (meta, slides) = parse_slides_with_meta(&markdown)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Parse error: {}", e)))?;

    if slides.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No slides found in file"));
    }

    let theme_name = theme_arg.unwrap_or_else(|| meta.theme.clone());
    tracing::debug!("Using theme: {}", theme_name);

    let theme = ThemeRegistry::get(&theme_name);

    lantern_core::printer::print_slides_to_stdout(&slides, &theme, width)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_present_command() {
        let cli = ArgParser::parse_from(["slides", "present", "test.md"]);
        match cli.command {
            Commands::Present { file, theme } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert_eq!(theme, None);
            }
            _ => panic!("Expected Present command"),
        }
    }

    #[test]
    fn cli_present_with_theme() {
        let cli = ArgParser::parse_from(["slides", "present", "test.md", "--theme", "dark"]);
        match cli.command {
            Commands::Present { file, theme } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert_eq!(theme, Some("dark".to_string()));
            }
            _ => panic!("Expected Present command"),
        }
    }

    #[test]
    fn cli_print_command() {
        let cli = ArgParser::parse_from(["slides", "print", "test.md", "-w", "100"]);
        match cli.command {
            Commands::Print { file, width, theme } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert_eq!(width, 100);
                assert_eq!(theme, None);
            }
            _ => panic!("Expected Print command"),
        }
    }

    #[test]
    fn cli_init_command() {
        let cli = ArgParser::parse_from(["slides", "init", "--name", "my-deck.md"]);
        match cli.command {
            Commands::Init { path, name } => {
                assert_eq!(path, PathBuf::from("."));
                assert_eq!(name, "my-deck.md");
            }
            _ => panic!("Expected Init command"),
        }
    }

    #[test]
    fn cli_check_command() {
        let cli = ArgParser::parse_from(["slides", "check", "test.md", "--strict"]);
        match cli.command {
            Commands::Check { file, strict, theme } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert!(strict);
                assert!(!theme);
            }
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn cli_check_theme_command() {
        let cli = ArgParser::parse_from(["slides", "check", "theme.yml", "--theme"]);
        match cli.command {
            Commands::Check { file, strict, theme } => {
                assert_eq!(file, PathBuf::from("theme.yml"));
                assert!(!strict);
                assert!(theme);
            }
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn run_print_with_test_file() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_slides.md");

        let content = "# Test Slide\n\nThis is a test paragraph.\n\n---\n\n# Second Slide\n\n- Item 1\n- Item 2";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_print(&test_file, 80, None);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_print_empty_file() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("empty_slides.md");

        std::fs::write(&test_file, "").expect("Failed to write test file");

        let result = run_print(&test_file, 80, None);
        assert!(result.is_err());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_print_nonexistent_file() {
        let test_file = PathBuf::from("/nonexistent/file.md");
        let result = run_print(&test_file, 80, None);
        assert!(result.is_err());
    }

    #[test]
    fn run_print_with_theme_from_frontmatter() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_themed_slides.md");

        let content = "---\ntheme: dark\n---\n# Test Slide\n\nThis is a test paragraph.";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_print(&test_file, 80, None);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_print_with_theme_override() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_override_slides.md");

        let content = "---\ntheme: light\n---\n# Test Slide\n\nThis is a test paragraph.";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_print(&test_file, 80, Some("monokai".to_string()));
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_check_valid_slides() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_check_valid.md");
        let content = "# Test Slide\n\nThis is a test paragraph.";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_check(&test_file, false, false);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_check_invalid_slides() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_check_invalid.md");
        let content = "";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_check(&test_file, false, false);
        assert!(result.is_err());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_check_nonexistent_file() {
        let test_file = PathBuf::from("/nonexistent/test_check.md");
        let result = run_check(&test_file, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn run_check_strict_mode() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_check_strict.md");
        let content = "---\ntheme: nonexistent-theme\n---\n# Slide 1\n\nContent";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_check(&test_file, true, false);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_check_valid_theme() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_check_valid_theme.yml");
        let content = r###"
system: "base16"
name: "Test Theme"
author: "Test Author"
variant: "dark"
palette:
  base00: "#000000"
  base01: "#111111"
  base02: "#222222"
  base03: "#333333"
  base04: "#444444"
  base05: "#555555"
  base06: "#666666"
  base07: "#777777"
  base08: "#888888"
  base09: "#999999"
  base0A: "#aaaaaa"
  base0B: "#bbbbbb"
  base0C: "#cccccc"
  base0D: "#dddddd"
  base0E: "#eeeeee"
  base0F: "#ffffff"
"###;
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_check(&test_file, false, true);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_check_invalid_theme() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_check_invalid_theme.yml");
        let content = "invalid: yaml: content: [unclosed";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_check(&test_file, false, true);
        assert!(result.is_err());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_check_invalid_frontmatter() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_check_bad_frontmatter.md");
        let content = "---\ninvalid yaml: [unclosed\n---\n# Slide";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_check(&test_file, false, false);
        assert!(result.is_err());

        std::fs::remove_file(&test_file).ok();
    }
}
