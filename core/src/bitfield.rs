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

use std::fmt;
use std::fmt::Formatter;

use crate::impl_common_bitfield_traits;
use arbitrary_int::{u12, u3, u4, u5, u7};
use bitbybit::bitfield;
use strum::{EnumDiscriminants, FromRepr};

pub(crate) type Opcode7 = u7;
pub(crate) type Funct3 = u3;
pub(crate) type Funct7 = u7;
pub(crate) type Rd5 = u5;
pub(crate) type Rs5 = u5;
pub(crate) type Immediate12 = u12;

// #[bitfield(u8, default = 0)]
// #[derive(Debug, PartialEq)]
// pub struct Opcode7Bitfield {
//     #[bits(0..=6, rw)]
//     value: u7,
// }
//
// #[bitfield(u8)]
// #[derive(Debug, PartialEq)]
// pub struct Funct3Bitfield {
//     #[bits(0..=2, rw)]
//     value: u3,
// }
//
// #[bitfield(u8)]
// #[derive(Debug, PartialEq)]
// pub struct Funct7Bitfield {
//     #[bits(0..=6, rw)]
//     value: u7,
// }
//
// #[bitfield(u8)]
// #[derive(Debug, PartialEq)]
// pub struct Rd5Bitfield {
//     #[bits(0..=4, rw)]
//     value: u5,
// }
//
// #[bitfield(u8)]
// #[derive(Debug, PartialEq)]
// pub struct Rs5Bitfield {
//     #[bits(0..=4, rw)]
//     value: u5,
// }
//
// #[bitfield(u8)]
// #[derive(Debug, PartialEq)]
// pub struct Immediate12Bitfield {
//     #[bits(0..=11, rw)]
//     value: u12,
// }

#[bitfield(u32, default = 0)]
#[derive(Debug, PartialEq)]
pub struct RType32Bitfield {
    #[bits(0..=7, rw)]
    opcode: Opcode7,
    #[bits(7..=11, rw)]
    rd: Rd5,
    #[bits(12..=14, rw)]
    funct3: Funct3,
    #[bits(15..=19, rw)]
    rs1: Rs5,
    #[bits(20..=24, rw)]
    rs2: Rs5,
    #[bits(25..=31, rw)]
    funct7: Funct7,
}

#[bitfield(u32, default = 0)]
#[derive(Debug, PartialEq)]
pub struct IType32Bitfield {
    #[bits(0..=6, rw)]
    opcode: Opcode7,
    #[bits(7..=11, rw)]
    rd: Rd5,
    #[bits(12..=14, rw)]
    funct3: Funct3,
    #[bits(15..=19, rw)]
    rs1: Rs5,
    #[bits(20..=31, rw)]
    imm: Immediate12,
}

#[bitfield(u32, default = 0)]
#[derive(Debug, PartialEq)]
pub struct IFenceType32Bitfield {
    #[bits(0..=6, rw)]
    opcode: Opcode7,
    #[bits(7..=11, rw)]
    rd: Rd5,
    #[bits(12..=14, rw)]
    funct3: Funct3,
    #[bits(15..=19, rw)]
    rs1: Rs5,
    #[bit(20, rw)]
    sw: bool,
    #[bit(21, rw)]
    sr: bool,
    #[bit(22, rw)]
    so: bool,
    #[bit(23, rw)]
    si: bool,
    #[bit(24, rw)]
    pw: bool,
    #[bit(25, rw)]
    pr: bool,
    #[bit(26, rw)]
    po: bool,
    #[bit(27, rw)]
    pi: bool,
    #[bits(28..=31, rw)]
    fmt: u4,
}

#[bitfield(u32, default = 0)]
#[derive(Debug, PartialEq)]
pub struct SType32Bitfield {
    #[bits(0..=6, rw)]
    opcode: Opcode7,
    #[bits(12..=14, rw)]
    funct3: Funct3,
    #[bits(15..=19, rw)]
    rs1: Rs5,
    #[bits(20..=24, rw)]
    rs2: Rs5,
    #[bits([7, 8..=11, 25..=30, 31], rw)]
    imm: Immediate12,
}

// Note: similar to SType32, just the imm bits are different
#[bitfield(u32, default = 0)]
#[derive(Debug, PartialEq)]
pub struct BType32Bitfield {
    #[bits(0..=6, rw)]
    opcode: Opcode7,
    #[bits(12..=14, rw)]
    funct3: Funct3,
    #[bits(15..=19, rw)]
    rs1: Rs5,
    #[bits(20..=24, rw)]
    rs2: Rs5,
    // TODO: Implement imm: must shift left by 1 bit
    #[bits([8..=11, 25..=30, 7, 31], rw)]
    imm_raw: Immediate12,
}

#[bitfield(u32, default = 0)]
#[derive(Debug, PartialEq)]
pub struct UType32Bitfield {
    #[bits(0..=6, rw)]
    opcode: Opcode7,
    #[bits(7..=11, rw)]
    rd: Rd5,
    // TODO: Implement imm: must shift left by 12 bits
    #[bits([12..=19, 20..=30, 31], rw)]
    imm_raw: Immediate12,
}

#[bitfield(u32, default = 0)]
#[derive(Debug, PartialEq)]
pub struct JType32Bitfield {
    #[bits(0..=6, rw)]
    opcode: Opcode7,
    #[bits(7..=11, rw)]
    rd: Rd5,
    #[bits([21..=30, 20, 12..=19, 31], rw)]
    imm_raw: Immediate12,
}

