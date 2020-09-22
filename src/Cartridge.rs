use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Cartridge {
    data: Box<[u8; 32768]>
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
            data: Box::new([0; 32768])
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        if address < 0x4000 {
            return self.data[(address) as usize];
        } else {
            return self.data[(address - 0x4000) as usize];
        }
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
