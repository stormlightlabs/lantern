use clap::{Parser, Subcommand};
use std::path::PathBuf;
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
            tracing::info!("Presenting slides from: {}", file.display());
            if let Some(theme) = theme {
                tracing::debug!("Using theme: {}", theme);
            }
            eprintln!("TUI presentation mode not yet implemented");
        }

        Commands::Print { file, width, theme } => {
            tracing::info!("Printing slides from: {} (width: {})", file.display(), width);
            if let Some(theme) = theme {
                tracing::debug!("Using theme: {}", theme);
            }
            eprintln!("Print mode not yet implemented");
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
}