#[derive(Debug, Eq, FromRepr, PartialEq, EnumDiscriminants)]
#[repr(u8)]
pub enum Opcode7Table {
    Zero                = 0b0000000,    //   0 == 0x00
    Load                = 0b0000011,    //   3 == 0x03
    LoadFloatingPoint   = 0b0000111,    //   7 == 0x07
    Custom0             = 0b0001011,    //   8 == 0x0B
    MiscMemory          = 0b0001111,    //  15 == 0x0F
    OpImmediate         = 0b0010011,    //  19 == 0x13
    AddUpperImmediateToPC = 0b0010111,  //  23 == 0x17
    OpImmediate32       = 0b0011011,    //  27 == 0x1B
    Store               = 0b0100011,    //  35 == 0x23
    StoreFloatingPoint  = 0b0100111,    //  39 == 0x27
    Custom1             = 0b0101011,    //  43 == 0x2B
    AtomicMemoryOperation = 0b0101111,  //  47 == 0x2F
    OpcodeRegister      = 0b0110011,    //  51 == 0x33
    LoadUpperImmediate  = 0b0110111,    //  55 == 0x37
    OpRegister32        = 0b0111011,    //  59 == 0x3B
    MultiplyAndAdd      = 0b1000011,    //  67 == 0x43
    MultiplyAndSubtract = 0b1000111,    //  71 == 0x47
    NegMultiplyAndSubtract = 0b1001011, //  75 == 0x4B
    NegMultiplyAndAdd   = 0b1001111,    //  79 == 0x4F
    OpFloatingPoint     = 0b1010011,    //  83 == 0x53
    OpVector            = 0b1010111,    //  87 == 0x57
    Custom2Rv128        = 0b1011011,    //  91 == 0x5B
    Branch              = 0b1100011,    //  99 == 0x63
    JumpAndLinkRegister = 0b1100111,    // 103 == 0x67
    Reserved            = 0b1101011,    // 107 == 0x6B
    JumpAndLink         = 0b1101111,    // 111 == 0x6F
    System              = 0b1110011,    // 115 == 0x73
    OpVectorElement     = 0b1110111,    // 119 == 0x77
    Custom3Rv128        = 0b1111011,    // 123 == 0x7B
}

#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[repr(u8)]
//#[EnumAlias(SUB = ADD, SRL = SRA)]
pub enum Funct3OpcodeTable {
    ADD  = 0b000, // 0
    SLL  = 0b001, // 1
    SLT  = 0b010, // 2
    SLTU = 0b011, // 3
    XOR  = 0b100, // 4
    SRA  = 0b101, // 5
    OR   = 0b110, // 6
    AND  = 0b111, // 7
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
enum Funct3BranchTable {
    BEQ  = 0b000, // 0
    BNE  = 0b001, // 1
    BLT  = 0b100, // 4
    BGE  = 0b101, // 5
    BLTU = 0b110, // 6
    BGEU = 0b111, // 7
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
enum Funct3LoadTable {
    LB  = 0b000, // 0
    LH  = 0b001, // 1
    LW  = 0b010, // 2
    LBU = 0b100, // 4
    LHU = 0b101, // 5
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
enum Funct3StoreTable {
    SB = 0b000, // 0
    SH = 0b001, // 1
    SW = 0b010, // 2
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
//#[EnumAlias(SRLI = SRAI)]
enum Funct3IntegerRegisterImmediateTable {
    ADDI  = 0b000, // 0
    SLLI  = 0b001, // 1
    SLTI  = 0b010, // 2
    SLTIU = 0b011, // 3
    XORI  = 0b100, // 4
    SRAI  = 0b101, // 5
    ORI   = 0b110, // 6
    ANDI  = 0b111, // 7
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
//#[EnumAlias(EBREAK = ECALL)]
enum Funct3SystemTable {
    ECALL = 0b000, // 0
}

#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[repr(u8)]
pub enum Funct3Uop {
    Zero = 0,
    Opcode(Funct3OpcodeTable),
    Branch(Funct3BranchTable),
    Load(Funct3LoadTable),
    Store(Funct3StoreTable),
    IntegerRegisterImmediate(Funct3IntegerRegisterImmediateTable),
    System(Funct3SystemTable),
}

#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[repr(u8)]
pub enum Funct7Uop {
    Logical    = 0,
    Arithmetic = 0b0100000,
}

// begin TODO: temp declarations

#[derive(Debug, Eq, PartialEq, EnumDiscriminants)]
#[repr(u8)]
pub enum Immediate32Uop {
    Zero = 0,
}

// end

// impl_common_bitfield_traits!(Opcode7Bitfield);
// impl_common_bitfield_traits!(Funct3Bitfield);
// impl_common_bitfield_traits!(Funct7Bitfield);
// impl_common_bitfield_traits!(Rd5Bitfield);
// impl_common_bitfield_traits!(Rs5Bitfield);
// impl_common_bitfield_traits!(Immediate12Bitfield);
impl_common_bitfield_traits!(RType32Bitfield);
impl_common_bitfield_traits!(IType32Bitfield);
impl_common_bitfield_traits!(IFenceType32Bitfield);
impl_common_bitfield_traits!(SType32Bitfield);
impl_common_bitfield_traits!(BType32Bitfield);
impl_common_bitfield_traits!(UType32Bitfield);
impl_common_bitfield_traits!(JType32Bitfield);

// TODO: make automatic conversions from Funct3* to Funct3Bits

impl Into<Funct3> for Funct3Op {
    fn into(self) -> Funct3 {
        match self {
            Funct3Uop::Opcode(funct3) => funct3.into(),
            Funct3Uop::Branch(funct3) => funct3.into(),
            Funct3Uop::Load(funct3) => funct3.into(),
            Funct3Uop::Store(funct3) => funct3.into(),
            Funct3Uop::IntegerRegisterImmediate(funct3) => funct3.into(),
            Funct3Uop::System(funct3) => funct3.into(),
        }
    }
}
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
