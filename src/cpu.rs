use crate::memory::memory;

pub struct CPU {
    pub regs: [u32; 16], // Array of general-purpose registers
    pub pc: u32, // Program counter
    pub cpsr: u32, // Current program status register
}