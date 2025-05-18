// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::Parser;
use clap::builder::{
    PathBufValueParser, Styles,
    styling::{AnsiColor, Effects},
};
use console::style;
use jsavrs::error::compile_error::CompileError;
use jsavrs::lexer::lexer_tokenize_with_errors;
use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

const HELP_STR: &str = r#"
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"#;


#[derive(Parser, Debug)]
#[command(
    version = clap::crate_version!(),
    author = clap::crate_authors!("\n"),
    about,
    long_about = None,
    help_template = HELP_STR,
    styles = custom_styles()
)]
struct Args {
    /// Input file to compile (required .vn)
    #[arg(
        short,
        long,
        value_parser = PathBufValueParser::new()
    )]
    input: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn custom_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightCyan.on_default() | Effects::BOLD)
        .literal(AnsiColor::BrightMagenta.on_default() | Effects::BOLD)
        .error(AnsiColor::BrightRed.on_default() | Effects::BOLD)
        .valid(AnsiColor::BrightGreen.on_default() | Effects::BOLD)
        .invalid(AnsiColor::BrightYellow.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .placeholder(AnsiColor::BrightBlue.on_default())
        .usage(AnsiColor::BrightCyan.on_default() | Effects::BOLD | Effects::UNDERLINE)
}

fn main() -> Result<(), CompileError> {
    let args = Args::parse();
    let file_path: &Path = args.input.as_path();

    // Read input file with error styling
    let input = fs::read_to_string(file_path).map_err(|e| {
        eprintln!(
            "{} {}: {}",
            style("ERROR:").red().bold(),
            style("I/O").red(),
            style(format!("{}", e)).yellow() // Use formatted string instead of moving `e`
        );
        e
    })?;

    let (tokens, errors) = lexer_tokenize_with_errors(
        &input,
        file_path.to_str().ok_or_else(|| {
            CompileError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid file path",
            ))
        })?,
    );

    if !errors.is_empty() {
        // Print styled error messages
        for error in errors {
            match error {
                CompileError::LexerError { message, span } => {
                    eprintln!(
                        "{} {}: {}\n{} {}",
                        style("ERROR:").red().bold(),
                        style("LEX").red(),
                        style(message).yellow(),
                        style("Location:").blue(),
                        style(span).cyan()
                    );
                }
                CompileError::IoError(e) => {
                    eprintln!(
                        "{} {}: {}",
                        style("ERROR:").red().bold(),
                        style("I/O").red(),
                        style(e).yellow()
                    );
                }
            }
        }
        process::exit(1);
    }

    // Print tokens with color if verbose
    if args.verbose {
        for token in tokens {
            println!(
                "{} {}",
                style(format!("{:?}", token.kind)).green(),
                style(format!("at {}", token.span)).dim()
            );
        }
    } else {
        println!("{} tokens found", tokens.len());
    }

    Ok(())
}
