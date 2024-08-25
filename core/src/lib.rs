use std::collections::HashMap;
use deku::prelude::*;

// init memory as 128MB
pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

struct Cpu {
    // RISC-V has 32 registers
    regs: [u64; 32],
    // pc register contains the memory address of next instruction
    pc: u64,
    // memory, a byte-array. There is no memory in real CPU.
    dram: Vec<u8>,
}

pub enum SavedBy {
    None,
    Caller,
    Callee
}

trait RegisterX0 {
    const x0: u32;
}

pub struct Registers32 {
    data: [u32; 32]
}

pub struct RegisterMapItem {
    pos: i8,
    abi: &'static str,
    id: &'static str,
    saved_by: SavedBy
}

// TODO optimize
pub const REGISTER_MAP: HashMap<&str, RegisterMapItem> = HashMap::from([
    /* x00 */ (   "zero", RegisterMapItem { pos:  0, abi:    "zero", id:      "x01", saved_by: SavedBy::None   } ),
    /* x01 */ (     "ra", RegisterMapItem { pos:  1, abi:      "ra", id:      "x02", saved_by: SavedBy::Caller } ),
    /* x02 */ (     "sp", RegisterMapItem { pos:  2, abi:      "sp", id:      "x03", saved_by: SavedBy::Callee } ),
    /* x03 */ (     "gp", RegisterMapItem { pos:  3, abi:      "gp", id:      "x04", saved_by: SavedBy::None   } ),
    /* x04 */ (     "tp", RegisterMapItem { pos:  4, abi:      "tp", id:      "x05", saved_by: SavedBy::None   } ),
    /* x05 */ (     "t0", RegisterMapItem { pos:  5, abi:      "t0", id:      "x06", saved_by: SavedBy::Caller } ),
    /* x06 */ (     "t1", RegisterMapItem { pos:  6, abi:      "t1", id:      "x07", saved_by: SavedBy::Caller } ),
    /* x07 */ (     "t2", RegisterMapItem { pos:  7, abi:      "t2", id:      "x08", saved_by: SavedBy::Caller } ),
    /* x08 */ ("s0 / fp", RegisterMapItem { pos:  8, abi: "s0 / fp", id:      "x09", saved_by: SavedBy::Callee } ),
    /* x09 */ (     "s1", RegisterMapItem { pos:  9, abi:      "s1", id:      "x10", saved_by: SavedBy::Callee } ),
    /* x10 */ (     "a0", RegisterMapItem { pos: 10, abi:      "a0", id:      "x11", saved_by: SavedBy::Caller } ),
    /* x11 */ (     "a1", RegisterMapItem { pos: 11, abi:      "a1", id:      "x12", saved_by: SavedBy::Caller } ),
    /* x12 */ (     "a2", RegisterMapItem { pos: 12, abi:      "a2", id:      "x13", saved_by: SavedBy::Caller } ),
    /* x13 */ (     "a3", RegisterMapItem { pos: 13, abi:      "a3", id:      "x14", saved_by: SavedBy::Caller } ),
    /* x14 */ (     "a4", RegisterMapItem { pos: 14, abi:      "a4", id:      "x15", saved_by: SavedBy::Caller } ),
    /* x15 */ (     "a5", RegisterMapItem { pos: 15, abi:      "a5", id:      "x16", saved_by: SavedBy::Caller } ),
    /* x16 */ (     "a6", RegisterMapItem { pos: 16, abi:      "a6", id:      "x17", saved_by: SavedBy::Caller } ),
    /* x17 */ (     "a7", RegisterMapItem { pos: 17, abi:      "a7", id:      "x18", saved_by: SavedBy::Caller } ),
    /* x18 */ (     "s2", RegisterMapItem { pos: 18, abi:      "s2", id:      "x19", saved_by: SavedBy::Callee } ),
    /* x19 */ (     "s3", RegisterMapItem { pos: 19, abi:      "s3", id:      "x20", saved_by: SavedBy::Callee } ),
    /* x20 */ (     "s4", RegisterMapItem { pos: 20, abi:      "s4", id:      "x21", saved_by: SavedBy::Callee } ),
    /* x21 */ (     "s5", RegisterMapItem { pos: 21, abi:      "s5", id:      "x22", saved_by: SavedBy::Callee } ),
    /* x22 */ (     "s6", RegisterMapItem { pos: 22, abi:      "s6", id:      "x23", saved_by: SavedBy::Callee } ),
    /* x23 */ (     "s7", RegisterMapItem { pos: 23, abi:      "s7", id:      "x24", saved_by: SavedBy::Callee } ),
    /* x24 */ (     "s8", RegisterMapItem { pos: 24, abi:      "s8", id:      "x25", saved_by: SavedBy::Callee } ),
    /* x25 */ (     "s9", RegisterMapItem { pos: 25, abi:      "s9", id:      "x26", saved_by: SavedBy::Callee } ),
    /* x26 */ (    "s10", RegisterMapItem { pos: 26, abi:     "s10", id:      "x27", saved_by: SavedBy::Callee } ),
    /* x27 */ (    "s11", RegisterMapItem { pos: 27, abi:     "s11", id:      "x28", saved_by: SavedBy::Callee } ),
    /* x28 */ (     "t3", RegisterMapItem { pos: 28, abi:      "t3", id:      "x29", saved_by: SavedBy::Caller } ),
    /* x29 */ (     "t4", RegisterMapItem { pos: 29, abi:      "t4", id:      "x30", saved_by: SavedBy::Caller } ),
    /* x30 */ (     "t5", RegisterMapItem { pos: 30, abi:      "t5", id:      "x31", saved_by: SavedBy::Caller } ),
    /* x31 */ (     "t6", RegisterMapItem { pos: 31, abi:      "t6", id:      "x32", saved_by: SavedBy::Caller } )
]);

impl Registers32 {
    pub fn get(&self, reg: &str) {
        let index: Option<i8> = REGISTER_MAP.iter()
            .find_map(|(str, it)| if str == &reg { Some(it.pos) } else { None });
        self.data[index];
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct EncodingType {
    #[deku(bits = 7)]
    opcode: u8,
    data: EncodingData
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u32", bits = 25)]
pub enum EncodingData {
    #[deku(id = 0x00)]
    Raw32(u32),
    #[deku(id = 0x01)]
    TypeR32 {
        #[deku(bits = 5)]
        rd: u8,
        #[deku(bits = 3)]
        funct3: u8,
        #[deku(bits = 5)]
        rs1: u8,
        #[deku(bits = 5)]
        rs2: u8,
        #[deku(bits = 7)]
        funct7: u8
    }
}

struct Instruction<> {
    name: &str,
    opcode: B7,

}

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_SIZE - 1;
        Self {regs, pc: 0, dram: code}
    }

    fn execute(&mut self, inst: u32) {
        // decode as R-type
        // TODO: improve how we deal with different Basic Instruction Encoding Formats
        //       couldn't we use a struct here instead?
        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as usize;
        let rs1 = ((inst >> 15) & 0x1f) as usize;
        let rs2 = ((inst >> 20) & 0x1f) as usize;
        let funct3 = (inst >> 12) & 0x7;
        let funct7 = (inst >> 25) & 0x7f;

        // x0 is hardwired zero
        self.regs[0] = 0;

        // execute stage
        match opcode {
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

    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        let inst = self.dram[index] as u32
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
        return inst;
    }
}
