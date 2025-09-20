//! Main entry point for demonstrating the assembly generator
use jsavrs::asm::generator::{NasmGenerator, TargetOS};
use jsavrs::asm::register::Register;
use jsavrs::asm::operand::Operand;
use jsavrs::asm::instruction::Instruction;
use jsavrs::asm::generator::Section;

fn main() {
    println!("=== x86-64 NASM Assembly Generator ===
");
    
    // Example 1: Hello World for Linux
    let mut generator = NasmGenerator::new(TargetOS::Linux);
    generator.create_hello_world_linux();
    
    println!("1. Hello World Program (Linux):\n{}", generator.generate());
    
    // Save to file
    if let Err(e) = generator.save_to_file("hello_world.asm") {
        eprintln!("Error saving hello_world.asm: {}", e);
    } else {
        println!("File 'hello_world.asm' saved successfully!\n");
    }
    
    // Example 2: Factorial function
    let mut generator2 = NasmGenerator::new(TargetOS::Linux);
    generator2.add_standard_prelude();
    generator2.add_section(Section::Text);
    generator2.add_global("_start");
    generator2.add_empty_line();
    
    generator2.create_factorial_function();
    
    // Main program
    generator2.add_label("_start");
    generator2.add_comment("Calculate factorial of 5");
    generator2.add_instruction(Instruction::Mov(Operand::reg(Register::RDI), Operand::imm(5)));
    generator2.add_instruction(Instruction::Call("factorial".to_string()));
    
    generator2.add_comment("Result is now in RAX");
    generator2.add_comment("Exit program");
    generator2.add_instruction(Instruction::Mov(Operand::reg(Register::RDI), Operand::reg(Register::RAX)));
    generator2.add_instruction(Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(60)));
    generator2.add_instruction(Instruction::Syscall);
    
    println!("2. Program with factorial function:\n{}", generator2.generate());
    
    if let Err(e) = generator2.save_to_file("factorial.asm") {
        eprintln!("Error saving factorial.asm: {}", e);
    } else {
        println!("File 'factorial.asm' saved successfully!\n");
    }
    
    // Example 3: Advanced usage with dynamic loop generation
    println!("3. Dynamic loop generation example:");
    let mut generator3 = NasmGenerator::new(TargetOS::Linux);
    generator3.add_standard_prelude();
    generator3.add_section(Section::Text);
    generator3.add_global("_start");
    generator3.add_empty_line();
    
    // Generate a loop dynamically
    generator3.add_label("_start");
    generator3.add_comment("Initialize counter");
    generator3.add_instruction(Instruction::Mov(Operand::reg(Register::RCX), Operand::imm(10)));
    
    let loop_label = generator3.generate_label("loop");
    let end_label = generator3.generate_label("end");
    
    generator3.add_label(&loop_label);
    generator3.add_instruction(Instruction::Dec(Operand::reg(Register::RCX)));
    generator3.add_instruction(Instruction::Cmp(Operand::reg(Register::RCX), Operand::imm(0)));
    generator3.add_instruction(Instruction::Jne(loop_label.clone()));
    
    generator3.add_label(&end_label);
    generator3.add_instruction(Instruction::Mov(Operand::reg(Register::RAX), Operand::imm(60)));
    generator3.add_instruction(Instruction::Mov(Operand::reg(Register::RDI), Operand::imm(0)));
    generator3.add_instruction(Instruction::Syscall);
    
    println!("{}", generator3.generate());
    
    println!("\n=== Compilation Instructions ===");
    println!("For Linux:");
    println!("  nasm -f elf64 hello_world.asm -o hello_world.o");
    println!("  ld hello_world.o -o hello_world");
    println!("  ./hello_world");
    println!();
    println!("For Windows (with MinGW):\nnasm -f win64 program.asm -o program.o\ngcc program.o -o program.exe\n");
    println!("For macOS:\nnasm -f macho64 program.asm -o program.o\n  ld program.o -o program -macosx_version_min 10.7.0 -lSystem -syslibroot `xcrun -show-sdk-path`");
}