use crate::{asm::{Abi, AssemblyFile}, error::compile_error::CompileError, ir::{Module, TargetTriple}};


#[allow(dead_code)]
pub struct AsmGen {
    ir: Module,
    errors: Vec<CompileError>,
    assembly_file: AssemblyFile
}

impl AsmGen {
    pub fn new(ir: Module) -> Self {
        let target_triple = ir.target_triple;
        Self {
            ir,
            errors: Vec::new(),
            assembly_file: AssemblyFile::new(Self::target_triple_to_abi(target_triple)),
        }
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
            | TargetTriple::Wasm32UnknownEmscripten => todo!("ABI for target triple {:?} not supported yet", target_triple),
        }
    }

    pub fn gen_asm(self) -> (AssemblyFile, Vec<CompileError>) {
        println!("Generating assembly for abi: {:?}", self.assembly_file.abi());
        (self.assembly_file, self.errors)
    }
}