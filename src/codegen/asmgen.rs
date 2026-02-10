use crate::{
    asm::{Abi, AssemblyFile},
    error::compile_error::CompileError,
    ir::{Module, TargetTriple},
};

#[allow(dead_code)]
/// Assembly code generator for translating IR to target-specific assembly.
///
/// `AsmGen` consumes an intermediate representation [`Module`] and produces an
/// [`AssemblyFile`] configured for the appropriate target ABI. Errors encountered
/// during code generation are accumulated in the `errors` vector.
///
/// # Lifecycle
///
/// 1. Create an instance via [`AsmGen::new`] with an IR module.
/// 2. Call [`AsmGen::gen_asm`] to consume the generator and retrieve output.
///
/// # Example
///
/// ```ignore
/// let generator = AsmGen::new(ir_module);
/// let (assembly, errors) = generator.gen_asm();
/// ```
pub struct AsmGen {
    /// The intermediate representation module to be translated into assembly.
    /// Contains the target triple, function definitions, and global data.
    ir: Module,
    /// Accumulated compilation errors encountered during assembly generation.
    /// Empty if generation succeeds without issues.
    errors: Vec<CompileError>,
    /// The output assembly file being constructed, configured for the target ABI.
    assembly_file: AssemblyFile,
}

impl AsmGen {
    #[must_use]
    pub fn new(ir: Module) -> Self {
        let target_triple = ir.target_triple;
        Self { ir, errors: Vec::new(), assembly_file: AssemblyFile::new(Self::target_triple_to_abi(target_triple)) }
    }

    fn target_triple_to_abi(target_triple: TargetTriple) -> Abi {
        match target_triple {
            TargetTriple::X86_64UnknownLinuxGnu => Abi::SYSTEM_V_LINUX,
            TargetTriple::X86_64PcWindowsGnu => Abi::WINDOWS,
            TargetTriple::X86_64AppleDarwin => Abi::SYSTEM_V_MACOS,
            TargetTriple::AArch64UnknownLinuxGnu
            | TargetTriple::AArch64AppleDarwin
            | TargetTriple::AArch64PcWindowsGnu
            | TargetTriple::I686PcWindowsGnu
            | TargetTriple::I686UnknownLinuxGnu
            | TargetTriple::Wasm32UnknownEmscripten => {
                todo!("ABI for target triple {:?} not supported yet", target_triple)
            }
        }
    }

    /// Consumes the generator and produces the final assembly output.
    ///
    /// This method finalizes assembly generation, returning the constructed
    /// [`AssemblyFile`] along with any errors encountered during the process.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - `AssemblyFile`: The generated assembly code configured for the target ABI.
    /// - `Vec<CompileError>`: Any errors encountered during generation. Empty if
    ///   generation succeeded without issues.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let generator = AsmGen::new(ir_module);
    /// let (assembly, errors) = generator.gen_asm();
    ///
    /// if errors.is_empty() {
    ///     // Write assembly to file or pass to assembler
    /// } else {
    ///     for error in &errors {
    ///         eprintln!("Compilation error: {}", error);
    ///     }
    /// }
    /// ```
    #[must_use]
    pub fn gen_asm(mut self) -> (AssemblyFile, Vec<CompileError>) {
        println!("Generating assembly for abi: {:?}", self.assembly_file.abi());
        self.ir.functions().iter().for_each(|func| {
            let func_name = &func.name;
            println!("Processing function: {func_name}");
            if func_name.as_ref() == "main" {
                println!("Found main function, adding global label");
                self.assembly_file.text_sec_add_global_label(func_name.to_string());
            }
            self.assembly_file.text_sec_add_label(func_name.to_string());
        });
        (self.assembly_file.clone(), self.errors)
    }
}
