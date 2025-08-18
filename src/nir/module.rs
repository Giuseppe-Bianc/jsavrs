use std::fmt;
use crate::nir::Function;

#[derive(Debug, Clone, PartialEq)]
pub enum DataLayout {
    LinuxX86_64,
    LinuxAArch64,
    WindowsX86_64,
    MacOSX86_64,
    FreeBSDX86_64,
    NetBSDX86_64,
    OpenBSDX86_64,
    DragonFlyX86_64,
}

impl fmt::Display for DataLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataLayout::LinuxX86_64 => write!(f, "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"),
            DataLayout::LinuxAArch64 => write!(f, "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"),
            DataLayout::WindowsX86_64 => write!(f, "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"),
            DataLayout::MacOSX86_64 => write!(f, "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"),
            DataLayout::FreeBSDX86_64 => write!(f, "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"),
            DataLayout::NetBSDX86_64 => write!(f, "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"),
            DataLayout::OpenBSDX86_64 => write!(f, "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"),
            DataLayout::DragonFlyX86_64 => write!(f, "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetTriple {
    X86_64UnknownLinuxGnu,
    X86_64PcWindowsGnu,
    X86_64AppleDarwin,
    AArch64UnknownLinuxGnu,
    AArch64AppleDarwin,
    AArch64PcWindowsGnu,
    I686PcWindowsGnu,
    I686UnknownLinuxGnu,
    Wasm32UnknownEmscripten,
}

impl fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetTriple::X86_64UnknownLinuxGnu => write!(f, "x86_64-unknown-linux-gnu"),
            TargetTriple::X86_64PcWindowsGnu => write!(f, "x86_64-pc-windows-gnu"),
            TargetTriple::X86_64AppleDarwin => write!(f, "x86_64-apple-darwin"),
            TargetTriple::AArch64UnknownLinuxGnu => write!(f, "aarch64-unknown-linux-gnu"),
            TargetTriple::AArch64AppleDarwin => write!(f, "aarch64-apple-darwin"),
            TargetTriple::AArch64PcWindowsGnu => write!(f, "aarch64-pc-windows-gnu"),
            TargetTriple::I686PcWindowsGnu => write!(f, "i686-pc-windows-gnu"),
            TargetTriple::I686UnknownLinuxGnu => write!(f, "i686-unknown-linux-gnu"),
            TargetTriple::Wasm32UnknownEmscripten => write!(f, "wasm32-unknown-emscripten"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
    pub data_layout: DataLayout,
    pub target_triple: TargetTriple,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            data_layout: DataLayout::LinuxX86_64,
            target_triple: TargetTriple::X86_64UnknownLinuxGnu,
        }
    }
    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
    pub fn set_data_layout(&mut self, layout: DataLayout) {
        self.data_layout = layout;
    }
    pub fn set_target_triple(&mut self, triple: TargetTriple) {
        self.target_triple = triple;
    }
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }
    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.iter_mut().find(|f| f.name == name)
    }
    pub fn get_functions(&self) -> &[Function] {
        &self.functions
    }
    pub fn get_data_layout(&self) -> &DataLayout {
        &self.data_layout
    }
    pub fn get_target_triple(&self) -> &TargetTriple {
        &self.target_triple
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module {} {{", self.name)?;
        writeln!(f, "data_layout = {};", self.data_layout)?;
        writeln!(f, "target_triple = {};", self.target_triple)?;
        for function in &self.functions {
            writeln!(f, "{}", function)?;
        }
        write!(f, "}}")
    }
}