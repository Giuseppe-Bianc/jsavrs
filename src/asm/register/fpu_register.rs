/// Registri x87 FPU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FPURegister {
    St0,
    St1,
    St2,
    St3,
    St4,
    St5,
    St6,
    St7,
}