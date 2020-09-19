#[path = "RAM.rs"] mod RAM;

pub struct CPUBus {
    ram: RAM::RAM,
}

impl CPUBus {
    pub fn new() -> CPUBus {
        CPUBus {
            ram: RAM::RAM::new()
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        return self.ram.read(address);
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.ram.write(address, data);
    }
}
