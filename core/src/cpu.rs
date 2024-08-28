use crate::register::Registers64;

// init memory as 128MB
pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

struct Cpu {
    // RISC-V has 32 registers
    // TODO support variable amount of registers
    registers: Registers64,

    // A byte-array representing "infinite memory".
    // There is no memory in real CPU. (duh)
    dram: Vec<u8>,
}

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[0] = 0;
        regs[2] = DRAM_SIZE - 1;

        let result = Self {
            registers: Registers64 { data: regs },
            dram: code
        };
        result
    }

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
    fn fetch(&self) -> u32 {
        let index = 0 /*self.pc*/ as usize;
        let inst = self.dram[index] as u32
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
        return inst;
    }
}
