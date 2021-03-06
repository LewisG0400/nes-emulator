use std::convert::TryInto;

pub struct RAM {
    data: Box<[u8; 2048]>
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            data: Box::new([0; 2048])
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.data[(address % 2047) as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[(address % 2047) as usize] = data;
    }

    pub fn get_page(&mut self, page: u8) -> [u8; 256] {
        self.data[((page as u16) << 8) as usize.. (((page as u16) << 8) + 256) as usize].try_into().expect("Error reading page")
    }
}
