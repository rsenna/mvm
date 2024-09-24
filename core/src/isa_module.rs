/*
 * Copyright ©️ 2024 Rogério Senna. All rights reserved.
 *
 * Licensed under the EUPL, Version 1.2 or – as soon they will be approved by
 * the European Commission - subsequent versions of the EUPL (the "Licence");
 * You may not use this work except in compliance with the Licence.
 * You may obtain a copy of the Licence at:
 *
 * https://joinup.ec.europa.eu/software/page/eupl
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the Licence is distributed on an "AS IS" basis,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the Licence for the specific language governing permissions and
 * limitations under the Licence.
 *
 */

use crate::instruction::InstructionFormat32;
use crate::memory::Word;
use crate::opcode::{Opcode7Bitfield, Opcode7Table};

pub(crate) enum InstructionBaseSet {
    _RV32I(RV32I),
    _RV64I(RV64I),
}

pub struct RV32I;
pub struct RV64I;

trait IsaModule<W, F> {
    fn decode(&self, isa_word: W) -> Option<F>;
}

// TODO: YAEM - Yet Another Enum Macro (instead of enum_aliases)
//       - This macro should be able to generate the enum alias and the conversion functions
//       - The conversion functions should be able to convert from the enum alias to the enum and vice-versa

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
    fn decode(&self, isa_word: Word) -> Option<InstructionFormat32> {
        let opcode: Opcode7Bitfield = isa_word.into();
        let opcode: Option<Opcode7Table> = opcode.into();

        match opcode {
            Some(Opcode7Table::OpImmediate) => Some(InstructionFormat32::integer_register_immediate(isa_word)),
            Some(Opcode7Table::OpcodeRegister) => Some(InstructionFormat32::integer_register_register(isa_word)),
            Some(Opcode7Table::JumpAndLink) => Some(InstructionFormat32::unconditional_jump(isa_word)),
            Some(Opcode7Table::Branch) => Some(InstructionFormat32::condition_branch(isa_word)),
            Some(Opcode7Table::LoadUpperImmediate) | Some(Opcode7Table::AddUpperImmediateToPC) => {
                Some(InstructionFormat32::load(isa_word))
            }
            Some(Opcode7Table::Store) => Some(InstructionFormat32::store(isa_word)),
            // Not used in RV32I:
            //
            // Some(Opcode7Table::Fence) => Some(InstructionFormat32::fence(isa_word)),
            // Some(Opcode7Table::ControlAndStatusRegister) => {
            //     Some(InstructionFormat32::control_and_status_register(isa_word))
            // }
            // Some(Opcode7Table::TimeAndCounter) => Some(InstructionFormat32::time_and_counter(isa_word)),
            // Some(Opcode7Table::EnvironmentCallAndBreakpoint) => {
            //     Some(InstructionFormat32::environment_call_and_breakpoint(isa_word))
            // }
            _ => None,
        }
    }
}
