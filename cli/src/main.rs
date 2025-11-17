use clap::{Parser, Subcommand};
use ratatui::{Terminal, backend::CrosstermBackend};
use slides_core::{parser::parse_slides_with_meta, term::Terminal as SlideTerminal, theme::ThemeColors};
use slides_tui::App;
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
    },
}

fn main() {
    let cli = ArgParser::parse();

    tracing_subscriber::fmt().with_max_level(cli.log_level).init();

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

        Commands::Check { file, strict } => {
            tracing::info!("Checking slides: {}", file.display());
            if strict {
                tracing::debug!("Strict mode enabled");
            }
            eprintln!("Check command not yet implemented");
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

    let theme_name = theme_arg.unwrap_or_else(|| meta.theme.clone());
    tracing::debug!("Using theme: {}", theme_name);

    let theme = ThemeColors::default();

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

    // TODO: Load theme from theme registry based on theme_name
    let theme = ThemeColors::default();

    slides_core::printer::print_slides_to_stdout(&slides, &theme, width)?;

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
            Commands::Check { file, strict } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert!(strict);
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
}
