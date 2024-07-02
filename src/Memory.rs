use crate::{InterruptFlags::InterruptFlags, PPU};

pub struct MemoryBus
{
    pub boot_rom: [u8; BOOT_ROM_SIZE],
    pub game_rom_bank_zero: [u8; GAME_ROM_BANK_ZERO_SIZE],
    pub game_rom_bank_n: [u8; GAME_ROM_BANK_N_SIZE],
    pub cartridge_ram: [u8; CARTRIDGE_RAM_SIZE],
    pub working_ram: [u8; WORKING_RAM_SIZE],
    pub echo_ram: [u8; ECHO_RAM_SIZE],
    pub ppu: PPU::PPU,
    pub object_attribute_memory: [u8; OBJECT_ATTRIBUTE_MEMORY_SIZE],
    pub unused_mem: [u8; UNUSED_MEMORY_SIZE],
    pub high_ram: [u8; HIGH_RAM_SIZE],
    pub interrupt_register: InterruptFlags,
    pub interrupt_flag: InterruptFlags,
    pub boot_rom_enabled: bool
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

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

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

pub const UNUSED_MEMORY_START: usize = 0xFEA0;
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
    pub fn new(boot_rom_buffer: Vec<u8>, game_rom_buffer: Vec<u8>) -> Self
    {
        let boot_rom = {if boot_rom_buffer.len() != BOOT_ROM_SIZE
        {
            panic!("INVALID BOOT ROM.");
        }
        else 
        {
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);
            boot_rom
        }};

        let mut game_rom_bank_zero = [0; GAME_ROM_BANK_ZERO_SIZE];
        for i in 0..GAME_ROM_BANK_ZERO_SIZE
        {
            game_rom_bank_zero[i] = game_rom_buffer[i];
        }
        let mut game_rom_bank_n = [0; GAME_ROM_BANK_N_SIZE];
        for i in 0..GAME_ROM_BANK_N_SIZE
        {
            game_rom_bank_n[i] = game_rom_buffer[GAME_ROM_BANK_N_SIZE + i];
        }

        Self
        {
            boot_rom,
            game_rom_bank_zero,
            game_rom_bank_n,
            ppu: PPU::PPU::new(),
            cartridge_ram: [0; CARTRIDGE_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            echo_ram: [0; ECHO_RAM_SIZE],
            object_attribute_memory: [0; OBJECT_ATTRIBUTE_MEMORY_SIZE],
            unused_mem: [0; UNUSED_MEMORY_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_register: InterruptFlags::new(),
            interrupt_flag: InterruptFlags::new(),
            boot_rom_enabled: true
        }
    }

    pub fn disable_boot_rom(&mut self)
    {
        self.boot_rom_enabled = false;
    }

    pub fn read_byte(&mut self, address: u16) -> u8
    {
        let address = address as usize;
        match address
        {
            BOOT_ROM_START..=BOOT_ROM_END =>
            {
                if self.boot_rom_enabled
                {self.boot_rom[address]}
                else
                {self.game_rom_bank_zero[address]}
            }
            GAME_ROM_BANK_ZERO_START..=GAME_ROM_BANK_ZERO_END => self.game_rom_bank_zero[address],
            GAME_ROM_BANK_N_START..=GAME_ROM_BANK_N_END => self.game_rom_bank_n[address - GAME_ROM_BANK_N_START],
            VRAM_START..=VRAM_END => self.ppu.read_from_vram(address),
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge_ram[address - CARTRIDGE_RAM_START],
            WORKING_RAM_START..=WORKING_RAM_END => self.working_ram[address - WORKING_RAM_START],
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram[address - ECHO_RAM_START],
            OBJECT_ATTRIBUTE_MEMORY_START..=OBJECT_ATTRIBUTE_MEMORY_END => self.object_attribute_memory[address - OBJECT_ATTRIBUTE_MEMORY_START],
            UNUSED_MEMORY_START..=UNUSED_MEMORY_END => 0,
            IO_REGISTERS_START..=IO_REGISTERS_END => self.read_io_registers(address),
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram[address - HIGH_RAM_START],
            INTERRUPT_REGISTER => self.interrupt_register.to_byte(),
            _ => {panic!("UNKNOWN ADDRESS 0x{:x}", address)}
        }
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
        let address = address as usize;
        match address
        {
            GAME_ROM_BANK_ZERO_START..=GAME_ROM_BANK_ZERO_END => {self.game_rom_bank_zero[address] = value;},
            GAME_ROM_BANK_N_START..=GAME_ROM_BANK_N_END => {self.game_rom_bank_n[address - GAME_ROM_BANK_N_START] = value;},
            VRAM_START..=VRAM_END => {self.ppu.write_to_vram(address, value);},
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => {self.cartridge_ram[address - CARTRIDGE_RAM_START] = value;},
            WORKING_RAM_START..=WORKING_RAM_END => {self.working_ram[address - WORKING_RAM_START] = value;},
            ECHO_RAM_START..=ECHO_RAM_END => {self.echo_ram[address - ECHO_RAM_START] = value;},
            OBJECT_ATTRIBUTE_MEMORY_START..=OBJECT_ATTRIBUTE_MEMORY_END => {self.object_attribute_memory[address - OBJECT_ATTRIBUTE_MEMORY_START] = value;},
            UNUSED_MEMORY_START..=UNUSED_MEMORY_END => {panic!("ATTEMPT TO WRITE TO UNUSED MEMORY ADDRESS 0x{:x}", address);},
            IO_REGISTERS_START..=IO_REGISTERS_END => {self.write_io_registers(address, value)},
            HIGH_RAM_START..=HIGH_RAM_END => {self.high_ram[address - HIGH_RAM_START] = value;},
            INTERRUPT_REGISTER => {self.interrupt_register.from_byte(value);},
            _ => {panic!("UNKNOWN ADDRESS 0x{:x}", address)}
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16)
    {
        let lo = (value & 0x00FF) as u8;
        let hi = ((value & 0xFF00) >> 8) as u8;
        self.write_byte(address, lo);
        self.write_byte(address + 1, hi);
    }

    pub fn read_io_registers(&mut self, address: usize) -> u8
    {
        match address
        {
            0xFF00 => {},
        }
    }

    pub fn write_io_registers(&mut self, address: usize, value: u8)
    {

    }
}