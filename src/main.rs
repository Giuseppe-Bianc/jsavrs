// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::{
    Parser,
    builder::{
        PathBufValueParser, Styles,
        styling::{AnsiColor, Effects},
    },
};
use console::style;
use jsavrs::parser::ast_printer::pretty_print_stmt;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::{
    error::compile_error::CompileError, lexer::lexer_tokenize_with_errors
};
use std::{
    fs,
    path::{Path, PathBuf},
    //process,
};
use jsavrs::error::error_reporter::ErrorReporter;
use jsavrs::lexer::Lexer;

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

    let mut lexer = Lexer::new(file_path.to_str().ok_or_else(|| {
        CompileError::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file path",
        ))
    })?, &input);
    let line_tracker = lexer.get_line_tracker();
    let error_reporter: ErrorReporter = ErrorReporter::new(line_tracker);
    let (tokens, lexer_errors) = lexer_tokenize_with_errors(
        &mut lexer
    );
    if !lexer_errors.is_empty() {
        error_reporter.report_errors(lexer_errors);
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
        error_reporter.report_errors(parer_errors);
        ()
    }

    println!("parsing done");

    //Print statements with color if verbose
    if args.verbose {
        //println!("{}", pretty_print(&statements.unwrap()));
        for stat in &statements {
            println!("{}", pretty_print_stmt(stat));
        }
    } else {
        println!("{} statements found", statements.iter().len());
    }

    Ok(())
}