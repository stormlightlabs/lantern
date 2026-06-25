//! Command-line entry point for the `lantern` binary.

/// TODO: Add --no-bg flag to present command to allow users to disable background color
use clap::{Parser, Subcommand};
use lantern_cli::validator::{validate_slides, validate_theme_file};
use lantern_cli::{
    metadata::Meta,
    parser::parse_slides_with_meta,
    slide::Slide,
    term::Terminal as SlideTerminal,
    theme::{ThemeColors, ThemeRegistry},
    ui::{App, PresentationUpdate},
};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use owo_colors::OwoColorize;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
};
use tracing::Level;

/// A modern terminal-based presentation tool
#[derive(Parser, Debug)]
#[command(name = "lantern")]
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
        /// Path to a Base16 YAML theme file
        #[arg(long)]
        theme_file: Option<PathBuf>,
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
        /// Path to a Base16 YAML theme file
        #[arg(long)]
        theme_file: Option<PathBuf>,
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

struct LoadedDeck {
    meta: Meta,
    slides: Vec<Slide>,
    theme: ThemeColors,
    theme_name: String,
    filename: String,
}

fn main() {
    let cli = ArgParser::parse();

    if let Ok(log_path) = std::env::var("LANTERN_LOG_FILE") {
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)
            .unwrap_or_else(|e| panic!("Failed to create log file at {log_path}: {e}"));

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
        Commands::Present { file, theme, theme_file } => {
            if let Err(e) = run_present(&file, theme, theme_file.as_deref()) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Commands::Print { file, width, theme, theme_file } => {
            if let Err(e) = run_print(&file, width, theme, theme_file.as_deref()) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Commands::Init { path, name } => {
            tracing::info!("Initializing new deck: {} in {}", name, path.display());
            eprintln!("Init command not yet implemented");
        }
        Commands::Check { file, strict, theme } => {
            if let Err(e) = run_check(&file, strict, theme) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn load_theme(theme_name: &str, theme_file: Option<&Path>) -> io::Result<(ThemeColors, String)> {
    if let Some(path) = theme_file {
        let theme = ThemeRegistry::load_file(path)?;
        return Ok((theme, path.display().to_string()));
    }

    Ok((ThemeRegistry::load_named(theme_name)?, theme_name.to_string()))
}

fn load_deck(file: &Path, theme_arg: Option<&str>, theme_file: Option<&Path>) -> io::Result<LoadedDeck> {
    let markdown = std::fs::read_to_string(file)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read file {}: {}", file.display(), e)))?;

    let (meta, slides) = parse_slides_with_meta(&markdown)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Parse error: {e}")))?;

    if slides.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No slides found in file"));
    }

    let theme_name = theme_arg.map(str::to_string).unwrap_or_else(|| meta.theme.clone());
    let (theme, theme_name) = load_theme(&theme_name, theme_file)?;
    let filename = file
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(LoadedDeck { meta, slides, theme, theme_name, filename })
}

fn watch_deck(file: &Path) -> io::Result<(RecommendedWatcher, Receiver<()>)> {
    let source = file.canonicalize()?;
    let watched_name = source.file_name().map(|name| name.to_owned()).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid source file path {}", file.display()),
        )
    })?;
    let parent = source.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid source file path {}", file.display()),
        )
    })?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |result: notify::Result<Event>| match result {
        Ok(event) => {
            if is_deck_reload_event(&event)
                && event
                    .paths
                    .iter()
                    .any(|path| path.file_name() == Some(watched_name.as_os_str()))
            {
                let _ = tx.send(());
            }
        }
        Err(error) => tracing::warn!("Slide deck watcher error: {error}"),
    })
    .map_err(io::Error::other)?;

    watcher
        .watch(parent, RecursiveMode::NonRecursive)
        .map_err(io::Error::other)?;
    Ok((watcher, rx))
}

fn is_deck_reload_event(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Any | EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
    )
}

fn reload_deck_if_changed(
    file: &Path, theme_arg: Option<&str>, theme_file: Option<&Path>, reload_events: &Receiver<()>,
) -> io::Result<Option<PresentationUpdate>> {
    if reload_events.try_iter().count() == 0 {
        return Ok(None);
    }

    match load_deck(file, theme_arg, theme_file) {
        Ok(deck) => {
            tracing::info!("Reloaded slides from: {}", file.display());
            Ok(Some(PresentationUpdate {
                slides: deck.slides,
                theme: deck.theme,
                theme_name: deck.theme_name,
            }))
        }
        Err(error) => {
            tracing::warn!("Keeping previous slides after reload failure: {error}");
            Ok(None)
        }
    }
}

