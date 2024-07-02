pub mod CPU;
pub mod Memory;
pub mod PPU;
pub mod InterruptFlags;
pub mod Joypad;

use std::env::args;
use std::fs::File;
use std::io::Read;

fn load_rom(filename: &str) -> Vec<u8>
    {
        let mut file = File::open(filename).expect("FAILED TO OPEN BOOT ROM");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("ERROR READING BOOT ROM");
        buffer
    }
fn main() 
    {
        let args: Vec<String> = args().collect();
        if args.len() < 2 
        {
            eprintln!("Usage: {} <boot_rom_file>", args[0]);
            std::process::exit(1);
        }
        let game_rom_filename = &args[1];
        let boot_rom = load_rom("boot.bin");
        let game_rom = load_rom(&game_rom_filename);
        let _cpu = CPU::CPU::new(boot_rom, game_rom);
    }
