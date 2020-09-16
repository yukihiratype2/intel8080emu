use intel8080disassembler;
use intel8080emu;

fn main() {
    let mut machine = intel8080emu::Machine::new();
    machine.excute(intel8080disassembler::Instruction{
        opcode: 0x04,
        operand1: 0b000,
        operand2: 0
    });
}
