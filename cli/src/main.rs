#![feature(associated_type_defaults)]

use monologvm_core::machine::Machine;

fn main() {
    println!("Hello, world!");

    let machine = Machine::new();
    let ram_2 = machine.hart.read_byte_ram(2).unwrap();
    let instruction = machine.hart.fetch();
    let opcode = instruction.get_opcode();

    let imm = instruction.get_imm()
        .map(|it| it.to_string())
        .unwrap_or("(None)".to_string());

    println!("ram[2] = {}, instruction = {}, opcode = {}, imm = {}", ram_2, instruction, opcode, imm);
}
