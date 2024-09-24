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

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use bitfield::bitfield;
use derive_more::{From, Into, TryFrom};

use crate::impl_common_bitfield_traits;

pub const INVALID_OPCODE7: Opcode7Bitfield = Opcode7Bitfield(0);

bitfield! {
    pub struct Opcode7Bitfield(u8);
    impl Debug;
    u8, get, set: 6, 0;
}

bitfield! {
    struct Funct3Bitfield(u8);
    impl Debug;
    u8, get, set: 2, 0;
}

bitfield! {
    struct Funct7Bitfield(u8);
    impl Debug;
    u8, get, set: 6, 0;
}

bitfield! {
    struct Rd5Bitfield(u8);
    impl Debug;
    u8, get, set: 4, 0;
}

bitfield! {
    struct Rs5Bitfield(u8);
    impl Debug;
    u8, get, set: 4, 0;
}

bitfield! {
    struct Immediate12Bitfield(u16);
    impl Debug;
    u16, get, set: 11, 0;
}

bitfield! {
    pub struct RType32Bitfield(u32);
    impl Debug;
    pub u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, funct7, set_funct7: 31, 25;
}

bitfield! {
    pub struct IType32Bitfield(u32);
    impl Debug;
    pub u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct IFenceType32Bitfield(u32);
    impl Debug;
    pub u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    sw, set_sw: 20;
    sr, set_sr: 21;
    so, set_so: 22;
    si, set_si: 23;
    pw, set_pw: 24;
    pr, set_pr: 25;
    po, set_po: 26;
    pi, set_pi: 27;
    u8, fmt, set_fmt: 31, 28;
}

