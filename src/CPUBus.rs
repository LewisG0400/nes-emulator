#[path = "RAM.rs"] mod RAM;
#[path = "Cartridge.rs"] mod Cartridge;
#[path = "PPU.rs"] mod PPU;

pub struct CPUBus {
    ram: RAM::RAM,
    ppu: PPU::PPU,
    cart: Cartridge::Cartridge
}

impl CPUBus {
    pub fn new() -> CPUBus {
        let mut ret: CPUBus = CPUBus {
            ram: RAM::RAM::new(),
            ppu: PPU::PPU::new(),
            cart: Cartridge::Cartridge::new()
        };
        ret.cart.load_from_file("res/nestest.nes".to_string());
        ret
    }

    pub fn clock_ppu(&mut self) -> bool {
        self.ppu.clock()
    }

    pub fn get_frame_buffer(&mut self) -> &Box<[u8; 61440 * 3]> {
        &self.ppu.frame_buffer
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if address <= 0x17FF {
            self.ram.read(address)
        } else if address >= 0x2000 && address <= 0x3FFF {
            self.ppu.cpu_read(address & 0x2000)
        } else if address == 0x4016 {
            //TODO: Joypad inputs
            0
        } else if address == 0x4017 {
            0
        } else if address >= 0x8000 {
            self.cart.read(address - 0x8000)
        } else {
            0
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if address < 0x17FF {
            self.ram.write(address, data);
        } else if address >= 0x2000 && address <= 0x3FFF {
            self.ppu.cpu_write(address & 0x2000, data);
        } else if address >= 0x4000 && address <= 0x4017 {
            match address {
                0x4014 => {
                    self.ppu.OAMDMA(self.ram.get_page(data));
                },
                _ => {

                }
            }
        } else if address >= 0x8000 {
            
        } else {
            
        }
    }
}
