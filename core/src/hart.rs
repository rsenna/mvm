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
use crate::memory::Byte;
use crate::register::{RegisterValue64, Registers64, RegistersArray64};

pub type InstructionFormat = InstructionFormat32;
pub type RegisterValue = RegisterValue64;
pub type Registers = Registers64;
pub type RegistersArray = RegistersArray64;

// TODO support variable amount of registers
#[derive(Debug)]
pub struct SimpleHart {
    registers: Registers,
    ram: Vec<Byte>,
}

pub trait Hart {
    fn fetch(&mut self) -> InstructionFormat;
    fn read_byte_ram(&self, index: usize) -> Option<&Byte>;
}

impl SimpleHart {
    pub(crate) fn new(ram: Vec<Byte>) -> Self {
        let registers = Registers::new(ram.len());
        Self { registers, ram }
    }
}

impl Hart for SimpleHart {
    // This routine only works for 32 bits instructions
    fn fetch(&mut self) -> InstructionFormat {
        let index = self.registers.pc as usize;
        let data = self.ram[index].0 as u32
            | (self.ram[index + 1].0 as u32) << 8
            | (self.ram[index + 2].0 as u32) << 16
            | (self.ram[index + 3].0 as u32) << 24;
        self.registers.pc += 4;
        data.into()
    }

    fn read_byte_ram(&self, i: usize) -> Option<&Byte> {
        self.ram.get(i)
    }

    // TODO FINALLY use the disruptor pattern! EDIT: actually crossbeam
    //      each Hart (cpu) should process instructions in their own disruptor
    //      that way we can gain speed?

    //    fn execute(&mut self, inst: u32) {
    // let instruction = Instruction::from_u32(inst);
    //
    // execute stage
    // match instruction.opcode {
    // 0x13 => {
    // addi
    // let imm = ((inst & 0xfff0_0000) as i64 >> 20) as u64;
    // self.regs[rd] = self.regs[rs1].wrapping_add(imm);
    // }
    // 0x33 => {
    // add
    // self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
    // }
    //
    // _ => {
    // dbg!(format!("Invalid opcode: {:#x}", opcode));
    // }
    // }
    // }
}
