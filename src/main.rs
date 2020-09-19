#[macro_use]
extern crate bitflags;

use std::{thread, time};

#[path = "CPU6502.rs"] mod CPU6502;

fn main() {
    let mut cpu = CPU6502::CPU6502::new();

    cpu.write(0x0000, 0xa9);
    cpu.write(0x0001, 0xf4);
    cpu.write(0x0002, 0x8d);
    cpu.write(0x0003, 0x22);
    cpu.write(0x0004, 0x22);
    cpu.write(0x0005, 0xa9);
    cpu.write(0x0006, 0xaa);
    cpu.write(0x0007, 0x8d);
    cpu.write(0x0008, 0x23);
    cpu.write(0x0009, 0x22);

    for i in 0..100 {
        cpu.clock();
        cpu.print_acc();
        thread::sleep(time::Duration::from_secs(1));
    }
}
