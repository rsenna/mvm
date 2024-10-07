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

use arbitrary_int::Number;
use derive_more::Display;
use kinded::Kinded;

use crate::bitfield::{
    BType32Bitfield, Funct3, Funct7, IFenceType32Bitfield, IType32Bitfield, JType32Bitfield, Opcode7, Opcode7Table,
    RType32Bitfield, SType32Bitfield,
};
use crate::instruction::{ChompRV32, Descriptor};
use crate::memory::Word;

// TODO: YAEM - Yet Another Enum Macro (instead of enum_aliases)
//       - This macro should be able to generate the enum alias and the conversion functions
//       - The conversion functions should be able to convert from the enum alias to the enum and vice-versa

pub(crate) trait Architecture<C, I>
where
    Self: Sized,
    Self: InstructionSet,
    C: Number,
    I: Instruction,
{
    type Chomp = C;
    type Instruction = I;

    fn decode(&self, chomp: C) -> Option<I>;
    fn get_opcode(&self, instruction: I) -> Opcode7;
    fn match_instruction(&self, instruction: I, descr: Descriptor) -> bool;
}

// Abstract "Tag" trait for the instruction enum types
pub trait Instruction {}
pub trait InstructionSet {}

pub struct RV32I;
impl InstructionSet for RV32I {}
impl RV32I {
    pub const fn name(&self) -> &str { "RV32I" }
}

pub struct RV64I;
impl InstructionSet for RV64I {}
impl RV64I {
    pub const fn name(&self) -> &str { "RV64I" }
}

#[derive(Debug, Kinded, PartialEq)]
#[kinded(kind = InstructionKind)]
#[repr(u8)]
pub enum Format<I, R, J, B, S, F> {
    IntegerRegisterImmediate(I),     // I Type
    IntegerRegisterRegister(R),      // R Type
    UnconditionalJump(J),            // J Type
    ConditionBranch(B),              // B Type
    Load(I),                         // I Type
    Store(S),                        // S Type
    Fence(F),                        // IFence Type
    ControlAndStatusRegister(I),     // I Type
    TimeAndCounter(I),               // I Type
    EnvironmentCallAndBreakpoint(R), // R Type-like ... TODO review
}

// TODO: move to rv32i.rs
pub type RV32Instruction =
    Format<IType32Bitfield, RType32Bitfield, JType32Bitfield, BType32Bitfield, SType32Bitfield, IFenceType32Bitfield>;
impl Instruction for RV32Instruction {}

// TODO
#[derive(Debug, Display, PartialEq)]
pub enum RV64Instruction {}
impl Instruction for RV64Instruction {}

impl Architecture<Word, RV32Instruction> for RV32I {
    fn decode(&self, chomp: Self::Chomp) -> Option<Self::Instruction> {
        let union = ChompRV32 { raw: chomp };

        unsafe {
            let opcode: Opcode7 = union.integer_register_register.opcode();
            let opcode: Option<Opcode7Table> = opcode.value().try_into().ok();

            match opcode {
                Some(Opcode7Table::OpImmediate) => Some(RV32Instruction::IntegerRegisterImmediate(
                    union.integer_register_immediate,
                )),
                Some(Opcode7Table::OpRegister) => Some(RV32Instruction::IntegerRegisterRegister(
                    union.integer_register_register,
                )),
                Some(Opcode7Table::JumpAndLink) => Some(RV32Instruction::UnconditionalJump(union.unconditional_jump)),
                Some(Opcode7Table::Branch) => Some(RV32Instruction::ConditionBranch(union.condition_branch)),
                Some(Opcode7Table::LoadUpperImmediate) | Some(Opcode7Table::AddUpperImmediatePC) => {
                    Some(RV32Instruction::Load(union.load))
                }
                Some(Opcode7Table::Store) => Some(RV32Instruction::Store(union.store)),

                // Not used in RV32I:
                //
                // Some(Opcode7Table::Fence) => Some(InstructionFormat32::Fence(union)),
                // Some(Opcode7Table::ControlAndStatusRegister) => {
                //     Some(InstructionFormat32::ControlAndStatusRegister(union))
                // }
                // Some(Opcode7Table::TimeAndCounter) => Some(InstructionFormat32::TimeAndCounter(union)),
                // Some(Opcode7Table::EnvironmentCallAndBreakpoint) => {
                //     Some(InstructionFormat32::EnvironmentCallAndBreakpoint(union))
                // }
                _ => None,
            }
        }
    }

    fn get_opcode(&self, instruction: RV32Instruction) -> Opcode7 {
        let opcode = match instruction {
            RV32Instruction::IntegerRegisterImmediate(chomp) => chomp.opcode(),
            RV32Instruction::IntegerRegisterRegister(chomp) => chomp.opcode(),
            RV32Instruction::UnconditionalJump(chomp) => chomp.opcode(),
            RV32Instruction::ConditionBranch(chomp) => chomp.opcode(),
            RV32Instruction::Load(chomp) => chomp.opcode(),
            RV32Instruction::Store(chomp) => chomp.opcode(),
            RV32Instruction::Fence(chomp) => chomp.opcode(),
            RV32Instruction::ControlAndStatusRegister(chomp) => chomp.opcode(),
            RV32Instruction::TimeAndCounter(chomp) => chomp.opcode(),
            RV32Instruction::EnvironmentCallAndBreakpoint(chomp) => chomp.opcode(),
        };

        opcode
    }

    fn match_instruction(&self, instruction: RV32Instruction, descr: Descriptor) -> bool {
        let check_o7f3f7 = |opcode: Opcode7, funct3: Funct3, funct7: Funct7| -> bool {
            descr.opcode == opcode.try_into().ok()
                // TODO: missing information to convert funct3 into Funct3Expr (same value can be in different tables)
                && descr.funct3 == funct3.try_into().ok()
                && descr.funct7 == funct7.try_into().ok()
        };

        let check_o7f3 = |opcode: Opcode7, funct3: Funct3| -> bool {
            descr.opcode == opcode.try_into().ok() && descr.funct3 == funct3.try_into().ok() && descr.funct7.is_none()
        };

        match instruction {
            RV32Instruction::Load(i_type)
            | RV32Instruction::ControlAndStatusRegister(i_type)
            | RV32Instruction::TimeAndCounter(i_type)
            | RV32Instruction::IntegerRegisterImmediate(i_type) => check_o7f3(i_type.opcode(), i_type.funct3()),

            RV32Instruction::UnconditionalJump(j_type) => j_type.opcode().try_into().ok() == descr.opcode,

            RV32Instruction::IntegerRegisterRegister(r_type)
            | RV32Instruction::EnvironmentCallAndBreakpoint(r_type) => {
                check_o7f3f7(r_type.opcode(), r_type.funct3(), r_type.funct7())
            }

            RV32Instruction::ConditionBranch(b_type) => check_o7f3(b_type.opcode(), b_type.funct3()),
            RV32Instruction::Store(s_type) => check_o7f3(s_type.opcode(), s_type.funct3()),
            RV32Instruction::Fence(i_fence_type) => check_o7f3(i_fence_type.opcode(), i_fence_type.funct3()),
        }
    }
}
