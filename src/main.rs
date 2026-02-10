// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::Parser;
use console::style;
// use jsavrs::asm::{Abi, AssemblyFile, DataDirective, GPRegister64, Immediate, Instruction, Operand, X86Register};
use jsavrs::cli::Args;
use jsavrs::codegen::asmgen::AsmGen;
use jsavrs::error::error_reporter::ErrorReporter;
use jsavrs::ir::optimizer::constant_folding::optimizer::ConstantFoldingOptimizer;
use jsavrs::ir::{Phase, generator::IrGenerator, optimizer::DeadCodeElimination, run_pipeline};
use jsavrs::lexer::Lexer;
use jsavrs::parser::jsav_parser::JsavParser;
//use jsavrs::printers::ast_printer::pretty_print_stmt;
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

#[cfg(feature = "dhat-heaps")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// Helper function per gestire e stampare errori I/O
fn handle_io_error<T: std::fmt::Display>(error_type: &str, e: T) {
    eprintln!("{} {}: {}\n", style("ERROR:").red().bold(), style(error_type).red(), style(e).yellow());
}
const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
const UNIT_LEN: usize = UNITS.len() - 1;

#[inline]
#[allow(clippy::cast_precision_loss)]
fn format_size(bytes: usize) -> (f64, &'static str) {
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNIT_LEN {
        size /= 1024.0;
        unit += 1;
    }

    (size, UNITS[unit])
}

#[allow(clippy::explicit_auto_deref, clippy::unused_unit, clippy::unnecessary_wraps)]
fn main() -> Result<(), CompileError> {
    #[cfg(feature = "dhat-heaps")]
    let _dhat = dhat::Profiler::new_heap();

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
    println!("total of bytes read: {size} {unit}");

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

    let parse = JsavParser::new(&tokens);
    let parse_timer = Timer::new("Parser");
    let (statements, parer_errors) = parse.parse();
    println!("{parse_timer}");
    let num_statements = statements.len();
    let num_statements_str = format!("{num_statements} statements found");
    if !parer_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(parer_errors));
        process::exit(1);
    }

    println!("parsing done");

    //Print statements
    println!("{num_statements_str}");

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
    let mut generator = IrGenerator::new();
    let nir_timer = Timer::new("NIR Generation");
    let (irmodule, ir_errors) = generator.generate(statements.clone(), file_path.to_str().unwrap());
    println!("{nir_timer}");

    if !ir_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(ir_errors));
        process::exit(1);
    }

    println!("NIR generation done");

    let pipeline: Vec<Box<dyn Phase>> = vec![
        Box::new(ConstantFoldingOptimizer::new(args.verbose, true)),
        Box::new(DeadCodeElimination::with_config(10, true, args.verbose, false)),
    ];
    if args.verbose {
        println!("Generated NIR Module:\n{irmodule}");
    }
    let mut module = irmodule;

    let optimization_timer = Timer::new("IR Optimization Pipeline");
    run_pipeline(&mut module, pipeline);
    println!("{optimization_timer}");
    println!("IR optimization done");
    /*if args.verbose {
        println!("optimized NIR Module:\n{}", module);
    }*/

    /*let mut assembly_file = AssemblyFile::new(Abi::SYSTEM_V_LINUX);
    assembly_file.data_sec_add_data("message", DataDirective::new_asciz("Hello, World!".to_string()));
    assembly_file.data_sec_add_data("message_len", DataDirective::new_equ_length_of("message"));
    assembly_file.text_sec_add_label("start");
    assembly_file.text_sec_add_instruction(Instruction::Mov {
        dest: Operand::Register(X86Register::GP64(GPRegister64::Rax)),
        src: Operand::Immediate(Immediate::Imm64(100)),
    });

    assembly_file.text_sec_add_label("start2");

    assembly_file.text_sec_add_instruction_with_comment(
        Instruction::Mov {
            dest: Operand::Register(X86Register::GP64(GPRegister64::Rcx)),
            src: Operand::Immediate(Immediate::Imm64(200)),
        },
        "move 200 -> rcx",
    );

    */
    let asm_gen: AsmGen = AsmGen::new(module);
    let (assembly_file, asm_errors) = asm_gen.gen_asm();
    if !asm_errors.is_empty() {
        eprintln!("{}", error_reporter.report_errors(asm_errors));
        process::exit(1);
    }
    println!("file:\n{assembly_file}");

    Ok(())
}
