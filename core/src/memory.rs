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

pub(crate) type Byte = u8;
pub(crate) type HalfWord = u16;
pub(crate) type Word = u32;
pub(crate) type DoubleWord = u64;

pub trait Memory {
    fn read_byte(&self, address: Word) -> Byte;
    fn read_half_word(&self, address: Word) -> HalfWord;
    fn read_word(&self, address: Word) -> Word;
    fn read_double_word(&self, address: Word) -> DoubleWord;
    fn write_byte(&mut self, address: Word, value: &Byte);
    fn write_half_word(&mut self, address: Word, value: &HalfWord);
    fn write_word(&mut self, address: Word, value: &Word);
    fn write_double_word(&mut self, address: Word, value: &DoubleWord);
}

#[derive(Debug)]
pub struct VecMemory {
    ram: Vec<Byte>,
}

pub enum InstructionLength {
    Byte       = 8,
    HalfWord   = 16,
    Word       = 32,
    DoubleWord = 64,
}

// TODO: I tried using functions and macros to avoid code duplication below, but I couldn't make it work.
impl Memory for VecMemory {
    fn read_byte(&self, address: Word) -> Byte { self.ram[address as usize] }

    fn read_half_word(&self, address: Word) -> HalfWord {
        self.read_byte(address) as HalfWord | ((self.read_byte(address + 1) as HalfWord) << Byte::BITS)
    }

    fn read_word(&self, address: Word) -> Word {
        self.read_half_word(address) as Word | ((self.read_half_word(address + 2) as Word) << HalfWord::BITS)
    }

    fn read_double_word(&self, address: Word) -> DoubleWord {
        self.read_word(address) as DoubleWord | ((self.read_word(address + 4) as DoubleWord) << Word::BITS)
    }

    fn write_byte(&mut self, address: Word, value: &Byte) { self.ram[address as usize] = *value }

    fn write_half_word(&mut self, address: Word, value: &HalfWord) {
        for i in 0..HalfWord::BITS {
            let actual_value = (value >> (i * Byte::BITS)) as Byte;
            self.write_byte(address + i as Word, &actual_value);
        }
    }

    fn write_word(&mut self, address: Word, value: &Word) {
        for i in 0..Word::BITS {
            let actual_value = (value >> (i * Byte::BITS)) as Byte;
            self.write_byte(address + i as Word, &actual_value);
        }
    }

    fn write_double_word(&mut self, address: Word, value: &DoubleWord) {
        for i in 0..DoubleWord::BITS {
            let actual_value = (value >> (i * Byte::BITS)) as Byte;
            self.write_byte(address + i as Word, &actual_value);
        }
    }
}

impl VecMemory {
    pub fn new(size: usize) -> Self { Self { ram: vec![0; size] } }
}
