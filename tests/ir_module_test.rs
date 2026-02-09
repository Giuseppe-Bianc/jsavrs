use std::sync::Arc;

use jsavrs::ir::{DataLayout, Function, IrParameter, IrType, Module, ParamAttributes, ScopeId, TargetTriple};
use jsavrs::location::source_span::SourceSpan;
use jsavrs::utils::module_redacted;
// tests/nir_module_test.rs

// Helper function per creare un parametro IR di test
fn create_test_param(name: &str, ty: IrType) -> IrParameter {
    IrParameter { name: name.into(), ty, attributes: ParamAttributes::default() }
}

// Helper function per creare una funzione di test
fn create_test_function(name: &str) -> Function {
    let params = vec![create_test_param("a", IrType::I32), create_test_param("b", IrType::F32)];
    Function::new(name, params, IrType::Void)
}

#[test]
fn test_new_module_with_defaults() {
    // Test: Creazione di un nuovo modulo con impostazioni predefinite
    let module = Module::new("test_module".to_string(), Some(ScopeId::new()));

    // Verifica delle proprietà di base
    assert_eq!(module.name(), "test_module");
    assert_eq!(module.functions().len(), 0);

    // Verifica che le impostazioni riflettano l'host OS (auto-detected)
    let expected_platform = jsavrs::ir::platform::detect_host_platform();
    assert_eq!(*module.data_layout(), expected_platform.data_layout);
    assert_eq!(*module.target_triple(), expected_platform.target_triple);

    // Verifica della rappresentazione testuale con target esplicito Linux
    let mut pinned = Module::new("test_module".to_string(), Some(ScopeId::new()));
    pinned.set_data_layout(DataLayout::LinuxX86_64);
    pinned.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
    let expected = r#"module test_module {
  data_layout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
  target_triple = "x86_64-unknown-linux-gnu";
  // No functions
}"#;
    assert_eq!(pinned.to_string(), expected);
}

#[test]
fn test_add_function() {
    // Test: Aggiunta di una funzione al modulo
    let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));
    // Pin to Linux for deterministic display string
    module.set_data_layout(DataLayout::LinuxX86_64);
    module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);
    let mut function = create_test_function("test_func");
    function.add_block("entry_test_func", SourceSpan::default());
    module.add_function(function.clone());

    // Verifica che la funzione sia stata aggiunta
    assert_eq!(module.functions().len(), 1);
    assert_eq!(module.functions()[0].name, Arc::from("test_func"));

    // Verifica della rappresentazione testuale
    let expected = r#"module test_module {
  data_layout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
  target_triple = "x86_64-unknown-linux-gnu";
function test_func (a: i32, b: f32) -> void:
block:
// Scope: SCOPE_0
entry_test_func:
  unreachable


}"#;
    assert_eq!(module_redacted(module), expected);
}

#[test]
fn test_add_multiple_functions() {
    // Test: Aggiunta di più funzioni al modulo
    let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));
    let func1 = create_test_function("func1");
    let func2 = create_test_function("func2");
    let func3 = create_test_function("func3");

    module.add_function(func1);
    module.add_function(func2);
    module.add_function(func3);

    // Verifica che tutte le funzioni siano state aggiunte
    assert_eq!(module.functions().len(), 3);
    assert_eq!(module.functions()[0].name, Arc::from("func1"));
    assert_eq!(module.functions()[1].name, Arc::from("func2"));
    assert_eq!(module.functions()[2].name, Arc::from("func3"));

    // Verifica della rappresentazione testuale
    let output = module.to_string();
    assert!(output.contains("function func1"));
    assert!(output.contains("function func2"));
    assert!(output.contains("function func3"));
}

#[test]
fn test_set_data_layout() {
    // Test: Impostazione di diversi layout di dati
    let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));

    // Verifica layout predefinito (auto-detected dall'host)
    let expected_platform = jsavrs::ir::platform::detect_host_platform();
    assert_eq!(*module.data_layout(), expected_platform.data_layout);

    // Imposta e verifica Windows layout
    module.set_data_layout(DataLayout::WindowsX86_64);
    assert_eq!(*module.data_layout(), DataLayout::WindowsX86_64);

    // Imposta e verifica macOS layout
    module.set_data_layout(DataLayout::MacOSX86_64);
    assert_eq!(*module.data_layout(), DataLayout::MacOSX86_64);

    // Pin target_triple per la rappresentazione testuale deterministica
    module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);

    // Verifica rappresentazione testuale
    let expected = r#"module test_module {
  data_layout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
  target_triple = "x86_64-unknown-linux-gnu";
  // No functions
}"#;
    assert_eq!(module.to_string(), expected);
}

