// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::Parser;
use console::style;
use jsavrs::cli::Args;
use jsavrs::error::error_reporter::ErrorReporter;
use jsavrs::ir::generator::NIrGenerator;
use jsavrs::lexer::Lexer;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::printers::ast_printer::pretty_print_stmt;
use jsavrs::semantic::type_checker::TypeChecker;
use jsavrs::time::timer::{AutoTimer, Timer};
use jsavrs::{error::compile_error::CompileError, lexer::lexer_tokenize_with_errors};
use std::process;
use std::{
    fs,
    path::Path,
    //process,
};
//use jsavrs::asm::generator::TargetOS;

// Helper function per gestire e stampare errori I/O
fn handle_io_error<T: std::fmt::Display>(error_type: &str, e: T) {
    eprintln!("{} {}: {}\n", style("ERROR:").red().bold(), style(error_type).red(), style(e).yellow());
}
const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
const UNIT_LEN: usize = UNITS.len() - 1;

#[inline]
fn format_size(bytes: usize) -> (f64, &'static str) {
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNIT_LEN {
        size /= 1024.0;
        unit += 1;
    }

    (size, UNITS[unit])
}

#[allow(clippy::explicit_auto_deref, clippy::unused_unit)]
fn main() -> Result<(), CompileError> {
    let args = Args::parse();
    let file_path: &Path = args.input.as_path();
    let read_file_timer_name = format!("reading file {}", file_path.display());

    // Read input file with error styling
    let input = {
        let _io_timer = AutoTimer::new(&read_file_timer_name);

        fs::read_to_string(file_path).unwrap_or_else(|e| {
            handle_io_error("I/O", e);
            process::exit(1); // esce con codice 1
        })
    };

    let size_bytes = input.len();
    let (size, unit) = format_size(size_bytes);
    println!("total of bytes read: {} {}", size, unit);

    let file_path_str: &str = file_path.to_str().unwrap_or_else(|| {
        handle_io_error("I/O", std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file path"));
        process::exit(1);
    });

    let _total_timer = AutoTimer::new("Total Execution"); // Timer totale
    let mut lexer = Lexer::new(file_path_str, &input);
    let line_tracker = lexer.get_line_tracker();
    let error_reporter: ErrorReporter = ErrorReporter::new(line_tracker.clone());
    let lexer_timer = Timer::new("Lexer Tokenization");
    let (tokens, lexer_errors) = lexer_tokenize_with_errors(&mut lexer);
    println!("{lexer_timer}");
    if !lexer_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(lexer_errors));
        process::exit(1);
    }

    // Print tokens with color if verbose
    println!("{} tokens found", tokens.len());

    let parse = JsavParser::new(tokens);
    let parse_timer = Timer::new("Parser");
    let (statements, parer_errors) = parse.parse();
    println!("{parse_timer}");
    let num_statements = statements.len();
    let num_statements_str = format!("{} statements found", num_statements);
    if !parer_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(parer_errors));
        process::exit(1);
    }

    println!("parsing done");

    //Print statements with color if verbose
    if args.verbose {
        let print_ast_timer = Timer::new("AST Pretty Print");
        //println!("{}", pretty_print(&statements.unwrap()));
        if num_statements > 5 {
            println!("{num_statements_str}");
        } else {
            for stat in &statements {
                println!("{}", pretty_print_stmt(stat));
            }
        }
        println!("{print_ast_timer}");
    } else {
        println!("{num_statements_str}");
    }

    let type_check_timer = Timer::new("Type Checking");
    let mut type_checkr = TypeChecker::new();
    let type_check_errors = type_checkr.check(&*statements);
    println!("{type_check_timer}");
    println!("type checking done");
    if !type_check_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(type_check_errors));
        process::exit(1);
    }

    // Extract type information from the type checker to guide IR generation
    let mut generator = NIrGenerator::new();
    let nir_timer = Timer::new("NIR Generation");
    let (module, ir_errors) = generator.generate(statements.clone(), file_path.to_str().unwrap());
    println!("{nir_timer}");

    if !ir_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(ir_errors));
        process::exit(1);
    }

    println!("NIR generation done");

    // Print the module
    if args.verbose {
        println!("{module}");
    }

    Ok(())
}
