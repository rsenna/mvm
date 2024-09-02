use phf::phf_map;
use std::iter::Iterator;

#[derive(Debug, PartialEq)]
pub struct RegisterType {
    pos: i32,
    abi: &'static str,
    id: &'static str,
    saved_by: SavedBy,
}

#[derive(Debug, PartialEq)]
pub enum SavedBy {
    None,
    Caller,
    Callee,
}

type RT = RegisterType;

const PC: RT = RT { pos: -2, abi: "pc", id: "", saved_by: SavedBy::None };
const ZERO: RT = RT { pos: -1, abi: "zero", id: "x01", saved_by: SavedBy::None };
const RA: RT = RT { pos: 0, abi: "ra", id: "x02", saved_by: SavedBy::Caller };
const SP: RT = RT { pos: 1, abi: "sp", id: "x03", saved_by: SavedBy::Callee };
const GP: RT = RT { pos: 2, abi: "gp", id: "x04", saved_by: SavedBy::None };
const TP: RT = RT { pos: 3, abi: "tp", id: "x05", saved_by: SavedBy::None };
const T0: RT = RT { pos: 4, abi: "t0", id: "x06", saved_by: SavedBy::Caller };
const T1: RT = RT { pos: 5, abi: "t1", id: "x07", saved_by: SavedBy::Caller };
const T2: RT = RT { pos: 6, abi: "t2", id: "x08", saved_by: SavedBy::Caller };
const S0_FP: RT = RT { pos: 7, abi: "s0 / fp", id: "x09", saved_by: SavedBy::Callee };
const S1: RT = RT { pos: 8, abi: "s1", id: "x10", saved_by: SavedBy::Callee };
const A0: RT = RT { pos: 9, abi: "a0", id: "x11", saved_by: SavedBy::Caller };
const A1: RT = RT { pos: 10, abi: "a1", id: "x12", saved_by: SavedBy::Caller };
const A2: RT = RT { pos: 11, abi: "a2", id: "x13", saved_by: SavedBy::Caller };
const A3: RT = RT { pos: 12, abi: "a3", id: "x14", saved_by: SavedBy::Caller };
const A4: RT = RT { pos: 13, abi: "a4", id: "x15", saved_by: SavedBy::Caller };
const A5: RT = RT { pos: 14, abi: "a5", id: "x16", saved_by: SavedBy::Caller };
const A6: RT = RT { pos: 15, abi: "a6", id: "x17", saved_by: SavedBy::Caller };
const A7: RT = RT { pos: 16, abi: "a7", id: "x18", saved_by: SavedBy::Caller };
const S2: RT = RT { pos: 17, abi: "s2", id: "x19", saved_by: SavedBy::Callee };
const S3: RT = RT { pos: 18, abi: "s3", id: "x20", saved_by: SavedBy::Callee };
const S4: RT = RT { pos: 19, abi: "s4", id: "x21", saved_by: SavedBy::Callee };
const S5: RT = RT { pos: 20, abi: "s5", id: "x22", saved_by: SavedBy::Callee };
const S6: RT = RT { pos: 21, abi: "s6", id: "x23", saved_by: SavedBy::Callee };
const S7: RT = RT { pos: 22, abi: "s7", id: "x24", saved_by: SavedBy::Callee };
const S8: RT = RT { pos: 23, abi: "s8", id: "x25", saved_by: SavedBy::Callee };
const S9: RT = RT { pos: 24, abi: "s9", id: "x26", saved_by: SavedBy::Callee };
const S10: RT = RT { pos: 25, abi: "s10", id: "x27", saved_by: SavedBy::Callee };
const S11: RT = RT { pos: 26, abi: "s11", id: "x28", saved_by: SavedBy::Callee };
const T3: RT = RT { pos: 27, abi: "t3", id: "x29", saved_by: SavedBy::Caller };
const T4: RT = RT { pos: 28, abi: "t4", id: "x30", saved_by: SavedBy::Caller };
const T5: RT = RT { pos: 29, abi: "t5", id: "x31", saved_by: SavedBy::Caller };
const T6: RT = RT { pos: 30, abi: "t6", id: "x32", saved_by: SavedBy::Caller };

static REGISTERS_BASE_MAP: phf::Map<&'static str, &RT> = phf_map! {
    "pc" => &PC, "zero" => &ZERO, "ra" => &RA, "sp" => &SP, "gp" => &GP, "tp" => &TP,
    "t0" => &T0, "t1" => &T1, "t2" => &T2,
    "s0 / fp" => &S0_FP, "s1" => &S1,
    "a0" => &A0, "a1" => &A1, "a2" => &A2, "a3" => &A3, "a4" => &A4, "a5" => &A5, "a6" => &A6, "a7" => &A7,
    "s2" => &S2, "s3" => &S3, "s4" => &S4, "s5" => &S5, "s6" => &S6, "s7" => &S7, "s8" => &S8, "s9" => &S9,
    "s10" => &S10, "s11" => &S11,
    "t3" => &T3, "t4" => &T4, "t5" => &T5, "t6" => &T6,
};

pub const REGISTERS_COUNT: usize = 30; // ignore PC and ZERO

pub type RegisterValue64 = u64;
pub type RegistersArray64 = [RegisterValue64; REGISTERS_COUNT];

#[derive(Debug, PartialEq)]
pub struct Registers64 {
    pub pc: RegisterValue64,
    pub array: RegistersArray64,
}

impl Registers64 {
    pub fn new(ram_size: usize) -> Self {
        let pc: RegisterValue64 = 0;

        let mut array: RegistersArray64 = [0; REGISTERS_COUNT];

        array[0] = 0;
        array[2] = (ram_size - 1) as RegisterValue64;

        Self { pc, array }
    }
}

impl RegisterType {
    pub fn get_type_by_name(name: &str) -> Option<&Self> {
        REGISTERS_BASE_MAP
            .into_iter()
            .find(|(&s, _)| s == name)
            .map(|(_, &r)| r)
    }

    pub fn get_type_by_id(id: &str) -> Option<&Self> {
        REGISTERS_BASE_MAP
            .into_iter()
            .find(|(_, &r)| r.id == id)
            .map(|(_, &r)| r)
    }
}

impl Registers64 {
    pub fn get(&self, reg: RT) -> RegisterValue64 {
        match reg {
            PC => self.pc,
            ZERO => 0,
            _ => self.array[reg.pos as usize]
        }
    }

    pub fn set(&mut self, rt: RT, v: RegisterValue64) {
        self.array[rt.pos as usize] = v
    }
}
