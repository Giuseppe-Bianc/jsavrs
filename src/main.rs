// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::{
    Parser,
    builder::{
        PathBufValueParser, Styles,
        styling::{AnsiColor, Effects},
    },
};
use console::style;
use jsavrs::{
    error::compile_error::CompileError, lexer::lexer_tokenize_with_errors,
    location::source_span::SourceSpan,
};
use std::{
    fs,
    path::{Path, PathBuf},
    //process,
};
use jsavrs::parser::ast::pretty_print;
use jsavrs::parser::jsav_parser::JsavParser;

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

#[allow(clippy::unused_unit)]
fn main() -> Result<(), CompileError> {
    let args = Args::parse();
    let file_path: &Path = args.input.as_path();

    // Read input file with error styling
    let input = fs::read_to_string(file_path).map_err(|e| {
        eprintln!(
            "{} {}: {}",
            style("ERROR:").red().bold(),
            style("I/O").red(),
            style(format!("{e}")).yellow()
        );
        e
    })?;

    let (tokens, lexer_errors) = lexer_tokenize_with_errors(
        &input,
        file_path.to_str().ok_or_else(|| {
            CompileError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid file path",
            ))
        })?,
    );
    if !lexer_errors.is_empty() {
        report_errors(lexer_errors);
        ()
    }

    // Print tokens with color if verbose
    if args.verbose {
        for token in tokens.clone() {
            println!(
                "{} {}",
                style(format!("{:?}", token.kind)).green(),
                style(format!("at {}", token.span)).dim()
            );
        }
    } else {
        println!("{} tokens found", tokens.len());
    }

    let parse = JsavParser::new(tokens);
    let (statements, parer_errors) = parse.parse();
    if !parer_errors.is_empty() {
        report_errors(parer_errors);
        ()
    }

    // Print statements with color if verbose
    if args.verbose {
        println!("{}", pretty_print(&statements.unwrap()));
    } else {
        println!("{} statements found", statements.iter().len());
    }

    Ok(())
}

fn report_errors(errors: Vec<CompileError>) {
    for error in errors {
        match error {
            CompileError::LexerError { message, span } => print_error("LEX", &message, &span),
            CompileError::SyntaxError { message, span } => print_error("SYNTAX", &message, &span),
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
}

fn print_error(category: &str, message: &str, span: &SourceSpan) {
    eprintln!(
        "{} {}: {}\n{} {}",
        style("ERROR").red().bold(),
        style(category).red(),
        style(message).yellow(),
        style("Location:").blue(),
        style(span).cyan()
    );
}
