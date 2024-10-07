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

use crate::architecture::{Architecture, Instruction, InstructionSet, RV32Instruction, RV32I};
use crate::instruction::ADDI;
use crate::memory::{InstructionLength, Memory, VecMemory, Word};
use crate::register::{RegisterValue64, Registers64};

// TODO support variable amount of registers
#[derive(Debug)]
pub struct SimpleRV32IHart {
    registers: Registers64,
    ram: VecMemory,
}

pub trait Hart<I: InstructionSet, F: Instruction> {
    type ISA = I;
    type Instruction = F;

    fn execute(&mut self, inst: Self::Instruction);
    fn fetch(&mut self) -> Option<Self::Instruction>;

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

impl Hart<RV32I, RV32Instruction> for SimpleRV32IHart {
    fn execute(&mut self, instruction: RV32Instruction) {
        match instruction {
            RV32Instruction::IntegerRegisterImmediate(i_type) => {
                let rd = i_type.rd();
                let rs1 = i_type.rs1();
                let imm = i_type.imm();

                if RV32I.match_instruction(instruction, ADDI) {
                    self.registers.array[rd] = self.registers.array[rs1].wrapping_add(imm);
                }
            }
            RV32Instruction::IntegerRegisterRegister(r_type) => {}
            RV32Instruction::UnconditionalJump(j_type) => {}
            RV32Instruction::ConditionBranch(b_type) => {}
            RV32Instruction::Load(i_type) => {}
            RV32Instruction::Store(s_type) => {}
            RV32Instruction::Fence(if_type) => {}
            RV32Instruction::ControlAndStatusRegister(i_type) => {}
            RV32Instruction::TimeAndCounter(i_type) => {}
            RV32Instruction::EnvironmentCallAndBreakpoint(i_type) => {}
        }
    }

    // This routine only works for 32 bits instructions
    fn fetch(&mut self) -> Option<RV32Instruction> {
        let index = self.registers.pc as Word;
        let data = self.ram.read_word(index);

        self.registers.pc += InstructionLength::Word as RegisterValue64;

        RV32I.decode(data)
    }
}
