# Logging

Lantern uses the `tracing` framework for internal logging and diagnostics. By default, logging is disabled, but can be enabled via environment variables for debugging and troubleshooting.

## Configuration

### File Path

To enable logging to a file, set the `LANTERN_LOG_FILE` environment variable:

```bash
export LANTERN_LOG_FILE=/path/to/lantern.log
lantern present slides.md
```

If `LANTERN_LOG_FILE` is not set, logs are discarded and won't appear anywhere.

### Level

Control the verbosity of logs using the `--log-level` flag:

```bash
LANTERN_LOG_FILE=debug.log lantern --log-level debug present slides.md
```

## Usage Examples

### Basic Debugging

Enable info-level logging for general troubleshooting:

```bash
LANTERN_LOG_FILE=lantern.log lantern present slides.md
```

### Detailed Diagnostics

Enable trace-level logging for in-depth debugging:

```bash
LANTERN_LOG_FILE=lantern-trace.log lantern --log-level trace present slides.md
```

### Temporary Log File

Use a temporary file that gets cleaned up automatically:

```bash
LANTERN_LOG_FILE=/tmp/lantern-$$.log lantern present slides.md
```

## Log Format

Logs are written in plain text format without ANSI color codes, making them easy to read and process with standard tools:

```sh
2025-11-18T10:30:45.123Z INFO lantern_cli: Presenting slides from: slides.md
2025-11-18T10:30:45.234Z INFO lantern_cli: Theme selection: CLI arg=None, frontmatter=oxocarbon-dark, final=oxocarbon-dark
2025-11-18T10:30:45.345Z DEBUG lantern_core::parser: Parsed 15 slides from markdown
```
