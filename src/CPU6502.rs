mod CPUBus;

extern crate bitflags;


bitflags! {
    struct StatusFlags: u8 {
        const CARRY = 0b00000001;
        const ZERO = 0b00000010;
        const IRQ = 0b00000100;
        const DECIMAL = 0b00001000;
        const BRK = 0b00010000;
        const OVERFLOW = 0b01000000;
        const NEGATIVE = 0b10000000;
    }
}

pub struct CPU6502 {
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    reg_x: u8,
    reg_y: u8,
    status: StatusFlags,
    pub cycles_to_wait: u8,
    main_bus: CPUBus::CPUBus
}

#[derive(Debug)]
enum AddressingMode {
    //Indexed
    ZeroPageIndexedX,
    ZeroPageIndexedY,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    IndexedIndirect, //X
    IndirectIndexed, //Y
    //Other
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    Absolute,
    Relative,
    Indirect
}

#[derive(Debug)]
struct Instruction {
    name: &'static str,
    addressing: AddressingMode,
    cycles: u8,
    extraCycles: u8
}

const NAN: Instruction = Instruction { name: "NAN", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 };

#[allow(dead_code)]
const INSTRUCTIONS: [Instruction; 256] = [
    //----------------------------0x------------------------------------------------- 
    Instruction { name: "BRK", addressing: AddressingMode::Implicit, cycles: 7, extraCycles: 0 },
    Instruction { name: "ORA", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "ORA", addressing: AddressingMode::ZeroPageIndexedX, cycles: 3, extraCycles: 0 },
    Instruction { name: "ASL", addressing: AddressingMode::ZeroPageIndexedX, cycles: 5, extraCycles: 0 },
    NAN,
    Instruction { name: "PHP", addressing: AddressingMode::Implicit, cycles: 3, extraCycles: 0 },
    Instruction { name: "ORA", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "ASL", addressing: AddressingMode::Accumulator, cycles: 2, extraCycles: 0 },
    NAN,
    NAN,
    Instruction { name: "ORA", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "ASL", addressing: AddressingMode::Absolute, cycles: 6, extraCycles: 0 },
    NAN,

    //----------------------------1x-------------------------------------------------
    Instruction { name: "BPL", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "ORA", addressing: AddressingMode::IndirectIndexed, cycles: 5, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "ORA", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "ASL", addressing: AddressingMode::ZeroPageIndexedX, cycles: 6, extraCycles: 0 },
    NAN,
    Instruction { name: "CLC", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "ORA", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "ORA", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "ASL", addressing: AddressingMode::AbsoluteIndexedX, cycles: 7, extraCycles: 0 },
    NAN,

    //----------------------------2x-------------------------------------------------
    Instruction { name: "JSR", addressing: AddressingMode::Absolute, cycles: 6, extraCycles: 0 },
    Instruction { name: "AND", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    Instruction { name: "BIT", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "AND", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "ROL", addressing: AddressingMode::ZeroPage, cycles: 5, extraCycles: 0 },
    NAN,
    Instruction { name: "PLP", addressing: AddressingMode::Implicit, cycles: 4, extraCycles: 0 },
    Instruction { name: "AND", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "ROL", addressing: AddressingMode::Accumulator, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "BIT", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "AND", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "ROL", addressing: AddressingMode::Absolute, cycles: 6, extraCycles: 0 },
    NAN,

    //----------------------------3x-------------------------------------------------
    Instruction { name: "BMI", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "AND", addressing: AddressingMode::IndexedIndirect, cycles: 5, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "AND", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "ROL", addressing: AddressingMode::ZeroPageIndexedX, cycles: 6, extraCycles: 0 },
    NAN,
    Instruction { name: "SEC", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "AND", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "AND", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "ROL", addressing: AddressingMode::AbsoluteIndexedX, cycles: 7, extraCycles: 0 },
    NAN,

    //----------------------------4x-------------------------------------------------
    Instruction { name: "RTI", addressing: AddressingMode::Implicit, cycles: 6, extraCycles: 0 },
    Instruction { name: "EOR", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "EOR", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "LSR", addressing: AddressingMode::ZeroPage, cycles: 5, extraCycles: 0 },
    NAN,
    Instruction { name: "PHA", addressing: AddressingMode::Implicit, cycles: 3, extraCycles: 0 },
    Instruction { name: "EOR", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "LSR", addressing: AddressingMode::Accumulator, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "JMP", addressing: AddressingMode::Absolute, cycles: 3, extraCycles: 0 },
    Instruction { name: "EOR", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "LSR", addressing: AddressingMode::Absolute, cycles: 6, extraCycles: 0 },
    NAN,

    //----------------------------5x-------------------------------------------------
    Instruction { name: "BVC", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "EOR", addressing: AddressingMode::IndirectIndexed, cycles: 5, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "EOR", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "LSR", addressing: AddressingMode::ZeroPageIndexedX, cycles: 6, extraCycles: 0 },
    NAN,
    Instruction { name: "CLI", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "EOR", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "EOR", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "LSR", addressing: AddressingMode::AbsoluteIndexedX, cycles: 7, extraCycles: 0 },
    NAN,
    
    //----------------------------6x----------------------------------------------
    Instruction { name: "RTS", addressing: AddressingMode::Implicit, cycles: 6, extraCycles: 0 },
    Instruction { name: "ADC", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "ADC", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "ROR", addressing: AddressingMode::ZeroPageIndexedX, cycles: 6, extraCycles: 0 },
    NAN,
    Instruction { name: "PLA", addressing: AddressingMode::Implicit, cycles: 4, extraCycles: 0 },
    Instruction { name: "ADC", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "ROR", addressing: AddressingMode::Accumulator, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "JMP", addressing: AddressingMode::Indirect, cycles: 5, extraCycles: 0 },
    Instruction { name: "ADC", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "ROR", addressing: AddressingMode::Absolute, cycles: 6, extraCycles: 0 },
    NAN,

    //----------------------------7x----------------------------------------------
    Instruction { name: "BVS", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "ADC", addressing: AddressingMode::IndirectIndexed, cycles: 5, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "ADC", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "ROR", addressing: AddressingMode::ZeroPageIndexedX, cycles: 6, extraCycles: 0 },
    NAN,
    Instruction { name: "SEI", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "ADC", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "ADC", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "ROR", addressing: AddressingMode::AbsoluteIndexedX, cycles: 7, extraCycles: 0 },
    NAN,

    //----------------------------8x----------------------------------------------
    NAN,
    Instruction { name: "STA", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    Instruction { name: "STY", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "STA", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "STX", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    NAN,
    Instruction { name: "DEY", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "TXA", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "STY", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "STA", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "STX", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    NAN,

    //----------------------------9x----------------------------------------------
    Instruction { name: "BCC", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "STA", addressing: AddressingMode::IndirectIndexed, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    Instruction { name: "STY", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "STA", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "STX", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    NAN,
    Instruction { name: "TYA", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "STA", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 0 },
    Instruction { name: "TXS", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    NAN,
    NAN,
    Instruction { name: "STA", addressing: AddressingMode::AbsoluteIndexedX, cycles: 5, extraCycles: 0 },
    NAN,
    NAN,

    //----------------------------Ax----------------------------------------------
    Instruction { name: "LDY", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "LDA", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    Instruction { name: "LDX", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "LDY", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "LDA", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "LDX", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    NAN,
    Instruction { name: "TAY", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "LDA", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "TAX", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "LDY", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "LDA", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "LDX", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    NAN,

    //----------------------------Bx----------------------------------------------
    Instruction { name: "BCS", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "LDA", addressing: AddressingMode::IndirectIndexed, cycles: 5, extraCycles: 1 },
    NAN,
    NAN,
    Instruction { name: "LDY", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "LDA", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "LDX", addressing: AddressingMode::ZeroPageIndexedY, cycles: 4, extraCycles: 0 },
    NAN,
    Instruction { name: "CLV", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "LDA", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 1 },
    Instruction { name: "TSX", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "LDY", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "LDA", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "LDX", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 1 },
    NAN,

    //----------------------------Cx----------------------------------------------
    Instruction { name: "CPY", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "CMP", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    Instruction { name: "CPY", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "CMP", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "DEC", addressing: AddressingMode::ZeroPage, cycles: 5, extraCycles: 0 },
    NAN,
    Instruction { name: "INY", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "CMP", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "DEX", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "CPY", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "CMP", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "DEC", addressing: AddressingMode::Absolute, cycles: 6, extraCycles: 0 },
    NAN,

    //----------------------------Dx----------------------------------------------
    Instruction { name: "BNE", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "CMP", addressing: AddressingMode::IndirectIndexed, cycles: 5, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "CMP", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "DEC", addressing: AddressingMode::ZeroPageIndexedX, cycles: 6, extraCycles: 0 },
    NAN,
    Instruction { name: "CLD", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "CMP", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "CMP", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "DEC", addressing: AddressingMode::AbsoluteIndexedX, cycles: 7, extraCycles: 0 },
    NAN,

    //----------------------------Ex----------------------------------------------
    Instruction { name: "CPX", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "SBC", addressing: AddressingMode::IndexedIndirect, cycles: 6, extraCycles: 0 },
    NAN,
    NAN,
    Instruction { name: "CPX", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "SBC", addressing: AddressingMode::ZeroPage, cycles: 3, extraCycles: 0 },
    Instruction { name: "INC", addressing: AddressingMode::ZeroPage, cycles: 5, extraCycles: 0 },
    NAN,
    Instruction { name: "INX", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "SBC", addressing: AddressingMode::Immediate, cycles: 2, extraCycles: 0 },
    Instruction { name: "NOP", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    NAN,
    Instruction { name: "CPX", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "SBC", addressing: AddressingMode::Absolute, cycles: 4, extraCycles: 0 },
    Instruction { name: "INC", addressing: AddressingMode::Absolute, cycles: 6, extraCycles: 0 },
    NAN,

    //----------------------------Fx----------------------------------------------
    Instruction { name: "BEQ", addressing: AddressingMode::Relative, cycles: 2, extraCycles: 1 },
    Instruction { name: "SBC", addressing: AddressingMode::IndirectIndexed, cycles: 5, extraCycles: 1 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "SBC", addressing: AddressingMode::ZeroPageIndexedX, cycles: 4, extraCycles: 0 },
    Instruction { name: "INC", addressing: AddressingMode::ZeroPageIndexedX, cycles: 6, extraCycles: 0 },
    NAN,
    Instruction { name: "SED", addressing: AddressingMode::Implicit, cycles: 2, extraCycles: 0 },
    Instruction { name: "SBC", addressing: AddressingMode::AbsoluteIndexedY, cycles: 4, extraCycles: 0 },
    NAN,
    NAN,
    NAN,
    Instruction { name: "SBC", addressing: AddressingMode::AbsoluteIndexedX, cycles: 4, extraCycles: 1 },
    Instruction { name: "INC", addressing: AddressingMode::AbsoluteIndexedX, cycles: 7, extraCycles: 0 },
    NAN
];

struct Executable {
    name: &'static str,
    target: u16,
    data: u8,
    cycles: u8
}


fn is_negative(number: u8) -> bool {
    return number & 0b10000000 == 0b10000000;
}

impl CPU6502 {
    pub fn new() -> CPU6502 {
        CPU6502 {
            program_counter: 0xc000,
            stack_pointer: 0xff,
            accumulator: 0,
            reg_x: 0,
            reg_y: 0,
            status: StatusFlags::empty(),
            cycles_to_wait: 0,
            main_bus: CPUBus::CPUBus::new()
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        return self.main_bus.read(address);
    }

    pub fn write (&mut self, address: u16, data: u8) {
        self.main_bus.write(address, data);
    }

    fn decode_next_instruction(&mut self) -> Executable {
        let opcode = self.read(self.program_counter);
        println!("Status:");
        println!("A: {:2x}, X: {:2x}, Y: {:2x}, PC: {:4x}, SP: {:2x}, status: {:8b}", self.accumulator, self.reg_x, self.reg_y, self.program_counter, self.stack_pointer, self.status);
        let instruction: &Instruction = &INSTRUCTIONS[opcode as usize];
        let mut ret_executable: Executable = Executable {name: instruction.name, target: 0, data: 0, cycles: 0};
        println!("OP: {}, {:?}", opcode, instruction);
        match instruction.addressing {
            AddressingMode::Immediate => {
                ret_executable.data = self.read(self.program_counter + 1);
                ret_executable.cycles = instruction.cycles;
                self.program_counter += 2;
            },
            AddressingMode::Absolute => {
                //TODO: optimise?
                ret_executable.target = self.read(self.program_counter + 1) as u16 + self.read(self.program_counter + 2) as u16 * 256;
                ret_executable.data = self.read(ret_executable.target);
                ret_executable.cycles = instruction.cycles;
                self.program_counter += 3;
            },
            AddressingMode::AbsoluteIndexedX => {
                let low = self.read(self.program_counter + 1);
                let high = self.read(self.program_counter + 2);

                //If did_overflow is set, we spill into the next page and need an oops cycle
                let (low, did_overflow) = low.overflowing_add(self.reg_x);
                if did_overflow {
                    //self.status.insert(StatusFlags::CARRY);
                    ret_executable.cycles = instruction.cycles + 1;
                    ret_executable.target = low as u16 + (high + 1) as u16;
                } else {
                    ret_executable.cycles = instruction.cycles;
                    ret_executable.target = low as u16 + high as u16;
                }
                ret_executable.data = self.read(ret_executable.target);
                self.program_counter += 3;
            },
            AddressingMode::AbsoluteIndexedY => {
                let low = self.read(self.program_counter + 1);
                let high = self.read(self.program_counter + 2);

                //If did_overflow is set, we spill into the next page and need an oops cycle
                let (low, did_overflow) = low.overflowing_add(self.reg_y);
                if did_overflow {
                    //self.status.insert(StatusFlags::CARRY);
                    ret_executable.cycles = instruction.cycles + 1;
                    ret_executable.target = low as u16 + (high + 1) as u16;
                } else {
                    ret_executable.cycles = instruction.cycles;
                    ret_executable.target = low as u16 + high as u16;
                }
                ret_executable.data = self.read(ret_executable.target);
                self.program_counter += 3;
            },
            AddressingMode::ZeroPage => {
                ret_executable.target = self.read(self.program_counter + 1) as u16;
                ret_executable.data = self.read(ret_executable.target);
                
                ret_executable.cycles = instruction.cycles;
                self.program_counter += 2;
            },
            AddressingMode::ZeroPageIndexedX => {
                //We wrap and ignore carry here
                ret_executable.target = self.read(self.program_counter + 1).wrapping_add(self.reg_x) as u16;
                ret_executable.data = self.read(ret_executable.target);

                ret_executable.cycles = instruction.cycles;
                self.program_counter += 2;
            },
            AddressingMode::ZeroPageIndexedY => {
                //We wrap and ignore carry here
                ret_executable.target = self.read(self.program_counter + 1).wrapping_add(self.reg_y) as u16;
                ret_executable.data = self.read(ret_executable.target);

                ret_executable.cycles = instruction.cycles;
                self.program_counter += 2;
            },
            AddressingMode::Relative => {
                //TODO: Implement the oops
                ret_executable.data = self.read(self.program_counter + 1);

                ret_executable.cycles = instruction.cycles;
                self.program_counter += 2;
            },
            AddressingMode::Indirect => {
                //The operand here is a pointer to the high byte of the actual target
                let loc_low = self.read(self.program_counter + 1);
                let loc_high = self.read(self.program_counter + 2);

                //Find actual target by reading operand address and the address after
                let loc = loc_low as u16 + loc_high as u16 * 256;

                ret_executable.target = self.read(loc) as u16 + self.read(loc + 1) as u16;

                ret_executable.cycles = instruction.cycles;
                self.program_counter += 3;
            },
            AddressingMode::IndexedIndirect => {
                //The target here has its low byte in $(operand+X) and high byte in $(operand+X+1)
                let operand = self.read(self.program_counter + 1);
                
                ret_executable.target = self.read((operand + self.reg_x) as u16) as u16 + self.read((operand + self.reg_x + 1) as u16) as u16 * 256;

                ret_executable.cycles = instruction.cycles;
                self.program_counter += 2;
            },
            AddressingMode::IndirectIndexed => {
                //The target here is the number found in the (operand address and the next one)
                //incremented by Y
                let operand = self.read(self.program_counter + 1);

                let target = self.read(operand as u16) as u16 + self.read((operand + 1) as u16) as u16 * 256;
                let (target, did_overflow) = target.overflowing_add(self.reg_y as u16);

                //TODO: Figure out this oops
                if did_overflow {
                    ret_executable.cycles = instruction.cycles + 1;
                } else {
                    ret_executable.cycles = instruction.cycles;
                }

                ret_executable.target = target;
            },
            AddressingMode::Implicit => {
                ret_executable.cycles = instruction.cycles;
                self.program_counter += 1;
            },
            AddressingMode::Accumulator => {
                ret_executable.cycles = instruction.cycles;
                self.program_counter += 1;
            }
        }
    return ret_executable;
    }

    fn is_flag_set(&self, flag: StatusFlags) -> bool {
        return self.status & flag == flag;
    }

    fn execute(&mut self, executable: Executable) {
        //This function will take up one cycle so we need to artificially wait for the rest
        self.cycles_to_wait = executable.cycles - 1;
        match executable.name {
            "ADC" => {
                let (result, did_overflow) = self.accumulator.overflowing_add(executable.data);
                if did_overflow {
                    self.status.insert(StatusFlags::CARRY);
                }

                self.status.set(StatusFlags::ZERO, result == 0);

                //TODO: fix this mess
                let initial_acc_sign = self.accumulator & 0b10000000;
                let data_sign = executable.data & 0b10000000;
                let result_sign = result & 0b10000000;

                self.status.set(StatusFlags::OVERFLOW,!(initial_acc_sign ^ data_sign) & (initial_acc_sign ^ result_sign) == 0b10000000);

                self.status.set(StatusFlags::NEGATIVE, result_sign == 0b1000000);
                self.accumulator = result;
            },
            "AND" => {
                let result = self.accumulator & executable.data;
                if result == 0 { self.status.insert(StatusFlags::ZERO); }
                self.status.set(StatusFlags::CARRY, is_negative(result));
                self.accumulator = result;
            },
            "ASL" => {
                self.status.set(StatusFlags::CARRY, is_negative(self.accumulator));
                self.accumulator <<= 1;
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.accumulator));
            },
            "BCC" => {
                if !self.is_flag_set(StatusFlags::CARRY) {
                    let new_pc = self.program_counter + executable.data as u16;
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },
            "BCS" => {
                if self.is_flag_set(StatusFlags::CARRY) {
                    let new_pc = self.program_counter + executable.data as u16;
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },            
            "BEQ" => {
                if self.is_flag_set(StatusFlags::ZERO) {
                    let new_pc = self.program_counter + executable.data as u16;
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },
            "BIT" => {
                self.status.set(StatusFlags::OVERFLOW, executable.data & 0b01000000 == 0b01000000);
                self.status.set(StatusFlags::NEGATIVE, executable.data & 0b10000000 == 0b10000000);

                let result = self.accumulator & executable.data;

                self.status.set(StatusFlags::ZERO, result == 0);

                self.accumulator = result;
            },
            "BMI" => {
                if self.is_flag_set(StatusFlags::NEGATIVE) {
                    let new_pc = self.program_counter + executable.data as u16;
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },
            "BNE" => {
                if !self.is_flag_set(StatusFlags::ZERO) {
                    let new_pc = (self.program_counter as i32 + (executable.data as i8) as i32) as u16;
                    println!("Old PC: {:2x}, Data: {:2x}, New PC: {:2x}", self.program_counter, executable.data, new_pc);
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },
            "BPL" => {
                if !self.is_flag_set(StatusFlags::NEGATIVE) {
                    let new_pc = self.program_counter + executable.data as u16;
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },
            "BRK" => {
                //Low byte
                self.push_stack((self.program_counter | 0b0000000011111111) as u8);
                //High byte
                self.push_stack((self.program_counter >> 8) as u8);
                self.push_stack(self.status.bits());
                self.status.insert(StatusFlags::BRK);
            },
            "BVC" => {
                if !self.is_flag_set(StatusFlags::OVERFLOW) {
                    let new_pc = self.program_counter + executable.data as u16;
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },
            "BVS" => {
                if self.is_flag_set(StatusFlags::OVERFLOW) {
                    let new_pc = self.program_counter + executable.data as u16;
                    if new_pc & 0xff00 != self.program_counter & 0xff00 { self.cycles_to_wait += 1; }
                    self.program_counter = new_pc;
                    self.cycles_to_wait += 1;
                }
            },
            "CLC" => {
                self.status.remove(StatusFlags::CARRY);
            },
            "CLD" => {
                self.status.remove(StatusFlags::DECIMAL);
            },
            "CLI" => {
                self.status.remove(StatusFlags::IRQ);
            },
            "CLV" => {
                self.status.remove(StatusFlags::OVERFLOW);
            },
            "CMP" => {
                let result = self.accumulator - executable.data;
                if result > 0 {
                    self.status.insert(StatusFlags::CARRY);
                } else if result == 0 {
                    self.status.insert(StatusFlags::ZERO);
                }
                
                self.status.set(StatusFlags::NEGATIVE, is_negative(result));
            },
            "CPX" => {
                let result = self.reg_x - executable.data;
                if result > 0 {
                    self.status.insert(StatusFlags::CARRY);
                } else if result == 0 {
                    self.status.insert(StatusFlags::ZERO);
                }
                
                self.status.set(StatusFlags::NEGATIVE, is_negative(result));
            },
            "CPY" => {
                let result = self.reg_y - executable.data;
                if result > 0 {
                    self.status.insert(StatusFlags::CARRY);
                } else if result == 0 {
                    self.status.insert(StatusFlags::ZERO);
                }
                
                self.status.set(StatusFlags::NEGATIVE, is_negative(result));
            }, 
            "DEC" => {
                let data = self.read(executable.target);
                let data = data.wrapping_sub(1);

                self.status.set(StatusFlags::ZERO, data == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(data));
                self.write(executable.target, data);
            },
            "DEX" => {
                self.reg_x = self.reg_x.wrapping_sub(1);
                self.status.set(StatusFlags::ZERO, self.reg_x == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.reg_x));
            },
            "DEY" => {
                self.reg_y = self.reg_y.wrapping_sub(1);
                self.status.set(StatusFlags::ZERO, self.reg_y == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.reg_y));
            }, 
            "EOR" => {
                self.accumulator &= executable.data;
                self.status.set(StatusFlags::ZERO, self.accumulator == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.accumulator));
            },
            "INC" => {
                let data = self.read(executable.target);
                let data = data.wrapping_add(1);

                self.status.set(StatusFlags::ZERO, data == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(data));

                self.write(executable.target, data);
            },
            "INX" => {
                self.reg_x = self.reg_x.wrapping_sub(1);
                self.status.set(StatusFlags::ZERO, self.reg_x == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.reg_x));
            }, 
            "INY" => {
                self.reg_y = self.reg_y.wrapping_sub(1);
                self.status.set(StatusFlags::ZERO, self.reg_y == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.reg_y));
            }, 
            "JMP" => {
                self.program_counter = executable.target;
            },
            "JSR" => {
                //Low byte
                self.push_stack((self.program_counter | 0b0000000011111111) as u8);
                //High byte
                self.push_stack((self.program_counter >> 8) as u8);

                self.program_counter = executable.target;
            },
            "LDA" => {
                self.status.set(StatusFlags::ZERO, executable.data == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(executable.data));

                self.accumulator = executable.data;
            },
            "LDX" => {
                self.status.set(StatusFlags::ZERO, executable.data == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(executable.data));

                self.reg_x = executable.data;
            },            
            "LDY" => {
                self.status.set(StatusFlags::ZERO, executable.data == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(executable.data));

                self.reg_y = executable.data;
            },
            "LSR" => {
                let result: u8;
                if executable.target != 0 {
                    self.status.set(StatusFlags::CARRY, executable.data & 0b00000001 == 0b00000001);
                    result = executable.data >> 1;
                    self.status.set(StatusFlags::ZERO, result == 0);
                    self.status.set(StatusFlags::NEGATIVE, is_negative(result));
                    self.write(executable.target, result);
                } else {
                    self.status.set(StatusFlags::CARRY, self.accumulator & 0b00000001 == 0b00000001);
                    result = self.accumulator >> 1;
                    self.status.set(StatusFlags::ZERO, result == 0);
                    self.status.set(StatusFlags::NEGATIVE, is_negative(result));
                    self.accumulator = result;
                }
            },
            "NAN" => {
                
            },
            "ORA" => {
                self.accumulator |= executable.data;
                self.status.set(StatusFlags::ZERO, self.accumulator == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.accumulator));
            },
            "PHA" => {
                self.push_stack(self.accumulator);
            },
            "PHP" => {
                self.push_stack(self.status.bits());
            },
            "PLA" => {
                self.accumulator = self.pop_stack();
            },
            "PLP" => {
                self.status = StatusFlags::from_bits_truncate(self.pop_stack());
            },
            "ROL" => {
                let result: u8;
                if executable.target != 0 {
                    self.status.set(StatusFlags::CARRY, executable.data & 0b00000001 == 0b00000001);
                    result = executable.data >> 1;
                    self.status.set(StatusFlags::ZERO, result == 0);
                    self.status.set(StatusFlags::NEGATIVE, is_negative(result));
                    self.write(executable.target, result);
                } else {
                    self.status.set(StatusFlags::CARRY, self.accumulator & 0b00000001 == 0b00000001);
                    result = self.accumulator >> 1;
                    self.status.set(StatusFlags::ZERO, result == 0);
                    self.status.set(StatusFlags::NEGATIVE, is_negative(result));
                    self.accumulator = result;
                }
            },
            "ROR" => {
                let mut result: u8;
                if executable.target != 0 {
                    let new_carry = executable.data & 0b00000001;
                    result = executable.data >> 1;
                    result |= (self.status & StatusFlags::CARRY).bits();

                    self.status.set(StatusFlags::CARRY, new_carry == 0b00000001);
                    self.status.set(StatusFlags::ZERO, result == 0);
                    self.status.set(StatusFlags::NEGATIVE, is_negative(result));

                    self.write(executable.target, result);
                } else {
                    let new_carry = self.accumulator & 0b00000001;
                    result = self.accumulator >> 1;
                    result |= (self.status & StatusFlags::CARRY).bits();

                    self.status.set(StatusFlags::CARRY, new_carry == 0b00000001);
                    self.status.set(StatusFlags::ZERO, result == 0);
                    self.status.set(StatusFlags::NEGATIVE, is_negative(result));

                    self.accumulator = result;
                }                
            },
            "RTI" => {
                self.status = StatusFlags::from_bits_truncate(self.pop_stack());
                self.program_counter = self.pop_stack() as u16 * 256 + self.pop_stack() as u16;
            },
            "RTS" => {
               self.program_counter = self.pop_stack() as u16 * 256 + self.pop_stack() as u16 - 1;
            },
            "SBC" => {
                let amt = if self.status & StatusFlags::CARRY == StatusFlags::CARRY { executable.data } else { executable.data + 1 };
                let (result, did_overflow) = self.accumulator.overflowing_sub(amt);

                self.status.set(StatusFlags::CARRY, !did_overflow);
                self.status.set(StatusFlags::ZERO, result == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(result));

                self.accumulator = result;
            },
            "SEC" => {
                self.status.insert(StatusFlags::CARRY);
            },
            "SED" => {
                self.status.insert(StatusFlags::DECIMAL);
            },
            "SEI" => {
                self.status.insert(StatusFlags::IRQ);
            },
            "STA" => {
                self.write(executable.target, self.accumulator);
            },
            "STX" => {
                self.write(executable.target, self.reg_x);
            },
            "STY" => {
                self.write(executable.target, self.reg_y);
            },
            "TAX" => {
                self.reg_x = self.accumulator;
                self.status.set(StatusFlags::ZERO, self.reg_x == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.reg_x));
            },
            "TAY" => {
                self.reg_y = self.accumulator;
                self.status.set(StatusFlags::ZERO, self.reg_y == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.reg_y));
            },
            "TSX" => {
                self.reg_x = self.stack_pointer;
                self.status.set(StatusFlags::ZERO, self.reg_x == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.reg_x));
            },
            "TXA" => {
                self.accumulator = self.reg_x;
                self.status.set(StatusFlags::ZERO, self.accumulator == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.accumulator));
            },
            "TXS" => {
                self.stack_pointer = self.reg_x;
            },
            "TYA" => {
                self.accumulator = self.reg_y;
                self.status.set(StatusFlags::ZERO, self.accumulator == 0);
                self.status.set(StatusFlags::NEGATIVE, is_negative(self.accumulator));
            },
            _ => {

            }
        }
    }

    pub fn clock(&mut self) {
        if self.cycles_to_wait == 0 {
            let exec: Executable = self.decode_next_instruction();
            self.cycles_to_wait = exec.cycles;
            self.execute(exec);
            println!("Executing, {} clocks left", self.cycles_to_wait);
        } else {
            self.cycles_to_wait -= 1;
        }
    }

    fn push_stack(&mut self, data: u8) {
        self.write(self.stack_pointer as u16 * 256, data);
        self.stack_pointer -= 1;
    }

    fn pop_stack(&mut self) -> u8 {
        self.stack_pointer.wrapping_add(1);
        return self.read(self.stack_pointer as u16 * 256);
    }

    pub fn print_acc(&self) {
        println!("PC: {}, ACC: {}", self.program_counter, self.accumulator);
    }
}
