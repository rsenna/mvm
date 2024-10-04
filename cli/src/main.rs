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

#![feature(associated_type_defaults)]

use monologvm_core::machine::Machine;

fn main() {
    println!("Hello, world!");

    let machine = Machine::new();
    let ram_2 = machine.hart.peek(2, 1).unwrap();
    let instruction = machine.hart.fetch();
    let opcode = instruction.get_opcode();

    let imm = instruction
        .get_imm()
        .map(|it| it.to_string())
        .unwrap_or("(None)".to_string());

    println!(
        "ram[2] = {}, instruction = {}, opcode = {}, imm = {}",
        ram_2, instruction, opcode, imm
    );
}
