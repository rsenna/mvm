use std::fmt::{Debug, Display};

use num_traits::FromPrimitive;
use strum_macros::Display;

use crate::etc::InstructionLength;
use crate::opcode;
use crate::opcode::{Funct3, Funct3Opcode, OpcodeMapID, INVALID_OPCODE7};

// We use the term IALIGN (measured in bits) to refer to the instruction-address alignment constraint
// the implementation enforces. IALIGN is 32 bits in the base ISA, but some ISA extensions, including
// the compressed ISA extension, relax IALIGN to 16 bits. IALIGN may not take on any value other than
// 16 or 32.
pub const IALIGN: InstructionLength = InstructionLength::Word;

// We use the term ILEN (measured in bits) to refer to the maximum instruction length supported by an
// implementation, and which is always a multiple of IALIGN. For implementations supporting only a
// base instruction set, ILEN is 32 bits. Implementations supporting longer instructions have larger
// values of ILEN.
pub const ILEN: InstructionLength = InstructionLength::Word;

// We use the term XLEN to
// refer to the width of an integer register in bits (either 32 or 64).
pub const XLEN: InstructionLength = InstructionLength::Word;

pub enum InstructionBaseSet {
    RV32I = 0,
}

pub const ADDI: Instruction32 = Instruction32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Add Immediate",
    format: get_integer_register_immediate,
    opcode: OpcodeMapID::OpImmediate,
    funct3: opcode::get_funct_3(Funct3Opcode::ADD),
    funct7: None,
    imm11: None,
};

pub const SLTI: Instruction32 = Instruction32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: get_integer_register_immediate,
    opcode: OpcodeMapID::ArithmeticImmediate,
    funct3: opcode::get_funct_3(Funct3Opcode::Slt as Byte),
    funct7: None,
    imm11: None,
};

pub const SLTIU: Instruction32 = Instruction32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: get_integer_register_immediate,
    opcode: OpcodeMapID::ArithmeticImmediate,
    funct3: opcode::get_funct_3(Funct3Opcode::Sltu as Byte),
    funct7: None,
    imm11: None,
};

pub const ANDI: Instruction32 = Instruction32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: get_integer_register_immediate,
    opcode: OpcodeMapID::ArithmeticImmediate,
    funct3: opcode::get_funct_3(Funct3Opcode::Sltu as Byte),
    funct7: None,
    imm11: None,
};

pub type InstructionFormat32Getter = fn(word: Word) -> InstructionFormat32;

const fn get_integer_register_immediate(word: Word) -> InstructionFormat32 {
    InstructionFormat32::IntegerRegisterImmediate(IType32(word))
}

pub struct Instruction32 {
    base_set: InstructionBaseSet,
    name: &'static str,
    format: InstructionFormat32Getter,
    opcode: OpcodeMapID,
    funct3: Option<Funct3>,
    funct7: Option<Funct7Bits>,
    imm11: Option<Immediate12Bits>,
}

#[derive(Debug, PartialEq, Display)]
pub enum InstructionFormat32 {
    IntegerRegisterImmediate(IType32),
    IntegerRegisterRegister(RType32),
    UnconditionalJump(JType32),
    ConditionBranch(BType32),
    Load(IType32),
    Store(SType32),
    Fence(IType32),                         // kind of i-type...
    ControlAndStatusRegister(IType32),
    TimeAndCounter(IType32),
    EnvironmentCallAndBreakpoint(RType32),  // kind of r-type...
}

// TODO variable instruction length;
//      see https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf page 5

// impl From<u8> for Opcode7 {
//     fn from(value: u8) -> Self {
//         Opcode7(value)
//     }
// }
//
// impl From<Opcode7> for u8 {
//     fn from(Opcode7(value): Opcode7) -> Self {
//         value
//     }
// }

impl From<Word> for InstructionFormat32 {
    fn from(value: Word) -> Self {
        let r_type_value = RType32(value);
        let opcode = r_type_value.opcode();
        let opcode_id = OpcodeMapID::from_u8(opcode).unwrap_or(OpcodeMapID::Null);

        match opcode_id {
            OpcodeMapID::ArithmeticImmediate =>
                InstructionFormat32::IntegerRegisterImmediate(IType32(value)),
            OpcodeMapID::JumpAndLinkRegister =>
                OpcodeMapID::Load | InstructionFormat32:: =>
                InstructionFormat32::I(IType32(value)),
            OpcodeMapID::Arithmetic =>
                InstructionFormat32::R(r_type_value), // we already have this one set above...
            OpcodeMapID::JumpAndLink =>
                InstructionFormat32::J(JType32(value)),
            OpcodeMapID::LoadUpperImmediate | OpcodeMapID::AddUpperImmediateToPC =>
                InstructionFormat32::U(UType32(value)),
            OpcodeMapID::Branch =>
                InstructionFormat32::B(BType32(value)),
            OpcodeMapID::Store =>
                InstructionFormat32::S(SType32(value)),
            _ =>
                InstructionFormat32::Invalid(value),
        }
    }
}

impl InstructionFormat32 {
    pub fn get_opcode(&self) -> OpcodeBits {
        match self {
            InstructionFormat32::R(r) => r.opcode(),
            InstructionFormat32::I(i) => i.opcode(),
            InstructionFormat32::S(s) => s.opcode(),
            InstructionFormat32::B(b) => b.opcode(),
            InstructionFormat32::U(u) => u.opcode(),
            InstructionFormat32::J(j) => j.opcode(),
            _ => INVALID_OPCODE7,
        }
    }

    pub fn get_imm(&self) -> Option<Immediate32Bits> {
        let mut result = Immediate32Bits(0);

        match self {
            InstructionFormat32::I(i) => {
                result.set_bit0(i.imm_b20());
                result.set_seq_b4_1(i.imm_b24_21());
                result.set_seq_b10_5(i.imm_b30_25());
                result.set_bit31(i.imm_b31());
            }
            InstructionFormat32::S(s) => {
                result.set_bit0(s.imm_b7());
                result.set_seq_b4_1(s.imm_b11_8());
                result.set_seq_b10_5(s.imm_b30_25());
                result.set_bit31(s.imm_b31());
            }
            InstructionFormat32::B(b) => {
                result.set_seq_b4_1(b.imm_b11_8());
                result.set_seq_b10_5(b.imm_b30_25());
                result.set_bit11(b.imm_b7());
                result.set_bit31(b.imm_b31());
            }
            InstructionFormat32::U(u) => {
                result.set_seq_b19_12(u.imm_b19_12());
                result.set_seq_b30_20(u.imm_b30_20());
                result.set_bit31(u.imm_b31());
            }
            InstructionFormat32::J(j) => {
                result.set_seq_b4_1(j.imm_b24_21());
                result.set_seq_b10_5(j.imm_b30_25());
                result.set_bit11(j.imm_b20());
                result.set_seq_b19_12(j.imm_b19_12());
                result.set_bit31(j.imm_b31());
            }
            _ => return None,
        }

        Some(result)
    }
}
