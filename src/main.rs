// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::Parser;
use console::style;
use jsavrs::cli::Args;
use jsavrs::error::error_reporter::ErrorReporter;
use jsavrs::lexer::Lexer;
//use jsavrs::nir::generator::NIrGenerator;
use jsavrs::parser::ast_printer::pretty_print_stmt;
use jsavrs::parser::jsav_parser::JsavParser;
use jsavrs::semantic::type_checker::TypeChecker;
use jsavrs::time::timer::{AutoTimer, Timer};
use jsavrs::{error::compile_error::CompileError, lexer::lexer_tokenize_with_errors};
use std::process;
use std::{
    fs,
    path::Path,
    //process,
};
use jsavrs::rvir::generator::RIrGenerator;
//use jsavrs::asm::generator::{AsmGenerator, TargetOS};

#[allow(clippy::explicit_auto_deref, clippy::unused_unit)]
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
        process::exit(1);
    }

    // Print tokens with color if verbose
    println!("{} tokens found", tokens.len());

    let parse = JsavParser::new(tokens);
    let parse_timer = Timer::new("Parser");
    let (statements, parer_errors) = parse.parse();
    println!("{parse_timer}");
    let num_statements = statements.iter().len();
    let num_statements_str = format!("{} statements found", num_statements);
    if !parer_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(parer_errors));
        process::exit(1);
    }

    println!("parsing done");

    //Print statements with color if verbose
    if args.verbose {
        //println!("{}", pretty_print(&statements.unwrap()));
        if num_statements > 5{
            println!("{num_statements_str}");
        } else {
            for stat in &statements {
                println!("{}", pretty_print_stmt(stat));
            }
        }
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

    /*let mut generator = NIrGenerator::new();
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
        println!("{module:?}");
        println!("{module}");
    }*/

    /*let mut buidler =  IrBuilder::new(file_path.as_os_str().to_str().unwrap().to_string());
    let nir_timer = Timer::new("NIR Generation");
    let module = buidler.build(statements.clone());
    println!("{nir_timer}");
    println!("{module:#?}");*/
    let mut generator = RIrGenerator::new();
    let rir_timer = Timer::new("RIR Generation");
    let (module, rir_errors) = generator.generate(statements.clone(), file_path.to_str().unwrap());
    println!("{rir_timer}");
    if !rir_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(rir_errors));
        process::exit(1);
    }
    println!("RIR generation done");
    if args.verbose {
        println!("{module}");
    }


    /*let mut asm_gen = AsmGenerator::new(if cfg!(windows) { TargetOS::Windows } else { TargetOS::Linux });
    let (nasm_code, asm_error) = asm_gen.generate(functions);
    std::fs::write("output.asm", nasm_code)?;*/
    Ok(())
}
