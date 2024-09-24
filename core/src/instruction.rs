// Copyright ©️ 2024 Rogério Senna. All rights reserved.
//
// Licensed under the EUPL, Version 1.2 or – as soon they will be approved by
// the European Commission - subsequent versions of the EUPL (the "Licence");
// You may not use this work except in compliance with the Licence.
// You may obtain a copy of the Licence at:
//
// https://joinup.ec.europa.eu/software/page/eupl
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licence is distributed on an "AS IS" basis,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licence for the specific language governing permissions and
// limitations under the Licence.
//
use std::convert::Into;
use std::fmt::Debug;

use derive_more::Display;

use crate::isa_module::InstructionBaseSet;
use crate::memory::{InstructionLength, Word};
use crate::opcode::{
    BType32Bitfield, Funct3OpcodeTable, Funct3Uop, Funct7Uop, IFenceType32Bitfield, IType32Bitfield, Immediate32Uop,
    JType32Bitfield, Opcode7Bitfield, Opcode7Table, RType32Bitfield, SType32Bitfield,
};

// We use the term IALIGN (measured in bits) to refer to the instruction-address alignment
// constraint  the implementation enforces. IALIGN is 32 bits in the base ISA, but some ISA
// extensions, including the compressed ISA extension, relax IALIGN to 16 bits. IALIGN may not take
// on any value other than  16 or 32.
pub const IALIGN: InstructionLength = InstructionLength::Word;

// We use the term ILEN (measured in bits) to refer to the maximum instruction length supported by
// an/ implementation, and which is always a multiple of IALIGN. For implementations supporting only
// a base instruction set, ILEN is 32 bits. Implementations supporting longer instructions have
// larger/ values of ILEN.
pub const ILEN: InstructionLength = InstructionLength::Word;

// We use the term XLEN to
// refer to the width of an integer register in bits (either 32 or 64).
pub const XLEN: InstructionLength = InstructionLength::Word;

pub const ADDI: InstructionDescriptor32 = InstructionDescriptor32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Add Immediate",
    format: InstructionFormat32::integer_register_immediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::ADD)),
    funct7: None,
    imm11: None,
};

pub const SLTI: InstructionDescriptor32 = InstructionDescriptor32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: InstructionFormat32::integer_register_immediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::SLT)),
    funct7: None,
    imm11: None,
};

pub const SLTIU: InstructionDescriptor32 = InstructionDescriptor32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: InstructionFormat32::integer_register_immediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::SLTU)),
    funct7: None,
    imm11: None,
};

pub const ANDI: InstructionDescriptor32 = InstructionDescriptor32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: InstructionFormat32::integer_register_immediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::SLTU)), // TODO review
    funct7: None,
    imm11: None,
};

#[derive(Debug, PartialEq, Display)]
pub enum InstructionFormat32 {
    IntegerRegisterImmediate(IType32Bitfield),     // IType32
    IntegerRegisterRegister(RType32Bitfield),      // RType32
    UnconditionalJump(JType32Bitfield),            // JType32
    ConditionBranch(BType32Bitfield),              // BType32
    Load(IType32Bitfield),                         // IType32
    Store(SType32Bitfield),                        // SType32
    Fence(IFenceType32Bitfield),                   // IFenceType32
    ControlAndStatusRegister(IType32Bitfield),     // IType32
    TimeAndCounter(IType32Bitfield),               // IType32
    EnvironmentCallAndBreakpoint(RType32Bitfield), // RType32-like ... TODO review
}

pub struct InstructionDescriptor32 {
    base_set: InstructionBaseSet,
    name: &'static str,
    format: InstructionFormat32Getter,
    opcode: Option<Opcode7Table>,
    funct3: Option<Funct3Uop>,
    funct7: Option<Funct7Uop>,
    imm11: Option<Immediate32Uop>,
}

pub type InstructionFormat32Getter = fn(word: Word) -> InstructionFormat32;

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

impl InstructionFormat32 {
    pub fn get_opcode(self) -> Opcode7Bitfield {
        let opcode = match self {
            InstructionFormat32::IntegerRegisterImmediate(isa_word) => isa_word.opcode(),
            InstructionFormat32::IntegerRegisterRegister(isa_word) => isa_word.opcode(),
            InstructionFormat32::UnconditionalJump(isa_word) => isa_word.opcode(),
            InstructionFormat32::ConditionBranch(isa_word) => isa_word.opcode(),
            InstructionFormat32::Load(isa_word) => isa_word.opcode(),
            InstructionFormat32::Store(isa_word) => isa_word.opcode(),
            InstructionFormat32::Fence(isa_word) => isa_word.opcode(),
            InstructionFormat32::ControlAndStatusRegister(isa_word) => isa_word.opcode(),
            InstructionFormat32::TimeAndCounter(isa_word) => isa_word.opcode(),
            InstructionFormat32::EnvironmentCallAndBreakpoint(isa_word) => isa_word.opcode(),
        };

        opcode.into()
    }

    pub(crate) fn integer_register_immediate(word: Word) -> InstructionFormat32 {
        InstructionFormat32::IntegerRegisterImmediate(IType32Bitfield(word.into()))
    }

    pub(crate) fn integer_register_register(word: Word) -> InstructionFormat32 {
        InstructionFormat32::IntegerRegisterRegister(RType32Bitfield(word.into()))
    }

    pub(crate) fn unconditional_jump(word: Word) -> InstructionFormat32 {
        InstructionFormat32::UnconditionalJump(JType32Bitfield(word.into()))
    }

    pub(crate) fn condition_branch(word: Word) -> InstructionFormat32 {
        InstructionFormat32::ConditionBranch(BType32Bitfield(word.into()))
    }

    pub(crate) fn load(word: Word) -> InstructionFormat32 { InstructionFormat32::Load(IType32Bitfield(word.into())) }

    pub(crate) fn store(word: Word) -> InstructionFormat32 { InstructionFormat32::Store(SType32Bitfield(word.into())) }

    pub(crate) fn fence(word: Word) -> InstructionFormat32 {
        InstructionFormat32::Fence(IFenceType32Bitfield(word.into()))
    }

    pub(crate) fn control_and_status_register(word: Word) -> InstructionFormat32 {
        InstructionFormat32::ControlAndStatusRegister(IType32Bitfield(word.into()))
    }

    pub(crate) fn time_and_counter(word: Word) -> InstructionFormat32 {
        InstructionFormat32::TimeAndCounter(IType32Bitfield(word.into()))
    }

    pub(crate) fn environment_call_and_breakpoint(word: Word) -> InstructionFormat32 {
        InstructionFormat32::EnvironmentCallAndBreakpoint(RType32Bitfield(word.into()))
    }
}
