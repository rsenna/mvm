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

use crate::instruction::{GenericInstructionFormat, InstructionFormat32, InstructionFormat32Union, ADDI};
use crate::isa_module::{InstructionSet, IsaModule, RV32I};
use crate::memory::{Byte, DoubleWord, HalfWord, InstructionLength, Memory, VecMemory, Word};
use crate::register::{RegisterValue64, Registers64};

// TODO support variable amount of registers
#[derive(Debug)]
pub struct SimpleRV32IHart {
    registers: Registers64,
    ram: VecMemory,
}

pub trait Hart<I: InstructionSet, F: GenericInstructionFormat> {
    type ISA = I;
    type InstructionFormat = F;

    fn execute(&mut self, inst: Self::InstructionFormat);
    fn fetch(&mut self) -> Option<Self::InstructionFormat>;

    // TODO FINALLY use the disruptor pattern! EDIT: actually crossbeam
    //      each Hart (cpu) should process instructions in their own disruptor
    //      that way we can gain speed?
}

impl SimpleRV32IHart {
    pub(crate) fn new(memory_size: usize) -> Self {
        let registers = Registers64::new(memory_size);
        let ram = VecMemory::new(memory_size);
        Self { registers, ram }
    }
}

impl Hart<RV32I, InstructionFormat32> for SimpleRV32IHart {
    fn execute(&mut self, formatted: InstructionFormat32) {
        match formatted {
            InstructionFormat32::IntegerRegisterImmediate(i_type) => {
                let rd = i_type.rd();
                let rs1 = i_type.rs1();
                let imm = i_type.imm();

                if formatted.is(ADDI) {
                    self.registers[rd] = self.registers[rs1].wrapping_add(imm);
                }
            }
            InstructionFormat32::IntegerRegisterRegister(r_type) => {}
            InstructionFormat32::UnconditionalJump(j_type) => {}
            InstructionFormat32::ConditionBranch(b_type) => {}
            InstructionFormat32::Load(i_type) => {}
            InstructionFormat32::Store(s_type) => {}
            InstructionFormat32::Fence(if_type) => {}
            InstructionFormat32::ControlAndStatusRegister(i_type) => {}
            InstructionFormat32::TimeAndCounter(i_type) => {}
            InstructionFormat32::EnvironmentCallAndBreakpoint(i_type) => {}
        }
    }

    // This routine only works for 32 bits instructions
    fn fetch(&mut self) -> Option<InstructionFormat32> {
        let index = self.registers.pc as Word;
        let data = self.ram.read_word(index);

        self.registers.pc += InstructionLength::Word as RegisterValue64;

        Self::ISA::decode(data)
    }
}