#[test]
fn test_set_target_triple() {
    // Test: Impostazione di diverse triplette di destinazione
    let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));

    // Verifica tripletta predefinita (auto-detected dall'host)
    let expected_platform = jsavrs::ir::platform::detect_host_platform();
    assert_eq!(*module.target_triple(), expected_platform.target_triple);

    // Imposta e verifica Windows tripletta
    module.set_target_triple(TargetTriple::X86_64PcWindowsGnu);
    assert_eq!(*module.target_triple(), TargetTriple::X86_64PcWindowsGnu);

    // Imposta e verifica macOS tripletta
    module.set_target_triple(TargetTriple::X86_64AppleDarwin);
    assert_eq!(*module.target_triple(), TargetTriple::X86_64AppleDarwin);

    // Pin data_layout per la rappresentazione testuale deterministica
    module.set_data_layout(DataLayout::LinuxX86_64);

    // Verifica rappresentazione testuale
    let expected = r#"module test_module {
  data_layout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
  target_triple = "x86_64-apple-darwin";
  // No functions
}"#;
    assert_eq!(module.to_string(), expected);
}

#[test]
fn test_get_function() {
    // Test: Ricerca di una funzione per nome (riferimento immutabile)
    let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));
    let func1 = create_test_function("func1");
    let func2 = create_test_function("func2");

    module.add_function(func1);
    module.add_function(func2);

    // Verifica recupero funzione esistente
    let retrieved = module.get_function("func1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, Arc::from("func1"));

    // Verifica recupero funzione inesistente
    let retrieved = module.get_function("nonexistent");
    assert!(retrieved.is_none());
}

#[test]
fn test_get_function_mut() {
    // Test: Ricerca di una funzione per nome (riferimento mutabile)
    let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));
    let func1 = create_test_function("func1");
    let func2 = create_test_function("func2");

    module.add_function(func1);
    module.add_function(func2);

    // Verifica recupero e modifica funzione esistente
    if let Some(func) = module.get_function_mut("func1") {
        func.name = Arc::from("modified_func");
    }

    // Verifica che la modifica sia stata applicata
    let retrieved = module.get_function("modified_func");
    assert!(retrieved.is_some());

    // Verifica che il nome originale non esista più
    let retrieved = module.get_function("func1");
    assert!(retrieved.is_none());

    // Verifica recupero funzione inesistente
    let retrieved = module.get_function_mut("nonexistent");
    assert!(retrieved.is_none());
}

#[test]
fn test_empty_module_display() {
    // Test: Rappresentazione testuale di un modulo vuoto
    // Pin to Linux for deterministic display string
    let mut module = Module::new("empty_module".to_string(), Some(ScopeId::new()));
    module.set_data_layout(DataLayout::LinuxX86_64);
    module.set_target_triple(TargetTriple::X86_64UnknownLinuxGnu);

    let expected = r#"module empty_module {
  data_layout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
  target_triple = "x86_64-unknown-linux-gnu";
  // No functions
}"#;
    assert_eq!(module.to_string(), expected);
}

#[test]
fn test_module_with_complex_function() {
    // Test: Modulo con una funzione complessa (con blocchi e variabili locali)
    let mut module = Module::new("complex_module".to_string(), Some(ScopeId::new()));
    let mut function = create_test_function("complex_func");

    // Aggiungi blocchi base
    function.add_block("block1", SourceSpan::default());
    function.add_block("block2", SourceSpan::default());

    module.add_function(function);

    // Verifica rappresentazione testuale
    let output = module.to_string();
    assert!(output.contains("function complex_func"));

    // Le variabili locali non vengono mostrate direttamente nell'output della funzione
    // ma sono memorizzate nella struttura dati

    // Verifica la presenza dei blocchi
    assert!(output.contains("block1:"));
    assert!(output.contains("block2:"));

    // I blocchi vuoti mostrano l'istruzione unreachable
    assert!(output.contains("unreachable"));

    // Verifica che i predecessori siano mostrati correttamente
    assert!(output.contains("block1"));
}

