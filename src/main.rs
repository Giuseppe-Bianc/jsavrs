// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::{Parser};
use console::style;
use jsavrs::error::error_reporter::ErrorReporter;
use jsavrs::lexer::Lexer;
use jsavrs::parser::ast_printer::pretty_print_stmt;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::time::timer::{AutoTimer, Timer};
use jsavrs::{error::compile_error::CompileError, lexer::lexer_tokenize_with_errors};
use std::{
    fs,
    path::Path,
    //process,
};
use jsavrs::cli::Args;

#[allow(clippy::unused_unit)]
fn main() -> Result<(), CompileError> {
    let _total_timer = AutoTimer::new("Total Execution"); // Timer totale
    let args = Args::parse();
    let file_path: &Path = args.input.as_path();

    // Read input file with error styling
    let input = {
        let _io_timer = AutoTimer::new("File I/O");
        fs::read_to_string(file_path).map_err(|e| {
            eprintln!(
                "{} {}: {}",
                style("ERROR:").red().bold(),
                style("I/O").red(),
                style(format!("{e}")).yellow()
            );
            e
        })?
    };

    let mut lexer = Lexer::new(
        file_path.to_str().ok_or_else(|| {
            CompileError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid file path",
            ))
        })?,
        &input,
    );
    let line_tracker = lexer.get_line_tracker();
    let error_reporter: ErrorReporter = ErrorReporter::new(line_tracker);
    let lexer_timer = Timer::new("Lexer Tokenization");
    let (tokens, lexer_errors) = lexer_tokenize_with_errors(&mut lexer);
    println!("{lexer_timer}");
    if !lexer_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(lexer_errors));
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
    let parse_timer = Timer::new("Parser");
    let (statements, parer_errors) = parse.parse();
    println!("{parse_timer}");
    if !parer_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(parer_errors));
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
