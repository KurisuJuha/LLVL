#[derive(Clone)]
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn get(&self, address: usize) -> u8 {
        if let Some(data) = self.data.get(address) {
            *data
        } else {
            0
        }
    }

    pub fn set(&mut self, address: usize, value: u8) {
        if self.data.len() <= address {
            self.data.resize(address + 1, 0);
        }
        self.data[address] = value;
    }
}
