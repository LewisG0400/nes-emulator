use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[path = "Mappers/Mapper.rs"] mod Mapper;

pub struct Cartridge {
    data: Box<[u8; 32768]>,
    mapper: Box<dyn Mapper::Mapper>
}

impl Cartridge {
    pub fn new() -> Cartridge {
        let amapper = Box::new(Mapper::Mapper0 {
            nprg_banks: 0,
            nchr_banks: 0
        });
        Cartridge {
            data: Box::new([0; 32768]),
            mapper: amapper
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.data[self.mapper.map_cpu_address(address) as usize]
    }

    pub fn write(&mut self, address: u16) {
        //This is ROM but used for bank switching
    }

    pub fn load_from_file(&mut self, path: String) -> std::io::Result<()> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut content = Vec::new();
        reader.read_to_end(&mut content)?;

        let mut current = 0;

        for token in &content[16..] {
            print!("{:02x} ", token);
            self.data[current] = *token;
            current += 1;
        }
        Ok(())
    }
}
