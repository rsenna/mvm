use crate::instruction::{InstructionFormat32, Word};
use crate::machine::Byte;
use crate::register::{RegisterValue64, Registers64, RegistersArray64};

pub type Instruction = InstructionFormat32;
pub type RegisterValue = RegisterValue64;
pub type Registers = Registers64;
pub type RegistersArray = RegistersArray64;

// TODO support variable amount of registers
#[derive(Debug)]
pub struct SimpleHart {
    registers: Registers,
    ram: Vec<Byte>,
}

pub trait Hart {
    fn fetch(&self) -> Instruction;
    fn read_byte_ram(&self, index: usize) -> Option<&Byte>;
}

impl SimpleHart {
    pub(crate) fn new(ram: Vec<Byte>) -> Self {
        let registers = Registers::new(ram.len());
        Self { registers, ram }
    }
}

impl Hart for SimpleHart {
    // This routine only works for 32 bits instructions
    fn fetch(&self) -> Instruction {
        let index = self.registers.pc as usize;
        let data: Word = self.ram[index] as u32
            | (self.ram[index + 1] as u32) << 8
            | (self.ram[index + 2] as u32) << 16
            | (self.ram[index + 3] as u32) << 24;
        Instruction::from(data)
    }

    fn read_byte_ram(&self, i: usize) -> Option<&Byte> {
        self.ram.get(i)
    }

    // TODO FINALLY use the disruptor pattern! EDIT: actually crossbeam
    //      each Hart (cpu) should process instructions in their own disruptor
    //      that way we can gain speed?

    /*    fn execute(&mut self, inst: u32) {
            let instruction = Instruction::from_u32(inst);

            // execute stage
            match instruction.opcode {
                0x13 => {
                    // addi
                    let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
                    self.regs[rd] = self.regs[rs1].wrapping_add(imm);
                }
                0x33 => {
                    // add
                    self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                }

                _ => {
                    dbg!(format!("Invalid opcode: {:#x}", opcode));
                }
            }
        }
    */
}
