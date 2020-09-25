
pub struct PPU {
    PPUCTRL: u8,
    PPUMASK: u8,
    PPUSTATUS: u8,
    OAMADDR: u8,
    PPUSCROLL: u16,
    PPUADDR: u16,

    VRAM: Box<[u8; 16384]>,
    SPR_RAM: Box<[u8; 256]>,

    //This is for loading in the PPUADDR in 2 writes
    temp_address: u16,
    first_write: bool
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            PPUCTRL: 0,
            PPUMASK: 0,
            PPUSTATUS: 0,
            OAMADDR: 0,
            PPUSCROLL: 0,
            PPUADDR: 0,

            VRAM: Box::new([0; 16384]),
            SPR_RAM: Box::new([0; 2048]),

            temp_address: 0,
            first_write: true
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        0
    }

    pub fn write(&mut self, address: u16, data: u8) {

    } 

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        match address {
            0x0000 => {
                //Meant to be write only but no harm
                self.PPUCTRL
            },
            0x0001 => {
                //So is this
                self.PPUMASK
            },
            0x0002 => {
                //End VBlank
                self.PPUSTATUS |= 0b10000000;
                //Clear the latch
                self.first_write = true;
                self.PPUSTATUS
            },
            0x0003 => {
                //And this
                self.OAMADDR
            },
            0x0004 => {
                self.SPR_RAM[self.OAMADDR as usize]
            },
            0x0005 => {
                0
            },
            0x0006 => {
                0
            },
            0x0007 => {
                self.VRAM[self.PPUADDR as usize]
            },
            _ => {
                0
            }
        }
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0x2000 => {
                self.PPUCTRL = data;
            },
            0x2001 => {
                self.PPUMASK = data;
            },
            0x2002 => {
                self.PPUSTATUS = data;
            },
            0x2003 => {
                self.OAMADDR = data;
            },
            0x2004 => {
                self.SPR_RAM[self.OAMADDR as usize] = data;
                //TODO: check if this actually overflows (probably does)
                self.OAMADDR = self.OAMADDR.wrapping_add(1);
            },
            0x2005 => {
                if self.first_write {
                    self.temp_address = (data as u16) << 8;
                    self.first_write = false;
                } else {
                    self.temp_address = self.temp_address + data as u16;
                    self.PPUSCROLL = self.temp_address;
                    self.first_write = true;
                }
            },
            0x2006 => {
                if self.first_write {
                    self.temp_address = (data as u16) << 8;
                    self.first_write = false;
                } else {
                    self.temp_address = self.temp_address + data as u16;
                    self.PPUADDR = self.temp_address;
                    self.first_write = true;
                }
            },
            0x2007 => {
                //TODO: Make sure screen is off first
                self.VRAM[self.PPUADDR as usize] = data;
                //If this bit is set to 0 we're going across so add 1. Else we're going down a line
                //so add 32
                self.PPUADDR = self.PPUADDR.wrapping_add(if self.PPUCTRL & 0b00000100 == 0 {1} else {32});
            },
            _ => {
                
            }
        }        
    }

    //This copies a page from cpu RAM to SPR_RAM
    ////TODO: optimise this
    pub fn OAMDMA(&mut self, data: [u8; 256]) {
        self.SPR_RAM = Box::new(data);
    }
}
