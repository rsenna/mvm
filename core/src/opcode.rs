/*
 * Copyright (c) Rogério Senna 2024.
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

use monologvm_pmacro::EnumAliases;

use crate::etc::{Byte, DoubleWord, HalfWord, InstructionLength, Word};
use crate::impl_traits;
use bitfield::bitfield;
use derive_more::{From, Into};
use enum_primitive_derive::Primitive;
use num_traits::{FromPrimitive, ToPrimitive};

pub const INVALID_OPCODE7: OpcodeBits = OpcodeBits(0);

#[derive(Debug, Eq, PartialEq, Primitive)]
#[repr(u8)]
pub enum OpcodeMapID {
    Null = 0b0000000,                   //   0 TODO: remove Null, use Option<OpcodeID>
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
#[derive(PartialEq, Eq, EnumAliases(SUB = ADD, SRL = SRA), Into(Byte))]
pub enum Funct3Opcode {
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
enum Funct3Branch {
    BEQ = 0b000,  // 0
    BNE = 0b001,  // 1
    BLT = 0b100,  // 4
    BGE = 0b101,  // 5
    BLTU = 0b110, // 6
    BGEU = 0b111, // 7
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Funct3Load {
    LB = 0b000,  // 0
    LH = 0b001,  // 1
    LW = 0b010,  // 2
    LBU = 0b100, // 4
    LHU = 0b101, // 5
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
enum Funct3Store {
    SB = 0b000, // 0
    SH = 0b001, // 1
    SW = 0b010, // 2
}

#[repr(u8)]
#[derive(PartialEq, Eq, EnumAliases(SRLI = SRAI))]
enum Funct3IntegerRegisterImmediate {
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
#[derive(PartialEq, Eq, EnumAliases(EBREAK = ECALL))]
enum Funct3System {
    ECALL = 0b000, // 0
}

pub enum Funct3 {
    Opcode(Funct3Opcode),
    Branch(Funct3Branch),
    Load(Funct3Load),
    Store(Funct3Store),
    IntegerRegisterImmediate(Funct3IntegerRegisterImmediate),
    System(Funct3System),
}

bitfield! {
    struct OpcodeBits(Byte);
    impl Debug, PartialEq, From;
    Byte, get, set: 6, 0;
}

bitfield! {
    struct Funct3Bits(Byte);
    impl Debug, PartialEq, From;
    Byte, get, set: 2, 0;
}

bitfield! {
    struct Funct7Bits(Byte);
    impl Debug, PartialEq, From;
    Byte, get, set: 6, 0;
}

bitfield! {
    struct RdRsBits(Byte);
    impl Debug, PartialEq, From;
    Byte, get, set: 4, 0;
}

bitfield! {
    struct Immediate12Bits(HalfWord);
    impl Debug, PartialEq, From;
    HalfWord, get, set: 11, 0;
}

bitfield! {
    pub struct RType32(Word);
    impl Debug, PartialEq, From;
    u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, funct7, set_funct7: 31, 25;
}

bitfield! {
    pub struct IType32(Word);
    impl Debug, PartialEq, From;
    u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct IFenceType32(Word);
    impl Debug, PartialEq, From;
    u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    sw, set_sw: 20;
    sr, set_sw: 21;
    so, set_sw: 22;
    si, set_sw: 23;
    pw, set_sw: 24;
    pr, set_sw: 25;
    po, set_sw: 26;
    pi, set_sw: 27;
    u8, fmt, set_fmt: 31, 28;
}

bitfield! {
    pub struct SType32(Word);
    impl Debug, PartialEq, From;
    u8, opcode, set_opcode: 6, 0;
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
    pub struct BType32(Word);
    impl Debug, PartialEq, From;
    u8, opcode, set_opcode: 6, 0;
    imm_b7, set_imm_b7: 7;
    u8, imm_b11_8, set_imm_b11_8: 11, 8;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct UType32(Word);
    impl Debug, PartialEq, From;
    u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    u16, imm_b30_20, set_imm_b30_20: 30, 20;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct JType32(Word);
    impl Debug, PartialEq, From;
    u8, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct Immediate32Bits(Word);
    impl Debug, PartialEq, From;
    bit0, set_bit0: 0;
    u8, seq_b4_1, set_seq_b4_1: 4, 1;
    u8, seq_b10_5, set_seq_b10_5: 10, 5;
    bit11, set_bit11: 11;
    u8, seq_b19_12, set_seq_b19_12: 19, 12;
    u16, seq_b30_20, set_seq_b30_20: 30, 20;
    bit31, set_bit31: 31;
}

impl_traits!(OpcodeBits);
impl_traits!(RType32);
impl_traits!(IType32);
impl_traits!(IFenceType32);
impl_traits!(SType32);
impl_traits!(BType32);
impl_traits!(UType32);
impl_traits!(JType32);
impl_traits!(Immediate32Bits);

// TODO: make automatic conversions from Funct3* to Funct3Bits

pub const fn get_funct_3(byte: Byte) -> Option<Funct3> {
    Some(Funct3(byte))
}

const fn get_funct_7(byte: Byte) -> Option<Funct7Bits> {
    Some(Funct7Bits(byte))
}

impl From<Funct3> for Byte {
    fn from(value: Funct3) -> Self {
        match value {
            Funct3(_) => {}
        }
    }
}

impl From<OpcodeMapID> for u8 {
    fn from(value: OpcodeMapID) -> Self {
        value.to_u8().unwrap()
    }
}

impl Debug for OpcodeBits {
    fn fmt(&self, form: &mut Formatter<'_>) -> fmt::Result {
        form.debug_tuple("Opcode").field(&self.0).finish()
    }
}

impl From<OpcodeMapID> for OpcodeBits {
    fn from(value: OpcodeMapID) -> Self {
        OpcodeBits(value.to_u8().unwrap())
    }
}
