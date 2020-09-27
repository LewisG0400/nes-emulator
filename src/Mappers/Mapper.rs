pub trait Mapper {
    fn map_cpu_address(&self, cpu_address: u16) -> u16;

    fn map_ppu_address(&self, ppu_address: u16) -> u16;
}

pub struct Mapper0 {
    pub nprg_banks: u8,
    pub nchr_banks: u8
}

impl Mapper for Mapper0 {
    fn map_cpu_address(&self, address: u16) -> u16 {
        if self.nprg_banks > 1 {
            address & 0x7FFF
        } else {
            address & 0x3FFF
        }
    }

    fn map_ppu_address(&self, address: u16) -> u16 {
        address
    }
}
