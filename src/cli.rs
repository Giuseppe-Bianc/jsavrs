// src/cli.rs
use clap::{
    Parser, ValueHint,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};
use std::path::PathBuf;

const HELP_STR: &str = r#"
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"#;

// Custom styles for clap
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

// Custom parser to enforce .vn extension
fn parse_vn_file(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    let is_vn = p.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("vn")).unwrap_or(false);
    if is_vn {
        Ok(p)
    } else {
        Err("expected a path to a .vn file".into())
    }
}

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