#[test]
fn test_module_with_all_data_layouts() {
    // Test: Verifica di tutti i layout di dati supportati
    let layouts = vec![
        DataLayout::LinuxX86_64,
        DataLayout::LinuxAArch64,
        DataLayout::WindowsX86_64,
        DataLayout::MacOSX86_64,
        DataLayout::FreeBSDX86_64,
        DataLayout::NetBSDX86_64,
        DataLayout::OpenBSDX86_64,
        DataLayout::DragonFlyX86_64,
    ];

    for layout in layouts {
        let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));
        module.set_data_layout(layout);

        // Verifica che il layout sia stato impostato correttamente
        assert_eq!(*module.data_layout(), layout);

        // Verifica che la rappresentazione testuale contenga la stringa corretta
        let output = module.to_string();
        let layout_str = layout.to_string();
        assert!(output.contains(&layout_str));
    }
}

#[test]
fn test_module_with_all_target_triples() {
    // Test: Verifica di tutte le triplette di destinazione supportate
    let triples = vec![
        TargetTriple::X86_64UnknownLinuxGnu,
        TargetTriple::X86_64PcWindowsGnu,
        TargetTriple::X86_64AppleDarwin,
        TargetTriple::AArch64UnknownLinuxGnu,
        TargetTriple::AArch64AppleDarwin,
        TargetTriple::AArch64PcWindowsGnu,
        TargetTriple::I686PcWindowsGnu,
        TargetTriple::I686UnknownLinuxGnu,
        TargetTriple::Wasm32UnknownEmscripten,
    ];

    for triple in triples {
        let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));
        module.set_target_triple(triple);

        // Verifica che la tripletta sia stata impostata correttamente
        assert_eq!(*module.target_triple(), triple);

        // Verifica che la rappresentazione testuale contenga la stringa corretta
        let output = module.to_string();
        let triple_str = triple.to_string();
        assert!(output.contains(&triple_str));
    }
}

#[test]
fn test_module_with_special_characters_in_name() {
    // Test: Modulo con caratteri speciali nel nome
    let mut module = Module::new("module_with_special_chars_!@#$%^&*()".to_string(), Some(ScopeId::new()));
    let function = create_test_function("func_with_special_chars_!@#$%^&*()");
    module.add_function(function);

    // Verifica che il nome sia conservato correttamente
    assert_eq!(module.name(), "module_with_special_chars_!@#$%^&*()");
    assert_eq!(module.functions()[0].name, Arc::from("func_with_special_chars_!@#$%^&*()"));

    // Verifica rappresentazione testuale
    let output = module.to_string();
    assert!(output.contains("module module_with_special_chars_!@#$%^&*()"));
    assert!(output.contains("function func_with_special_chars_!@#$%^&*()"));
}

#[test]
fn test_module_with_duplicate_function_names() {
    // Test: Aggiunta di funzioni con nomi duplicati
    let mut module = Module::new("test_module".to_string(), Some(ScopeId::new()));
    let func1 = create_test_function("duplicate_name");
    let func2 = create_test_function("duplicate_name");

    module.add_function(func1);
    module.add_function(func2);

    // Verifica che entrambe le funzioni siano state aggiunte
    assert_eq!(module.functions().len(), 2);
    assert_eq!(module.functions()[0].name, Arc::from("duplicate_name"));
    assert_eq!(module.functions()[1].name, Arc::from("duplicate_name"));

    // Verifica che get_function restituisca la prima occorrenza
    let retrieved = module.get_function("duplicate_name");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, Arc::from("duplicate_name"));
}

#[test]
fn test_module_getters() {
    // Test: Verifica di tutti i metodi getter
    let mut module = Module::new("getter_test".to_string(), Some(ScopeId::new()));
    let function = create_test_function("getter_func");
    module.add_function(function);

    // Imposta layout e tripletta personalizzati
    module.set_data_layout(DataLayout::MacOSX86_64);
    module.set_target_triple(TargetTriple::AArch64AppleDarwin);

    // Verifica tutti i getter
    assert_eq!(module.name(), "getter_test");
    assert_eq!(module.functions().len(), 1);
    assert_eq!(module.functions()[0].name, Arc::from("getter_func"));
    assert_eq!(*module.data_layout(), DataLayout::MacOSX86_64);
    assert_eq!(*module.target_triple(), TargetTriple::AArch64AppleDarwin);
}

#[test]
fn test_module_display_formatting() {
    // Test: Verifica della formattazione della rappresentazione testuale
    let mut module = Module::new("format_test".to_string(), Some(ScopeId::new()));
    let mut function = create_test_function("format_func");
    function.add_block("entry_format_func", SourceSpan::default());
    module.add_function(function);

    // Imposta layout e tripletta personalizzati
    module.set_data_layout(DataLayout::WindowsX86_64);
    module.set_target_triple(TargetTriple::X86_64PcWindowsGnu);

    let expected = r#"module format_test {
  data_layout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
  target_triple = "x86_64-pc-windows-gnu";
function format_func (a: i32, b: f32) -> void:
block:
// Scope: SCOPE_0
entry_format_func:
  unreachable


}"#;
    assert_eq!(module_redacted(module), expected);
}

