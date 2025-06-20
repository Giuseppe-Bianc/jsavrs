use jsavrs::ir::{BasicBlock, Function, IrType};

// Helper to create a basic block with a label
fn create_basic_block(label: &str) -> BasicBlock {
    BasicBlock::new(label)
}

// Helper to create an integer type
fn int_type() -> IrType {
    IrType::I32
}

// Helper to create a float type
fn float_type() -> IrType {
    IrType::F32
}

#[test]
fn test_function_creation() {
    let func = Function::new("test", Vec::new(), int_type());

    assert_eq!(func.name, "test");
    assert_eq!(func.parameters.len(), 0);
    assert_eq!(func.return_type, int_type());
    assert!(func.basic_blocks.is_empty());
    assert!(func.local_vars.is_empty());
}

#[test]
fn test_function_with_parameters() {
    let params = vec![
        ("a".to_string(), int_type()),
        ("b".to_string(), float_type()),
    ];
    let func = Function::new("test", params.clone(), float_type());

    assert_eq!(func.parameters, params);
    assert_eq!(func.return_type, float_type());
}

#[test]
fn test_add_block() {
    let mut func = Function::new("test", Vec::new(), int_type());
    let block = create_basic_block("entry");

    func.add_block(block.clone());

    assert_eq!(func.basic_blocks.len(), 1);
    assert_eq!(func.basic_blocks[0], block);
}

#[test]
fn test_add_multiple_blocks() {
    let mut func = Function::new("test", Vec::new(), int_type());
    let block1 = create_basic_block("entry");
    let block2 = create_basic_block("exit");

    func.add_block(block1.clone());
    func.add_block(block2.clone());

    assert_eq!(func.basic_blocks.len(), 2);
    assert_eq!(func.basic_blocks[0], block1);
    assert_eq!(func.basic_blocks[1], block2);
}

#[test]
fn test_add_local_var() {
    let mut func = Function::new("test", Vec::new(), int_type());

    func.add_local("x".to_string(), int_type());
    func.add_local("y".to_string(), float_type());

    assert_eq!(func.local_vars.len(), 2);
    assert_eq!(func.local_vars["x"], int_type());
    assert_eq!(func.local_vars["y"], float_type());
}

#[test]
fn test_add_duplicate_local_var() {
    let mut func = Function::new("test", Vec::new(), int_type());

    func.add_local("x".to_string(), int_type());
    func.add_local("x".to_string(), float_type()); // Overwrites previous

    assert_eq!(func.local_vars.len(), 1);
    assert_eq!(func.local_vars["x"], float_type());
}

#[test]
fn test_display_no_blocks() {
    let func = Function::new("example", Vec::new(), int_type());
    let output = format!("{}", func);

    assert_eq!(output, "function example () -> i32:\n");
}

#[test]
fn test_display_with_parameters() {
    let params = vec![
        ("a".to_string(), int_type()),
        ("b".to_string(), float_type()),
    ];
    let func = Function::new("example", params, int_type());
    let output = format!("{}", func);

    assert_eq!(output, "function example (a: i32, b: f32) -> i32:\n");
}

#[test]
fn test_display_with_blocks() {
    let mut func = Function::new("example", Vec::new(), int_type());
    let block1 = create_basic_block("entry");
    let block2 = create_basic_block("exit");

    func.add_block(block1);
    func.add_block(block2);

    let output = format!("{}", func);
    let lines: Vec<&str> = output.trim_end().split('\n').collect();

    assert_eq!(lines.len(), 5); // Header + 2 blocks
    assert_eq!(lines[0], "function example () -> i32:");
    assert_eq!(lines[1], "entry:");
    assert_eq!(lines[2], "  unreachable");
    assert_eq!(lines[3], "exit:");
    assert_eq!(lines[4], "  unreachable");
}

#[test]
fn test_display_special_characters() {
    let func = Function::new("weird<>name", Vec::new(), int_type());
    let output = format!("{}", func);

    assert_eq!(output, "function weird<>name () -> i32:\n");
}

#[test]
fn test_empty_function_name() {
    let func = Function::new("", Vec::new(), int_type());
    let output = format!("{}", func);

    assert_eq!(output, "function  () -> i32:\n");
}

#[test]
fn test_function_with_many_blocks() {
    let mut func = Function::new("test", Vec::new(), int_type());
    for i in 0..100 {
        func.add_block(create_basic_block(&format!("block{}", i)));
    }

    assert_eq!(func.basic_blocks.len(), 100);
    let output = format!("{}", func);
    let lines: Vec<&str> = output.trim_end().split('\n').collect();
    assert_eq!(lines.len(), 201); // Header + 100 blocks
}
