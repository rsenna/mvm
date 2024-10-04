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

use crate::bitfield::{Opcode7, Opcode7Table};
use crate::instruction::{InstructionFormat32, InstructionFormat32Union};
use crate::memory::Word;
use strum::{Display, EnumDiscriminants};

// TODO: YAEM - Yet Another Enum Macro (instead of enum_aliases)
//       - This macro should be able to generate the enum alias and the conversion functions
//       - The conversion functions should be able to convert from the enum alias to the enum and vice-versa

#[derive(Debug, PartialEq, Display, EnumDiscriminants)]
pub(crate) enum InstructionBaseSet {
    _RV32I(RV32I),
    _RV64I(RV64I),
}

pub trait InstructionSet {}

#[derive(Debug, PartialEq)]
pub struct RV32I;

#[derive(Debug, PartialEq)]
pub struct RV64I;

impl InstructionSet for RV32I {}
impl InstructionSet for RV64I {}

pub(crate) trait IsaModule<W, F>
where
    F: InstructionSet,
{
    fn decode(chomp: W) -> Option<F>;
}

impl InstructionBaseSet {
    pub const RV32I: InstructionBaseSet = InstructionBaseSet::_RV32I(RV32I);
    pub const RV64I: InstructionBaseSet = InstructionBaseSet::_RV64I(RV64I);
}

impl From<InstructionBaseSet> for RV32I {
    fn from(_: InstructionBaseSet) -> Self { RV32I }
}

impl From<InstructionBaseSet> for RV64I {
    fn from(_: InstructionBaseSet) -> Self { RV64I }
}

impl IsaModule<Word, InstructionFormat32> for RV32I {
    fn decode(chomp: Word) -> Option<InstructionFormat32> {
        let union = InstructionFormat32Union { raw: chomp };

        unsafe {
            let opcode: Opcode7 = union.integer_register_register.opcode();
            let opcode: Option<Opcode7Table> = opcode.value().try_into().ok();

            match opcode {
                Some(Opcode7Table::OpImmediate) => Some(InstructionFormat32::IntegerRegisterImmediate(
                    union.integer_register_immediate,
                )),
                Some(Opcode7Table::OpcodeRegister) => Some(InstructionFormat32::IntegerRegisterRegister(
                    union.integer_register_register,
                )),
                Some(Opcode7Table::JumpAndLink) => Some(InstructionFormat32::UnconditionalJump(
                    union.unconditional_jump,
                )),
                Some(Opcode7Table::Branch) => Some(InstructionFormat32::ConditionBranch(union.condition_branch)),
                Some(Opcode7Table::LoadUpperImmediate) | Some(Opcode7Table::AddUpperImmediateToPC) => {
                    Some(InstructionFormat32::Load(union.load))
                }
                Some(Opcode7Table::Store) => Some(InstructionFormat32::Store(union.store)),

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
}