#[test]
fn test_count_instructions_empty_module() {
    // Test: Conteggio istruzioni in un modulo vuoto
    let module = Module::new("empty_module", None);

    // Un modulo senza funzioni dovrebbe avere 0 istruzioni
    assert_eq!(module.count_instructions(), 0);
}

#[test]
fn test_count_instructions_single_function_no_instructions() {
    // Test: Conteggio istruzioni in una funzione senza istruzioni
    let mut module = Module::new("test_module", None);
    let mut function = create_test_function("empty_func");

    // Aggiungi un blocco base ma senza istruzioni
    function.add_block("entry_empty_func", SourceSpan::default());
    module.add_function(function);

    // Dovrebbe essere 0 perché il blocco non contiene istruzioni
    assert_eq!(module.count_instructions(), 0);
}

#[test]
fn test_count_instructions_single_function_with_instructions() {
    use jsavrs::ir::{Instruction, InstructionKind, IrLiteralValue, Value};

    // Test: Conteggio istruzioni in una funzione con istruzioni
    let mut module = Module::new("test_module", None);
    let mut function = create_test_function("test_func");

    // Aggiungi un blocco e alcune istruzioni
    function.add_block("entry_test_func", SourceSpan::default());

    // Aggiungi 3 istruzioni
    let inst1 = Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default())
        .with_result(Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))));

    let inst2 = Instruction::new(InstructionKind::Alloca { ty: IrType::F32 }, SourceSpan::default())
        .with_result(Value::new_temporary(1, IrType::Pointer(Box::new(IrType::F32))));

    let inst3 = Instruction::new(
        InstructionKind::Store {
            value: Value::new_literal(IrLiteralValue::I32(42)),
            dest: Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))),
        },
        SourceSpan::default(),
    );

    function.add_instruction("entry_test_func", inst1);
    function.add_instruction("entry_test_func", inst2);
    function.add_instruction("entry_test_func", inst3);

    module.add_function(function);

    // Dovrebbe contare 3 istruzioni
    assert_eq!(module.count_instructions(), 3);
}
#[test]
fn test_count_instructions_multiple_functions() {
    use jsavrs::ir::{Instruction, InstructionKind, Value};

    // Test: Conteggio istruzioni attraverso più funzioni
    let mut module = Module::new("test_module", None);

    // Funzione 1 con 2 istruzioni
    let mut func1 = create_test_function("func1");
    func1.add_block("entry_func1", SourceSpan::default());
    let inst1 = Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default())
        .with_result(Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))));
    let inst2 = Instruction::new(InstructionKind::Alloca { ty: IrType::F32 }, SourceSpan::default())
        .with_result(Value::new_temporary(1, IrType::Pointer(Box::new(IrType::F32))));
    func1.add_instruction("entry_func1", inst1);
    func1.add_instruction("entry_func1", inst2);

    // Funzione 2 con 3 istruzioni
    let mut func2 = create_test_function("func2");
    func2.add_block("entry_func2", SourceSpan::default());
    let inst3 = Instruction::new(InstructionKind::Alloca { ty: IrType::I64 }, SourceSpan::default())
        .with_result(Value::new_temporary(2, IrType::Pointer(Box::new(IrType::I64))));
    let inst4 = Instruction::new(InstructionKind::Alloca { ty: IrType::F64 }, SourceSpan::default())
        .with_result(Value::new_temporary(3, IrType::Pointer(Box::new(IrType::F64))));
    let inst5 = Instruction::new(InstructionKind::Alloca { ty: IrType::Bool }, SourceSpan::default())
        .with_result(Value::new_temporary(4, IrType::Pointer(Box::new(IrType::Bool))));
    func2.add_instruction("entry_func2", inst3);
    func2.add_instruction("entry_func2", inst4);
    func2.add_instruction("entry_func2", inst5);

    module.add_function(func1);
    module.add_function(func2);

    // Totale: 2 + 3 = 5 istruzioni
    assert_eq!(module.count_instructions(), 5);
}
#[test]
fn test_count_instructions_multiple_blocks() {
    use jsavrs::ir::{Instruction, InstructionKind, Value};

    // Test: Conteggio istruzioni attraverso più blocchi in una funzione
    let mut module = Module::new("test_module", None);
    let mut function = create_test_function("multi_block_func");

    // Blocco 1 con 2 istruzioni
    function.add_block("entry_multi_block_func", SourceSpan::default());
    let inst1 = Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default())
        .with_result(Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))));
    let inst2 = Instruction::new(InstructionKind::Alloca { ty: IrType::F32 }, SourceSpan::default())
        .with_result(Value::new_temporary(1, IrType::Pointer(Box::new(IrType::F32))));
    function.add_instruction("entry_multi_block_func", inst1);
    function.add_instruction("entry_multi_block_func", inst2);

    // Blocco 2 con 1 istruzione
    function.add_block("block2", SourceSpan::default());
    let inst3 = Instruction::new(InstructionKind::Alloca { ty: IrType::I64 }, SourceSpan::default())
        .with_result(Value::new_temporary(2, IrType::Pointer(Box::new(IrType::I64))));
    function.add_instruction("block2", inst3);

    module.add_function(function);

    // Totale: 2 + 1 = 3 istruzioni
    assert_eq!(module.count_instructions(), 3);
}