fn run_present(file: &Path, theme_arg: Option<String>, theme_file: Option<&Path>) -> io::Result<()> {
    tracing::info!("Presenting slides from: {}", file.display());

    let deck = load_deck(file, theme_arg.as_deref(), theme_file)?;
    tracing::info!(
        "Theme selection: CLI arg={:?}, theme file={:?}, frontmatter={}, final={}",
        theme_arg,
        theme_file,
        deck.meta.theme,
        deck.theme_name
    );

    let (_watcher, reload_events) = watch_deck(file)?;
    let mut slide_terminal = SlideTerminal::setup()?;

    let result = (|| -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        let mut app = App::new(deck.slides, deck.theme, deck.filename, deck.meta, deck.theme_name);
        app.run(&mut terminal, || {
            reload_deck_if_changed(file, theme_arg.as_deref(), theme_file, &reload_events)
        })?;

        Ok(())
    })();

    slide_terminal.restore()?;

    result
}

fn run_check(file: &Path, strict: bool, is_theme: bool) -> io::Result<()> {
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

fn run_print(file: &PathBuf, width: usize, theme_arg: Option<String>, theme_file: Option<&Path>) -> io::Result<()> {
    tracing::info!("Printing slides from: {} (width: {})", file.display(), width);

    let markdown = std::fs::read_to_string(file)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read file {}: {}", file.display(), e)))?;

    let (meta, slides) = parse_slides_with_meta(&markdown)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Parse error: {e}")))?;

    if slides.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "No slides found in file"));
    }

    let theme_name = theme_arg.unwrap_or_else(|| meta.theme.clone());
    tracing::debug!("Using theme: {}", theme_name);

    let (theme, _) = load_theme(&theme_name, theme_file)?;

    lantern_cli::printer::print_slides_to_stdout(&slides, &theme, width)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_present_command() {
        let cli = ArgParser::parse_from(["lantern", "present", "test.md"]);
        match cli.command {
            Commands::Present { file, theme, theme_file } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert_eq!(theme, None);
                assert_eq!(theme_file, None);
            }
            _ => panic!("Expected Present command"),
        }
    }

    #[test]
    fn cli_present_with_theme() {
        let cli = ArgParser::parse_from(["lantern", "present", "test.md", "--theme", "dark"]);
        match cli.command {
            Commands::Present { file, theme, theme_file } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert_eq!(theme, Some("dark".to_string()));
                assert_eq!(theme_file, None);
            }
            _ => panic!("Expected Present command"),
        }
    }

    #[test]
    fn cli_present_with_theme_file() {
        let cli = ArgParser::parse_from(["lantern", "present", "test.md", "--theme-file", "theme.yml"]);
        match cli.command {
            Commands::Present { file, theme, theme_file } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert_eq!(theme, None);
                assert_eq!(theme_file, Some(PathBuf::from("theme.yml")));
            }
            _ => panic!("Expected Present command"),
        }
    }

    #[test]
    fn cli_print_command() {
        let cli = ArgParser::parse_from(["lantern", "print", "test.md", "-w", "100"]);
        match cli.command {
            Commands::Print { file, width, theme, theme_file } => {
                assert_eq!(file, PathBuf::from("test.md"));
                assert_eq!(width, 100);
                assert_eq!(theme, None);
                assert_eq!(theme_file, None);
            }
            _ => panic!("Expected Print command"),
        }
    }

    #[test]
    fn cli_init_command() {
        let cli = ArgParser::parse_from(["lantern", "init", "--name", "my-deck.md"]);
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
        let cli = ArgParser::parse_from(["lantern", "check", "test.md", "--strict"]);
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
        let cli = ArgParser::parse_from(["lantern", "check", "theme.yml", "--theme"]);
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

        let result = run_print(&test_file, 80, None, None);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_print_empty_file() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("empty_slides.md");

        std::fs::write(&test_file, "").expect("Failed to write test file");

        let result = run_print(&test_file, 80, None, None);
        assert!(result.is_err());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_print_nonexistent_file() {
        let test_file = PathBuf::from("/nonexistent/file.md");
        let result = run_print(&test_file, 80, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn run_print_with_theme_from_frontmatter() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_themed_slides.md");

        let content = "---\ntheme: nord\n---\n# Test Slide\n\nThis is a test paragraph.";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_print(&test_file, 80, None, None);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_print_with_theme_override() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_override_slides.md");

        let content = "---\ntheme: nord\n---\n# Test Slide\n\nThis is a test paragraph.";
        std::fs::write(&test_file, content).expect("Failed to write test file");

        let result = run_print(&test_file, 80, Some("catppuccin-mocha".to_string()), None);
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn run_print_with_theme_file() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test_theme_file_slides.md");
        let theme_file = temp_dir.join("test_theme_file.yml");

        let content = "# Test Slide\n\nThis is a test paragraph.";
        let theme = r###"
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
        std::fs::write(&theme_file, theme).expect("Failed to write theme file");

        let result = run_print(&test_file, 80, None, Some(&theme_file));
        assert!(result.is_ok());

        std::fs::remove_file(&test_file).ok();
        std::fs::remove_file(&theme_file).ok();
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
