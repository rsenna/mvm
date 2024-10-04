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

use crate::bitfield::{
    BType32Bitfield, Funct3, Funct3OpcodeTable, Funct3Uop, Funct7, Funct7Uop, IFenceType32Bitfield, IType32Bitfield,
    Immediate32Uop, JType32Bitfield, Opcode7, Opcode7Table, RType32Bitfield, SType32Bitfield,
};
use crate::isa_module::InstructionBaseSet;
use crate::memory::{InstructionLength, Word};
use std::convert::Into;
use std::fmt::Debug;

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
    format: InstructionFormat::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::ADD)),
    funct7: None,
    imm11: None,
};

pub const SLTI: InstructionDescriptor32 = InstructionDescriptor32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: InstructionFormat::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::SLT)),
    funct7: None,
    imm11: None,
};

pub const SLTIU: InstructionDescriptor32 = InstructionDescriptor32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: InstructionFormat::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::SLTU)),
    funct7: None,
    imm11: None,
};

pub const ANDI: InstructionDescriptor32 = InstructionDescriptor32 {
    base_set: InstructionBaseSet::RV32I,
    name: "Set Less Than Immediate",
    format: InstructionFormat::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Uop::Opcode(Funct3OpcodeTable::SLTU)), // TODO review
    funct7: None,
    imm11: None,
};

pub type InstructionFormat32Getter = fn(word: Word) -> InstructionFormat32;

pub trait GenericInstructionFormat {
    fn get_opcode(&self) -> Opcode7;
    fn is(&self, descr: InstructionDescriptor32) -> bool; // TODO review fixed Opcode7 type, is it always 7 bits?
}

#[derive(Debug, PartialEq)]
pub struct InstructionDescriptor32 {
    base_set: InstructionBaseSet,
    name: &'static str,
    format: InstructionFormat,
    opcode: Option<Opcode7Table>,
    funct3: Option<Funct3Uop>,
    funct7: Option<Funct7Uop>,
    imm11: Option<Immediate32Uop>,
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum InstructionFormat {
    IntegerRegisterImmediate,     // IType32
    IntegerRegisterRegister,      // RType32
    UnconditionalJump,            // JType32
    ConditionBranch,              // BType32
    Load,                         // IType32
    Store,                        // SType32
    Fence,                        // IFenceType32
    ControlAndStatusRegister,     // IType32
    TimeAndCounter,               // IType32
    EnvironmentCallAndBreakpoint, // RType32-like ... TODO review
}

#[derive(Clone, Debug, PartialEq)]
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

#[repr(C)]
pub union InstructionFormat32Union {
    pub raw: Word,
    pub integer_register_immediate: IType32Bitfield,
    pub integer_register_register: RType32Bitfield,
    pub unconditional_jump: JType32Bitfield,
    pub condition_branch: BType32Bitfield,
    pub load: IType32Bitfield,
    pub store: SType32Bitfield,
    pub fence: IFenceType32Bitfield,
    pub control_and_status_register: IType32Bitfield,
    pub time_and_counter: IType32Bitfield,
    pub environment_call_and_breakpoint: RType32Bitfield,
}

// TODO variable instruction length;
//      see https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf page 5

impl GenericInstructionFormat for InstructionFormat32 {
    fn get_opcode(&self) -> Opcode7 {
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

        opcode
    }

    fn is(&self, descr: InstructionDescriptor32) -> bool {
        let check_o7f3f7 = |opcode: Opcode7, funct3: Funct3, funct7: Funct7| -> bool {
            descr.opcode == Some(opcode.into())
                && descr.funct3 == Some(funct3.into())
                && descr.funct7 == Some(funct7.into())
        };

        let check_o7f3 = |opcode: Opcode7, funct3: Funct3| -> bool {
            descr.opcode == Some(opcode.into()) && descr.funct3 == Some(funct3.into()) && descr.funct7 == None
        };

        match self {
            InstructionFormat32::IntegerRegisterImmediate(i_type)
            | InstructionFormat32::UnconditionalJump(i_type)
            | InstructionFormat32::ControlAndStatusRegister(i_type)
            | InstructionFormat32::TimeAndCounter(i_type)
            | InstructionFormat32::Load(i_type) => check_o7f3f7(i_type.opcode(), i_type.funct3(), i_type.funct7()),

            InstructionFormat32::IntegerRegisterRegister(r_type)
            | InstructionFormat32::EnvironmentCallAndBreakpoint(r_type) => {
                check_o7f3f7(r_type.opcode(), r_type.funct3(), r_type.funct7())
            }

            InstructionFormat32::ConditionBranch(b_type) => check_o7f3(b_type.opcode(), b_type.funct3()),
            InstructionFormat32::Store(s_type) => check_o7f3f7(s_type.opcode(), s_type.funct3(), s_type.funct7()),
            InstructionFormat32::Fence(i_fence_type) => check_o7f3f7(
                i_fence_type.opcode(),
                i_fence_type.funct3(),
                i_fence_type.funct7(),
            ),
        }
    }
}

// TODO: remove, mixes fetch and execute logic
impl InstructionFormat32Union {
    /// Use the opcode, funct3, and funct7 fields to determine the instruction type.
    /// We assume instr to be RType32Bitfield, so we can access all those fields.
    /// Later we will find out the actual type of instr.
    pub fn match_instr(&self, descr: InstructionDescriptor32) -> Option<InstructionFormat32> {
        let descr_opcode = descr
            .opcode
            .map(|it| it.into())
            .unwrap_or(Opcode7Table::Zero);
        let descr_funct3 = descr.funct3.map(|it| it.into()).unwrap_or(Funct3Uop::Zero);
        let descr_funct7 = descr
            .funct7
            .map(|it| it.into())
            .unwrap_or(Funct7Uop::Logical);

        unsafe {
            let instr_opcode = self.integer_register_register.opcode();
            let instr_funct3 = self.integer_register_register.funct3();
            let instr_funct7 = self.integer_register_register.funct7();

            let found = descr_opcode.into() == instr_opcode
                && descr_funct3.into() == instr_funct3
                && descr_funct7.into() == instr_funct7;

            if !found {
                return None;
            }

            match descr.format {
                InstructionFormat::IntegerRegisterImmediate => Some(InstructionFormat32::IntegerRegisterImmediate(
                    self.integer_register_immediate,
                )),
                InstructionFormat::IntegerRegisterRegister => Some(InstructionFormat32::IntegerRegisterRegister(
                    self.integer_register_register,
                )),
                InstructionFormat::UnconditionalJump => Some(InstructionFormat32::UnconditionalJump(
                    self.unconditional_jump,
                )),
                InstructionFormat::ConditionBranch => Some(InstructionFormat32::ConditionBranch(self.condition_branch)),
                InstructionFormat::Load => Some(InstructionFormat32::Load(self.load)),
                InstructionFormat::Store => Some(InstructionFormat32::Store(self.store)),
                InstructionFormat::Fence => Some(InstructionFormat32::Fence(self.fence)),
                InstructionFormat::ControlAndStatusRegister => Some(InstructionFormat32::ControlAndStatusRegister(
                    self.control_and_status_register,
                )),
                InstructionFormat::TimeAndCounter => Some(InstructionFormat32::TimeAndCounter(self.time_and_counter)),
                InstructionFormat::EnvironmentCallAndBreakpoint => Some(
                    InstructionFormat32::EnvironmentCallAndBreakpoint(self.environment_call_and_breakpoint),
                ),
            }
        }
    }

    pub(crate) fn new_with_raw_value(value: Word) -> Self { Self { raw: value } }
}