#[test]
#[allow(clippy::approx_constant)]
fn test_count_instructions_complex_scenario() {
    use jsavrs::ir::{Instruction, InstructionKind, IrLiteralValue, Value};

    // Test: Scenario complesso con più funzioni, blocchi e istruzioni
    let mut module = Module::new("complex_module", None);

    // Funzione 1: 2 blocchi con 3 istruzioni totali
    let mut func1 = create_test_function("func1");
    func1.add_block("entry_func1", SourceSpan::default());
    func1.add_block("block1_a", SourceSpan::default());

    func1.add_instruction(
        "entry_func1",
        Instruction::new(InstructionKind::Alloca { ty: IrType::I32 }, SourceSpan::default())
            .with_result(Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32)))),
    );
    func1.add_instruction(
        "block1_a",
        Instruction::new(
            InstructionKind::Store {
                value: Value::new_literal(IrLiteralValue::I32(10)),
                dest: Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))),
            },
            SourceSpan::default(),
        ),
    );
    func1.add_instruction(
        "block1_a",
        Instruction::new(
            InstructionKind::Load {
                src: Value::new_temporary(0, IrType::Pointer(Box::new(IrType::I32))),
                ty: IrType::I32,
            },
            SourceSpan::default(),
        )
        .with_result(Value::new_temporary(1, IrType::I32)),
    );

    // Funzione 2: 1 blocco senza istruzioni
    let mut func2 = create_test_function("func2");
    func2.add_block("entry_func2", SourceSpan::default());

    // Funzione 3: 3 blocchi con 5 istruzioni totali
    let mut func3 = create_test_function("func3");
    func3.add_block("entry_func3", SourceSpan::default());
    func3.add_block("block3_a", SourceSpan::default());
    func3.add_block("block3_b", SourceSpan::default());

    func3.add_instruction(
        "entry_func3",
        Instruction::new(InstructionKind::Alloca { ty: IrType::F32 }, SourceSpan::default())
            .with_result(Value::new_temporary(2, IrType::Pointer(Box::new(IrType::F32)))),
    );
    func3.add_instruction(
        "entry_func3",
        Instruction::new(InstructionKind::Alloca { ty: IrType::Bool }, SourceSpan::default())
            .with_result(Value::new_temporary(3, IrType::Pointer(Box::new(IrType::Bool)))),
    );
    func3.add_instruction(
        "block3_a",
        Instruction::new(
            InstructionKind::Store {
                value: Value::new_literal(IrLiteralValue::F32(3.14)),
                dest: Value::new_temporary(2, IrType::Pointer(Box::new(IrType::F32))),
            },
            SourceSpan::default(),
        ),
    );
    func3.add_instruction(
        "block3_a",
        Instruction::new(
            InstructionKind::Store {
                value: Value::new_literal(IrLiteralValue::Bool(true)),
                dest: Value::new_temporary(3, IrType::Pointer(Box::new(IrType::Bool))),
            },
            SourceSpan::default(),
        ),
    );
    func3.add_instruction(
        "block3_b",
        Instruction::new(
            InstructionKind::Load {
                src: Value::new_temporary(2, IrType::Pointer(Box::new(IrType::F32))),
                ty: IrType::F32,
            },
            SourceSpan::default(),
        )
        .with_result(Value::new_temporary(4, IrType::F32)),
    );

    module.add_function(func1);
    module.add_function(func2);
    module.add_function(func3);

    // Totale: 3 (func1) + 0 (func2) + 5 (func3) = 8 istruzioni
    assert_eq!(module.count_instructions(), 8);
}
