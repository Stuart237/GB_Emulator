pub struct MemoryBus
{
    pub memory: [u8; 0xFFFF]
    pub boot_rom: [u8; BOOT_ROM_SIZE],
    pub game_rom_bank_zero: [u8; GAME_ROM_BANK_ZERO_SIZE],
    pub game_rom_bank_n: [u8; GAME_ROM_BANK_N_SIZE],
    pub tile_ram: [u8; TILE_RAM_SIZE],
    pub background_map: [u8; BACKGROUND_MAP_SIZE],
    pub cartridge_ram: [u8; CARTRIDGE_RAM_SIZE],
    pub working_ram: [u8; WORKING_RAM_SIZE],
    pub echo_ram: [u8; ECHO_RAM_SIZE],
    pub object_attribute_memory: [u8; OBJECT_ATTRIBUTE_MEMORY_SIZE],
    pub unused_mem: [u8; UNUSED_MEMORY_SIZE],
    pub io_registers: [u8; IO_REGISTERS_SIZE],
    pub high_ram: [u8; HIGH_RAM_SIZE],
    pub interrupt_register: u8
}

pub const BOOT_ROM_START: usize = 0x0000;
pub const BOOT_ROM_END: usize = 0x00FF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_START + 1;

pub const GAME_ROM_BANK_ZERO_START: usize = 0x0000;
pub const GAME_ROM_BANK_ZERO_END: usize = 0x3FFF;
pub const GAME_ROM_BANK_ZERO_SIZE: usize = GAME_ROM_BANK_ZERO_END - GAME_ROM_BANK_ZERO_START + 1;

pub const GAME_ROM_BANK_N_START: usize = 0x4000;
pub const GAME_ROM_BANK_N_END: usize = 0x7FFF;
pub const GAME_ROM_BANK_N_SIZE: usize = GAME_ROM_BANK_N_END - GAME_ROM_BANK_N_START + 1;

pub const TILE_RAM_START: usize = 0x8000;
pub const TILE_RAM_END: usize = 0x97FF;
pub const TILE_RAM_SIZE: usize = TILE_RAM_END - TILE_RAM_START + 1;

pub const BACKGROUND_MAP_START: usize = 0x9800;
pub const BACKGROUND_MAP_END: usize = 0x9FFF;
pub const BACKGROUND_MAP_SIZE: usize = BACKGROUND_MAP_END - BACKGROUND_MAP_START + 1;

pub const CARTRIDGE_RAM_START: usize = 0xA000;
pub const CARTRIDGE_RAM_END: usize = 0xBFFF;
pub const CARTRIDGE_RAM_SIZE: usize = CARTRIDGE_RAM_END - CARTRIDGE_RAM_START + 1;

pub const WORKING_RAM_START: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_START + 1;

pub const ECHO_RAM_START: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;
pub const ECHO_RAM_SIZE: usize = ECHO_RAM_END - ECHO_RAM_START + 1;

pub const OBJECT_ATTRIBUTE_MEMORY_START: usize = 0xFE00;
pub const OBJECT_ATTRIBUTE_MEMORY_END: usize = 0xFE9F;
pub const OBJECT_ATTRIBUTE_MEMORY_SIZE: usize = OBJECT_ATTRIBUTE_MEMORY_END - OBJECT_ATTRIBUTE_MEMORY_START + 1;

pub const UNUSED_MEMORY_START: usize = 0xFE00;
pub const UNUSED_MEMORY_END: usize = 0xFEFF;
pub const UNUSED_MEMORY_SIZE: usize = UNUSED_MEMORY_END - UNUSED_MEMORY_START + 1;

pub const IO_REGISTERS_START: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;
pub const IO_REGISTERS_SIZE: usize = IO_REGISTERS_END - IO_REGISTERS_START + 1;

pub const HIGH_RAM_START: usize = 0xFF80;
pub const HIGH_RAM_END: usize = 0xFFFE;
pub const HIGH_RAM_SIZE: usize = HIGH_RAM_END - HIGH_RAM_START + 1;

pub const INTERRUPT_REGISTER: usize = 0xFFFF;

impl MemoryBus
{
    //pub fn new(&mut self) -> MemoryBus
    //{
    //    
    //}

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