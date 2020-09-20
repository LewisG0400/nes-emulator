#[path = "CPU6502.rs"] mod CPU6502;
#[path = "CPUBus.rs"] mod CPUBus;

pub struct Emulator {
    cpu: CPU6502::CPU6502,
    cpu_bus: CPUBus::CPUBus,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU6502::CPU6502::new(),
            cpu_bus: CPUBus::CPUBus::new()
        }
    }
}
