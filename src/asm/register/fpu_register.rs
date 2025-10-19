/// x87 Floating-Point Unit register stack.
///
/// Eight 80-bit extended precision registers (ST0-ST7) organized as a stack.
/// ST0 is top-of-stack (TOS). Operations push/pop values, rotating the stack
/// pointer. Provides higher precision than IEEE 754 double (64-bit) for
/// intermediate calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FPURegister {
    /// ST(0) - Top of stack, primary operand for most operations.
    St0,
    St1,
    St2,
    St3,
    St4,
    St5,
    St6,
    /// ST(7) - Bottom of stack when full.
    St7,
}
