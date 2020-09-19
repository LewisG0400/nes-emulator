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
}
