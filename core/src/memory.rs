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

use derive_more::Display;

use crate::instruction::InstructionFormat32;
use crate::opcode::Opcode7Table;

#[derive(Copy, Clone, Debug, Display, PartialEq)]
pub struct Byte(pub u8);

#[derive(Copy, Clone, Debug, Display, PartialEq)]
pub struct HalfWord(pub u16);

#[derive(Copy, Clone, Debug, Display, PartialEq)]
pub struct Word(pub u32);

#[derive(Copy, Clone, Debug, Display, PartialEq)]
pub struct DoubleWord(pub u64);

pub struct Memory {
    ram: Vec<Byte>,
}

pub enum InstructionLength {
    Byte = 8,
    HalfWord = 16,
    Word = 32,
    DoubleWord = 64,
}

impl From<Word> for Option<InstructionFormat32> {
    fn from(value: Word) -> Self {
        let opcode = value.into().opcode();
        let opcode_id_opt = opcode.try_into().ok();

        match opcode_id_opt {
            Some(Opcode7Table::OpImmediate) => Some(InstructionFormat32::IntegerRegisterImmediate(value)),
            Some(Opcode7Table::JumpAndLinkRegister) => Some(InstructionFormat32::ConditionBranch(value)),
            Some(Opcode7Table::OpcodeRegister) => Some(InstructionFormat32::IntegerRegisterRegister(value)),
            Some(Opcode7Table::JumpAndLink) => Some(InstructionFormat32::UnconditionalJump(value)),
            Some(Opcode7Table::LoadUpperImmediate) | Some(Opcode7Table::AddUpperImmediateToPC) => {
                Some(InstructionFormat32::Load(value))
            }
            Some(Opcode7Table::Branch) => Some(InstructionFormat32::ConditionBranch(value)),
            Some(Opcode7Table::Store) => Some(InstructionFormat32::Store(value)),
            _ => None, // TODO missing other instruction formats
        }
    }
}
