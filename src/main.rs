// run --package jsavrs --bin jsavrs -- -i C:/dev/visualStudio/transpiler/Vandior/input.vn -v
use clap::Parser;
use console::style;
use jsavrs::asm::{
    Abi, AbiKind, AssemblyElement, AssemblyFile, AssemblySection, DataDirective, GPRegister64, Immediate, Instruction,
    MemoryOperand, Operand, Platform, Section, X86Register,
};
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

    /*// Test Platform display
    println!("Platform: {}", Platform::Linux);

    // Test AbiKind display
    println!("AbiKind: {}", AbiKind::SystemV);

    // Test Abi display
    let abi = Abi::SYSTEM_V_LINUX;
    println!("Abi: {}", abi);

    // Test Section display
    println!("Section: {}", Section::Text);

    // Test register display
    println!("Register: {}", GPRegister64::Rax);
    println!("X86Register: {}", X86Register::GP64(GPRegister64::Rbx));

    // Test immediate display
    let imm = Immediate::Imm32(42);
    println!("Immediate: {}", imm);

    // Test operand display
    let operand = Operand::Register(X86Register::GP64(GPRegister64::Rcx));
    println!("Operand: {}", operand);

    // Test memory operand display
    let mem_op = MemoryOperand::new(Some(GPRegister64::Rsp)).with_displacement(8);
    println!("MemoryOperand: {}", mem_op);

    // Test instruction display
    let instr = Instruction::Mov {
        dest: Operand::Register(X86Register::GP64(GPRegister64::Rax)),
        src: Operand::Immediate(Immediate::Imm32(42))
    };
    println!("Instruction: {}", instr);

    // Test data directive display
    let data_dir = DataDirective::Dd(vec![1, 2, 3]);
    println!("DataDirective: {}", data_dir);

    // Test assembly element display
    let elem_label = AssemblyElement::Label("my_label".to_string());
    let elem_instr = AssemblyElement::Instruction(Instruction::Nop);
    let elem_comment = AssemblyElement::Comment("This is a block comment".to_string());
    let elem_data = AssemblyElement::Data("my_var".to_string(), DataDirective::Db(vec![1, 2, 3]));

    let elem_inline_comment = AssemblyElement::InstructionWithComment(Instruction::Nop, "This is an inline comment".to_string());

    println!("Label element: {}", elem_label);
    println!("Instruction element: {}", elem_instr);
    println!("Block comment element: {}", elem_comment);
    println!("Inline comment element: {}", elem_inline_comment);
    println!("Data element: {}", elem_data);

    // Test assembly section display
    let mut section = AssemblySection::text_section();
    section.add_label("start");
    section.add_instruction(Instruction::Mov {
        dest: Operand::Register(X86Register::GP64(GPRegister64::Rax)),
        src: Operand::Immediate(Immediate::Imm64(100))
    });
    section.add_data("message", DataDirective::Asciz("Hello, World!".to_string()));
    section.add_comment("End of example");
    section.add_label("start");
    section.add_instruction_with_comment(Instruction::Mov {
        dest: Operand::Register(X86Register::GP64(GPRegister64::Rcx)),
        src: Operand::Immediate(Immediate::Imm64(200))
    }, "move 200 -> rcx");

    println!("\nAssemblySection:\n{}", section);*/

    let mut assembly_file = AssemblyFile::new(Abi::SYSTEM_V_LINUX);
    assembly_file.data_sec_add_data("message", DataDirective::new_asciz("Hello, World!".to_string()));
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

    println!("file:\n{}", assembly_file);

    Ok(())
}
