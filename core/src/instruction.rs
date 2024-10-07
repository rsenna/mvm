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

use crate::architecture::{InstructionKind, RV32I};
use crate::bitfield::{
    BType32Bitfield, Funct3Expr, Funct3OpRegisterTable, Funct7Table, IFenceType32Bitfield, IType32Bitfield,
    Immediate11Table, JType32Bitfield, Opcode7Table, RType32Bitfield, SType32Bitfield,
};
use crate::memory::{InstructionLength, Word};
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

pub const ADDI: Descriptor = Descriptor {
    set: RV32I.name(),
    name: "Add Immediate",
    format: InstructionKind::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Expr::OpRegister(Funct3OpRegisterTable::ADD)),
    funct7: None,
    imm11: None,
};

pub const SLTI: Descriptor = Descriptor {
    set: RV32I.name(),
    name: "Set Less Than Immediate",
    format: InstructionKind::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Expr::OpRegister(Funct3OpRegisterTable::SLT)),
    funct7: None,
    imm11: None,
};

pub const SLTIU: Descriptor = Descriptor {
    set: RV32I.name(),
    name: "Set Less Than Immediate Unsigned",
    format: InstructionKind::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Expr::OpRegister(Funct3OpRegisterTable::SLTU)),
    funct7: None,
    imm11: None,
};

pub const ANDI: Descriptor = Descriptor {
    set: RV32I.name(),
    name: "AND Immediate",
    format: InstructionKind::IntegerRegisterImmediate,
    opcode: Some(Opcode7Table::OpImmediate),
    funct3: Some(Funct3Expr::OpRegister(Funct3OpRegisterTable::SLTU)), // TODO review
    funct7: None,
    imm11: None,
};

// TODO variable instruction length;
//      see https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf page 5

#[repr(C)]
pub union ChompRV32 {
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

#[derive(Debug, PartialEq)]
pub struct Descriptor {
    pub set: &'static str,
    pub name: &'static str,
    pub format: InstructionKind,
    pub opcode: Option<Opcode7Table>,
    pub funct3: Option<Funct3Expr>,
    pub funct7: Option<Funct7Table>,
    pub imm11: Option<Immediate11Table>,
}
