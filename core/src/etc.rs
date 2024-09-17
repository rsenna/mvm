pub type Byte = u8;
pub type HalfWord   = u16;
pub type Word = u32;
pub type DoubleWord = u64;

pub enum InstructionLength {
    Byte = 8,
    HalfWord = 16,
    Word = 32,
    DoubleWord = 64
}
