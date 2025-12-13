//! Command-Line Interface (CLI) module for the jsavrs compiler.
//!
//! This module defines the command-line argument structure and custom styling
//! for the jsavrs compiler binary. It uses the `clap` crate for argument parsing
//! with enhanced terminal output styling.
//!
//! # Key Features
//!
//! - Custom ANSI color schemes for help output
//! - File path validation ensuring `.vn` extension
//! - Verbose mode flag for detailed compilation output
//!
//! # Examples
//!
//! ```ignore
//! use jsavrs::cli::Args;
//! use clap::Parser;
//!
//! let args = Args::parse();
//! println!("Input file: {:?}", args.input);
//! ```

// src/cli.rs
use clap::{
    Parser, ValueHint,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};
use std::path::PathBuf;

/// Custom help template string for clap CLI output.
///
/// Defines the layout and formatting of the help message displayed
/// when users run `jsavrs --help`.
const HELP_STR: &str = r"
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}";

/// Creates custom ANSI color styles for CLI help output.
///
/// Defines color schemes and text effects for different elements of the
/// command-line interface, making the help text more readable and visually
/// appealing in terminal environments.
///
/// # Returns
///
/// A `Styles` instance configured with:
/// - **Header**: Bright cyan with bold effect
/// - **Literal**: Bright magenta with bold effect  
/// - **Error**: Bright red with bold effect
/// - **Valid**: Bright green with bold effect
/// - **Invalid**: Bright yellow with bold and underline effects
/// - **Placeholder**: Bright blue
/// - **Usage**: Bright cyan with bold and underline effects
///
/// # Examples
///
/// ```ignore
/// use jsavrs::cli::custom_styles;
///
/// let styles = custom_styles();
/// // Use with clap Parser derive macro
/// ```
pub fn custom_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightCyan.on_default() | Effects::BOLD)
        .literal(AnsiColor::BrightMagenta.on_default() | Effects::BOLD)
        .error(AnsiColor::BrightRed.on_default() | Effects::BOLD)
        .valid(AnsiColor::BrightGreen.on_default() | Effects::BOLD)
        .invalid(AnsiColor::BrightYellow.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .placeholder(AnsiColor::BrightBlue.on_default())
        .usage(AnsiColor::BrightCyan.on_default() | Effects::BOLD | Effects::UNDERLINE)
}

/// Validates and parses file paths to ensure `.vn` extension.
///
/// This custom parser function enforces that input files have the `.vn`
/// extension (case-insensitive), which is required for jsavrs source files.
///
/// # Arguments
///
/// * `s` - The file path string to validate
///
/// # Returns
///
/// * `Ok(PathBuf)` - If the path has a `.vn` extension
/// * `Err(String)` - If the path lacks the required `.vn` extension
///
/// # Examples
///
/// ```ignore
/// assert!(parse_vn_file("program.vn").is_ok());
/// assert!(parse_vn_file("program.VN").is_ok());  // case-insensitive
/// assert!(parse_vn_file("program.txt").is_err());
/// ```
fn parse_vn_file(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    let is_vn = p.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("vn")).unwrap_or(false);
    if is_vn { Ok(p) } else { Err("expected a path to a .vn file".into()) }
}

/// Command-line arguments structure for the jsavrs compiler.
///
/// This struct defines all command-line options accepted by the jsavrs binary,
/// using the `clap` derive API for automatic parsing and validation.
///
/// # Fields
///
/// * `input` - Path to the input `.vn` source file (required)
/// * `verbose` - Flag to enable verbose compilation output (optional)
///
/// # Examples
///
/// ```ignore
/// use jsavrs::cli::Args;
/// use clap::Parser;
///
/// // Parse from command line
/// let args = Args::parse();
///
/// // Parse from custom args
/// let args = Args::try_parse_from(["jsavrs", "-i", "program.vn", "-v"]);
/// ```
#[derive(Parser, Debug)]
#[command(
    version = clap::crate_version!(),
    author = clap::crate_authors!("\n"),
    about,
    long_about = None,
    help_template = HELP_STR,
    styles = custom_styles()
)]
pub struct Args {
    /// Input file for compilation (a .vn file is required)
    #[arg(
        short,
        long,
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        value_parser = parse_vn_file
    )]
    pub input: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
