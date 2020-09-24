#[path = "RAM.rs"] mod RAM;
#[path = "Cartridge.rs"] mod Cartridge;

pub struct CPUBus {
    ram: RAM::RAM,
    cart: Cartridge::Cartridge
}

impl CPUBus {
    pub fn new() -> CPUBus {
        let mut ret: CPUBus = CPUBus {
            ram: RAM::RAM::new(),
            cart: Cartridge::Cartridge::new()
        };
        ret.cart.load_from_file("nestest.nes".to_string());
        return ret;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if address <= 0x17FF {
            return self.ram.read(address);
        } else if address >= 0x2000 && address <= 0x3FFF {
            return 0;            
        } else if address >= 0x8000 {
            return self.cart.read(address - 0x8000);
        } else {
            return 0;
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if address < 0x17FF {
            self.ram.write(address, data);
        } else if address >= 0x8000 {
            
        } else {
            
        }
    }
}
