/// Registri di controllo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlRegister {
    Cr0,
    Cr2,
    Cr3,
    Cr4,
    Cr8,
}