bitfield! {
    pub struct SType32Bitfield(u32);
    impl Debug;
    pub u8, opcode, set_opcode: 6, 0;
    imm_b7, set_imm_b7: 7;
    u8, imm_b11_8, set_imm_b11_8: 11, 8;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

// Note: same as SType32, but let's keep a separate type just in case
bitfield! {
    pub struct BType32Bitfield(u32);
    impl Debug;
    pub u8, opcode, set_opcode: 6, 0;
    imm_b7, set_imm_b7: 7;
    u8, imm_b11_8, set_imm_b11_8: 11, 8;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct UType32Bitfield(u32);
    impl Debug;
    pub u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    u16, imm_b30_20, set_imm_b30_20: 30, 20;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct JType32Bitfield(u32);
    impl Debug;
    pub u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct Immediate32Bitfield(u32);
    impl Debug;
    bit0, set_bit0: 0;
    u8, seq_b4_1, set_seq_b4_1: 4, 1;
    u8, seq_b10_5, set_seq_b10_5: 10, 5;
    bit11, set_bit11: 11;
    u8, seq_b19_12, set_seq_b19_12: 19, 12;
    u16, seq_b30_20, set_seq_b30_20: 30, 20;
    bit31, set_bit31: 31;
}

pub trait Bits {
    // TODO:
}

#[derive(Debug, Eq, PartialEq, TryFrom)]
#[try_from(repr)]
#[repr(u8)]
pub enum Opcode7Table {
    Load = 0b0000011,                   //   3
    LoadFloatingPoint = 0b0000111,      //   7
    Custom0 = 0b0001011,                //   8
    MiscMemory = 0b0001111,             //  15
    OpImmediate = 0b0010011,            //  19
    AddUpperImmediateToPC = 0b0010111,  //  23
    OpImmediate32 = 0b0011011,          //  27
    Store = 0b0100011,                  //  35
    StoreFloatingPoint = 0b0100111,     //  39
    Custom1 = 0b0101011,                //  43
    AtomicMemoryOperation = 0b0101111,  //  47
    OpcodeRegister = 0b0110011,         //  51
    LoadUpperImmediate = 0b0110111,     //  55
    OpRegister32 = 0b0111011,           //  59
    MultiplyAndAdd = 0b1000011,         //  67
    MultiplyAndSubtract = 0b1000111,    //  71
    NegMultiplyAndSubtract = 0b1001011, //  75
    NegMultiplyAndAdd = 0b1001111,      //  79
    OpFloatingPoint = 0b1010011,        //  83
    OpVector = 0b1010111,               //  87
    Custom2Rv128 = 0b1011011,           //  91
    Branch = 0b1100011,                 //  99
    JumpAndLinkRegister = 0b1100111,    // 103
    Reserved = 0b1101011,               // 107
    JumpAndLink = 0b1101111,            // 111
    System = 0b1110011,                 // 115
    OpVectorElement = 0b1110111,        // 119
    Custom3Rv128 = 0b1111011,           // 123
}

#[repr(u8)]
#[derive(PartialEq, Eq, From)]
#[from(Byte)]
//#[EnumAlias(SUB = ADD, SRL = SRA)]
pub enum Funct3OpcodeTable {
    ADD = 0b000,  // 0
    SLL = 0b001,  // 1
    SLT = 0b010,  // 2
    SLTU = 0b011, // 3
    XOR = 0b100,  // 4
    SRA = 0b101,  // 5
    OR = 0b110,   // 6
    AND = 0b111,  // 7
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Funct3BranchTable {
    BEQ = 0b000,  // 0
    BNE = 0b001,  // 1
    BLT = 0b100,  // 4
    BGE = 0b101,  // 5
    BLTU = 0b110, // 6
    BGEU = 0b111, // 7
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Funct3LoadTable {
    LB = 0b000,  // 0
    LH = 0b001,  // 1
    LW = 0b010,  // 2
    LBU = 0b100, // 4
    LHU = 0b101, // 5
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Funct3StoreTable {
    SB = 0b000, // 0
    SH = 0b001, // 1
    SW = 0b010, // 2
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
//#[EnumAlias(SRLI = SRAI)]
enum Funct3IntegerRegisterImmediateTable {
    ADDI = 0b000,  // 0
    SLLI = 0b001,  // 1
    SLTI = 0b010,  // 2
    SLTIU = 0b011, // 3
    XORI = 0b100,  // 4
    SRAI = 0b101,  // 5
    ORI = 0b110,   // 6
    ANDI = 0b111,  // 7
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
//#[EnumAlias(EBREAK = ECALL)]
enum Funct3SystemTable {
    ECALL = 0b000, // 0
}

pub enum Funct3Uop {
    Opcode(Funct3OpcodeTable),
    Branch(Funct3BranchTable),
    Load(Funct3LoadTable),
    Store(Funct3StoreTable),
    IntegerRegisterImmediate(Funct3IntegerRegisterImmediateTable),
    System(Funct3SystemTable),
}

// TODO: temp declarations
pub enum Funct7Uop {}
pub enum Immediate32Uop {}

impl_common_bitfield_traits!(Opcode7Bitfield);
impl_common_bitfield_traits!(RType32Bitfield);
impl_common_bitfield_traits!(IType32Bitfield);
impl_common_bitfield_traits!(IFenceType32Bitfield);
impl_common_bitfield_traits!(SType32Bitfield);
impl_common_bitfield_traits!(BType32Bitfield);
impl_common_bitfield_traits!(UType32Bitfield);
impl_common_bitfield_traits!(JType32Bitfield);
impl_common_bitfield_traits!(Immediate32Bitfield);

// TODO: make automatic conversions from Funct3* to Funct3Bits

// impl From<Funct3Uop> for Byte {
//     fn from(value: Funct3Uop) -> Self {
//         match value {
//             Funct3Uop::Opcode(opcode) => Self(opcode.into()),
//             Funct3Uop::Branch(branch) => Self(branch.into()),
//             Funct3Uop::Load(load) => Self(load.into()),
//             Funct3Uop::Store(store) => Self(store.into()),
//             Funct3Uop::IntegerRegisterImmediate(iri) => Self(iri.into()),
//             Funct3Uop::System(system) => Self(system.into()),
//         }
//     }
// }
//
// impl From<Opcode7Table> for Byte {
//     fn from(value: Opcode7Table) -> Self {
//         Byte(value.into().unwrap())
//     }
// }
//
// impl Debug for Opcode7Bitfield {
//     fn fmt(&self, form: &mut Formatter<'_>) -> fmt::Result {
//         form.debug_tuple("Opcode").field(&self.0).finish()
//     }
// }
//
// impl From<Opcode7Table> for Opcode7Bitfield {
//     fn from(value: Opcode7Table) -> Self {
//         Opcode7Bitfield(value.into())
//     }
// }
//
// impl From<Word> for Opcode7Bitfield {
//     fn from(value: Word) -> Self {
//         Opcode7Bitfield(Byte(value.0 as u8))
//     }
// }
//
// impl From<Opcode7Bitfield> for Option<Opcode7Table> {
//     fn from(value: Opcode7Bitfield) -> Self {
//         value.into()
//     }
// }
