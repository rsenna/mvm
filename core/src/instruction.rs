// TODO variable instruction length; see https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf
//      page 5

use bitfield::bitfield;

/* bitfield! {
    // `field3` will be read as an `u32` and then converted to `FooBar`.
    // The setter is not affected, it still needs an `u32` value.
    u32, into FooBar, field3, set_field3: 10, 0;
    // `field4` will be read as an `u32` and then converted to `FooBar`.
    // The setter will take a `FooBar`, and converted back to an `u32`.
    u32, from into FooBar, field4, set_field4: 10, 0;
    // `field5` will be read as an `u32` and then converted to `FooBar`.
    // The setter will take a `FooBar`, and converted back to an `u32`.
    // The struct will have an associated constant `FIELD5_MASK` of type u64
    // with the bits of field5 set
    u32, mask FIELD5_MASK(u64), from into FooBar, field5, set_field5: 10, 0;
}*/

type RawInstruction32 = u32;

bitfield! {
    pub struct Opcode7(u8);
    impl Debug;
    pub u8, get, set: 6, 0;
}

impl From<u8> for Opcode7 {
    fn from(value: u8) -> Self {
        Opcode7(value)
    }
}

impl From<Opcode7> for u8 {
    fn from(value: Opcode7) -> Self {
        value.0
    }
}

bitfield! {
    pub struct RType32(RawInstruction32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, funct7, set_funct7: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct IType32(RawInstruction32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct SType32(RawInstruction32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    imm_b7, set_imm_b7: 7;
    u8, imm_b11_8, set_imm_b11_8: 11, 8;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

type BType32 = SType32;

bitfield! {
    pub struct UType32(RawInstruction32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    u16, imm_b30_20, set_imm_b30_20: 30, 20;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct JType32(RawInstruction32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

/*
union AnyType32 {
    r: std::mem::ManuallyDrop<RType32>,
    i: std::mem::ManuallyDrop<IType32>,
    s: std::mem::ManuallyDrop<SType32>,
    b: std::mem::ManuallyDrop<BType32>,
    u: std::mem::ManuallyDrop<UType32>,
    j: std::mem::ManuallyDrop<JType32>
}
*/

enum Instruction32 {
    R(RType32),
    I(IType32),
    S(SType32),
    B(BType32),
    U(UType32),
    J(JType32),
}

type Type1 = u8;   // 1 bit  on 32 bit instructions on Risc-V 64
type Type3 = u8;   // 3 bits on 32 bit instructions on Risc-V 64
type Type4 = u8;   // ...
type Type5 = u8;
type Type6 = u8;
type Type7 = u8;
type Type11 = u16;
type Type25 = u32;

impl Instruction32 {
    fn get_opcode(&self) -> Opcode7 {
        match self {
            Instruction32::R(r) => r.opcode(),
            Instruction32::I(i) => i.opcode(),
            Instruction32::S(s) => s.opcode(),
            Instruction32::B(b) => b.opcode(),
            Instruction32::U(u) => u.opcode(),
            Instruction32::J(j) => j.opcode(),
        }
    }

    // TODO
    pub fn get_imm(&self) -> Option<u64> { None }
}
