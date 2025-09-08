use super::{Function, ScopeId};
use std::fmt;
use std::sync::Arc;

/// Descrive il layout dei dati per diversi target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        let layout = match self {
            DataLayout::LinuxX86_64 => "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            DataLayout::LinuxAArch64 => "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
            DataLayout::WindowsX86_64 => "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            DataLayout::MacOSX86_64 => "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            DataLayout::FreeBSDX86_64 => "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            DataLayout::NetBSDX86_64 => "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            DataLayout::OpenBSDX86_64 => "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
            DataLayout::DragonFlyX86_64 => "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128",
        };
        f.write_str(layout)
    }
}

/// Identifica la tripletta di destinazione (arch-os-environment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        let triple = match self {
            TargetTriple::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            TargetTriple::X86_64PcWindowsGnu => "x86_64-pc-windows-gnu",
            TargetTriple::X86_64AppleDarwin => "x86_64-apple-darwin",
            TargetTriple::AArch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            TargetTriple::AArch64AppleDarwin => "aarch64-apple-darwin",
            TargetTriple::AArch64PcWindowsGnu => "aarch64-pc-windows-gnu",
            TargetTriple::I686PcWindowsGnu => "i686-pc-windows-gnu",
            TargetTriple::I686UnknownLinuxGnu => "i686-unknown-linux-gnu",
            TargetTriple::Wasm32UnknownEmscripten => "wasm32-unknown-emscripten",
        };
        f.write_str(triple)
    }
}

/// Rappresenta un modulo IR (Intermediate Representation).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Module {
    pub name: Arc<str>,
    pub functions: Vec<Function>,
    pub data_layout: DataLayout,
    pub target_triple: TargetTriple,
    root_scope: Option<ScopeId>, // Root scope ID for the module settable only at creation
}

impl Module {
    /// Crea un nuovo modulo con nome specificato e impostazioni predefinite.
    pub fn new(name: impl Into<Arc<str>>, root_scope: Option<ScopeId>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::new(),
            data_layout: DataLayout::LinuxX86_64,
            target_triple: TargetTriple::X86_64UnknownLinuxGnu,
            root_scope,
        }
    }

    /// Aggiunge una funzione al modulo.
    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    /// Imposta il layout dei dati.
    pub fn set_data_layout(&mut self, layout: DataLayout) {
        self.data_layout = layout;
    }

    /// Imposta la tripletta di destinazione.
    pub fn set_target_triple(&mut self, triple: TargetTriple) {
        self.target_triple = triple;
    }

    /// Cerca una funzione per nome (riferimento immutabile).
    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Cerca una funzione per nome (riferimento mutabile).
    pub fn get_function_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.iter_mut().find(|f| f.name == name)
    }

    /// Restituisce tutte le funzioni del modulo.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Restituisce il layout dei dati.
    pub fn data_layout(&self) -> &DataLayout {
        &self.data_layout
    }

    /// Restituisce la tripletta di destinazione.
    pub fn target_triple(&self) -> &TargetTriple {
        &self.target_triple
    }

    /// Restituisce il nome del modulo.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module {} {{", self.name)?;
        writeln!(f, "  data_layout = \"{}\";", self.data_layout)?;
        writeln!(f, "  target_triple = \"{}\";", self.target_triple)?;
        /*if let Some(rs) = self.root_scope {
            writeln!(f, "  root_scope = \"{}\";", rs)?;
        } else {
            writeln!(f, "  // root_scope: none")?;
        }*/

        if self.functions.is_empty() {
            writeln!(f, "  // No functions")?;
        } else {
            for function in &self.functions {
                let s = function.to_string();
                for line in s.trim_end_matches('\n').lines() {
                    writeln!(f, "  {line}")?;
                }
            }
        }

        write!(f, "}}")
    }
}
