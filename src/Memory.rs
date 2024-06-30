pub struct MemoryBus
{
    pub memory: [u8; 0xFFFF]
}

impl MemoryBus
{
    pub fn read_byte(&mut self, address: u16) -> u8
    {
        self.memory[address as usize]
    }

    pub fn read_word(&mut self, address: u16) -> u16
    {
        let lo = self.read_byte(address) as u16;
        let hi = (self.read_byte(address + 1) << 8) as u16;
        let word = lo | hi;
        word
    }

    pub fn write_byte(&mut self, address: u16, value: u8)
    {
        self.memory[address as usize] = value;
    }

    pub fn write_word(&mut self, address: u16, value: u16)
    {
        let lo = (value & 0x00FF) as u8;
        let hi = ((value & 0xFF00) >> 8) as u8;
        self.memory[address as usize] = lo;
        self.memory[(address + 1) as usize] = hi;
    }
}