//Defines register structure
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

//Implements functionality for the registers structure
impl Registers {
    pub fn set_bc(&mut self, value: u16)
    {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_bc(&mut self) -> u16
    {
        return ((self.b as u16) << 8 | self.c as u16);
    }

    pub fn set_af(&mut self, value: u16)
    {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    pub fn get_af(&mut self) -> u16
    {
        let f_by: u8 = (&self.f).into();
        return ((self.a as u16) << 8 | f_by as u16);
    }

    pub fn set_de(&mut self, value: u16)
    {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_de(&mut self) -> u16
    {
        return ((self.d as u16) << 8 | self.e as u16);
    }

    pub fn set_hl(&mut self, value: u16)
    {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn get_hl(&mut self) -> u16
    {
        return ((self.h as u16) << 8 | self.l as u16);
    }
} 
//provides a default for register values
impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::from(0),
            h: 0,
            l: 0,
        }
    }
}
//declares constants for byte positions of flags
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

//Defines structure for the flag register
pub struct FlagsRegister
    {
        zero: bool,
        subtract: bool,
        half_carry: bool,
        carry: bool,
    }
//implements standard conversion for flagregister to u8 and vice versa
impl std::convert::From<FlagsRegister> for u8  
{
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister 
{
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
//implements conversion from flagregister reference to u8
impl std::convert::From<&FlagsRegister> for u8 {
    fn from(flag: &FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

struct CPU
{
    registers: Registers,
    pc: u16,
    bus: MemoryBus
}

struct MemoryBus
{
    memory: [u8; 0xFFFF]
}

impl MemoryBus
{
    fn read_byte(&mut self, address: u16) -> u8
    {
        self.memory[address as usize]
    }

    fn read_word(&mut self, address: u16) -> u16
    {
        let lo = self.read_byte(address) as u16;
        let hi = (self.read_byte(address + 1) << 8) as u16;
        let word = lo | hi;
        word
    }

    fn write_byte(&mut self, address: u16, value: u8)
    {
        self.memory[address as usize] = value;
    }

    fn write_word(&mut self, address: u16, value: u16)
    {
        let lo = (value & 0x00FF) as u8;
        let hi = ((value & 0xFF00) >> 8) as u8;
        self.memory[address as usize] = lo;
        self.memory[(address + 1) as usize] = hi;
    }
}

enum Instruction
{
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget16),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC8(ArithmeticTarget),
    INC16(ArithmeticTarget16),
    DEC8(ArithmeticTarget),
    DEC16(ArithmeticTarget16),
    CCF(),
    SCF(),
    RRA(),
    RLA(),
    RRCA(),
    RLCA(),
    CPL(),
    BIT(ArithmeticTarget, u8),
    RES(ArithmeticTarget, u8),
    SET(ArithmeticTarget, u8),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    DAA(),
    JP(JumpTest),
    LD(LoadType)
}
enum ArithmeticTarget
{
    A, B, C, D, E, H, L, HL, U8
}
enum ArithmeticTarget16
{
    HL, BC, DE, AF
}
enum LoadByteTarget 
{
    A, B, C, D, E, H, L, HLI
}
enum LoadByteSource 
{
    A, B, C, D, E, H, L, D8, HLI
}
enum LoadByteIndirect
{
    BC, DE, HLP, HLN
}
enum LoadType 
{
  Byte(LoadByteTarget, LoadByteSource),
  Word(LoadWordTarget, LoadWordSource),
  AFromIndirect(LoadByteIndirect),
  IndirectFromA(LoadByteIndirect),
  AFromByteAddress(LoadByteAddress),
  ByteAddressFromA(LoadByteAddress)
}
enum LoadWordTarget
{
    AF, HL, DE, BC, HLI
}
enum LoadWordSource
{
    AF, BC, DE, HL, D16, HLI
}
enum LoadByteAddress
{
    U8, C
}
enum JumpTest
{
    NotZero, Zero, NotCarry, Carry, Always
}
impl Instruction
{
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
          Instruction::from_byte_prefixed(byte)
        } else {
          Instruction::from_byte_not_prefixed(byte)
        }
      }
    
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>
        {
            match byte
                {
                    // 0x00 => Some(Instruction::()),
                    0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))),
                    0x02 => Some(Instruction::LD(LoadType::IndirectFromA(LoadByteIndirect::BC))),
                    0x03 => Some(Instruction::INC16(ArithmeticTarget16::BC)),
                    0x04 => Some(Instruction::INC8(ArithmeticTarget::B)),
                    0x05 => Some(Instruction::DEC8(ArithmeticTarget::B)),
                    0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
                    0x07 => Some(Instruction::RLCA()),
                    // 0x08 => Some(Instruction::()),
                    0x09 => Some(Instruction::ADDHL(ArithmeticTarget16::BC)),
                    0x0A => Some(Instruction::LD(LoadType::AFromIndirect(LoadByteIndirect::BC))),
                    0x0B => Some(Instruction::DEC16(ArithmeticTarget16::BC)),
                    0x0C => Some(Instruction::INC8(ArithmeticTarget::C)),
                    0x0D => Some(Instruction::DEC8(ArithmeticTarget::C)),
                    0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
                    0x0F => Some(Instruction::RRCA()),
                    // 0x10 => Some(Instruction::()),
                    0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))),
                    0x12 => Some(Instruction::LD(LoadType::IndirectFromA(LoadByteIndirect::DE))),
                    0x13 => Some(Instruction::INC16(ArithmeticTarget16::DE)),
                    0x14 => Some(Instruction::INC8(ArithmeticTarget::D)),
                    0x15 => Some(Instruction::DEC8(ArithmeticTarget::D)),
                    0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
                    0x17 => Some(Instruction::RLA()),
                    //0x18 => Some(Instruction::()),
                    0x19 => Some(Instruction::ADDHL(ArithmeticTarget16::DE)),
                    0x1A => Some(Instruction::LD(LoadType::AFromIndirect(LoadByteIndirect::DE))),
                    0x1B => Some(Instruction::DEC16(ArithmeticTarget16::DE)),
                    0x1C => Some(Instruction::INC8(ArithmeticTarget::E)),
                    0x1D => Some(Instruction::DEC8(ArithmeticTarget::E)),
                    0x1E => Some(Instruction::()),
                    0x1F => Some(Instruction::RRA()),
                    // 0x20 => Some(Instruction::()),
                    // 0x21 => Some(Instruction::()),
                    // 0x22 => Some(Instruction::()),
                    0x23 => Some(Instruction::INC16(ArithmeticTarget16::HL)),
                    0x24 => Some(Instruction::INC8(ArithmeticTarget::H)),
                    0x25 => Some(Instruction::DEC8(ArithmeticTarget::H)),
                    // 0x26 => Some(Instruction::()),
                    0x27 => Some(Instruction::DAA()),
                    // 0x28 => Some(Instruction::()),
                    0x29 => Some(Instruction::ADDHL(ArithmeticTarget16::HL)),
                    // 0x2A => Some(Instruction::()),
                    0x2B => Some(Instruction::DEC16(ArithmeticTarget16::HL)),
                    0x2C => Some(Instruction::INC8(ArithmeticTarget::L)),
                    0x2D => Some(Instruction::DEC8(ArithmeticTarget::L)),
                    // 0x2E => Some(Instruction::()),
                    0x2F => Some(Instruction::CPL()),
                    // 0x30 => Some(Instruction::()),
                    // 0x31 => Some(Instruction::()),
                    // 0x32 => Some(Instruction::()),
                    // 0x33 => Some(Instruction::()),
                    0x34 => Some(Instruction::INC8(ArithmeticTarget::HL)),
                    0x35 => Some(Instruction::INC8(ArithmeticTarget::HL)),
                    // 0x36 => Some(Instruction::()),
                    0x37 => Some(Instruction::SCF()),
                    // 0x38 => Some(Instruction::()),
                    // 0x39 => Some(Instruction::()),
                    // 0x3A => Some(Instruction::()),
                    // 0x3B => Some(Instruction::()),
                    0x3C => Some(Instruction::INC8(ArithmeticTarget::A)),
                    0x3D => Some(Instruction::DEC8(ArithmeticTarget::A)),
                    // 0x3E => Some(Instruction::()),
                    0x3F => Some(Instruction::CCF()),
                    // 0x40 => Some(Instruction::()),
                    // 0x41 => Some(Instruction::()),
                    // 0x42 => Some(Instruction::()),
                    // 0x43 => Some(Instruction::()),
                    // 0x44 => Some(Instruction::()),
                    // 0x45 => Some(Instruction::()),
                    // 0x46 => Some(Instruction::()),
                    // 0x47 => Some(Instruction::()),
                    // 0x48 => Some(Instruction::()),
                    // 0x49 => Some(Instruction::()),
                    // 0x4A => Some(Instruction::()),
                    // 0x4B => Some(Instruction::()),
                    // 0x4C => Some(Instruction::()),
                    // 0x4D => Some(Instruction::()),
                    // 0x4E => Some(Instruction::()),
                    // 0x4F => Some(Instruction::()),
                    // 0x50 => Some(Instruction::()),
                    // 0x51 => Some(Instruction::()),
                    // 0x52 => Some(Instruction::()),
                    // 0x53 => Some(Instruction::()),
                    // 0x54 => Some(Instruction::()),
                    // 0x55 => Some(Instruction::()),
                    // 0x56 => Some(Instruction::()),
                    // 0x57 => Some(Instruction::()),
                    // 0x58 => Some(Instruction::()),
                    // 0x59 => Some(Instruction::()),
                    // 0x5A => Some(Instruction::()),
                    // 0x5B => Some(Instruction::()),
                    // 0x5C => Some(Instruction::()),
                    // 0x5D => Some(Instruction::()),
                    // 0x5E => Some(Instruction::()),
                    // 0x5F => Some(Instruction::()),
                    // 0x60 => Some(Instruction::()),
                    // 0x61 => Some(Instruction::()),
                    // 0x62 => Some(Instruction::()),
                    // 0x63 => Some(Instruction::()),
                    // 0x64 => Some(Instruction::()),
                    // 0x65 => Some(Instruction::()),
                    // 0x66 => Some(Instruction::()),
                    // 0x67 => Some(Instruction::()),
                    // 0x68 => Some(Instruction::()),
                    // 0x69 => Some(Instruction::()),
                    // 0x6A => Some(Instruction::()),
                    // 0x6B => Some(Instruction::()),
                    // 0x6C => Some(Instruction::()),
                    // 0x6D => Some(Instruction::()),
                    // 0x6E => Some(Instruction::()),
                    // 0x6F => Some(Instruction::()),
                    // 0x70 => Some(Instruction::()),
                    // 0x71 => Some(Instruction::()),
                    // 0x72 => Some(Instruction::()),
                    // 0x73 => Some(Instruction::()),
                    // 0x74 => Some(Instruction::()),
                    // 0x75 => Some(Instruction::()),
                    // 0x76 => Some(Instruction::()),
                    // 0x77 => Some(Instruction::()),
                    // 0x78 => Some(Instruction::()),
                    // 0x79 => Some(Instruction::()),
                    // 0x7A => Some(Instruction::()),
                    // 0x7B => Some(Instruction::()),
                    // 0x7C => Some(Instruction::()),
                    // 0x7D => Some(Instruction::()),
                    // 0x7E => Some(Instruction::()),
                    // 0x7F => Some(Instruction::()),
                    0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
                    0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
                    0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
                    0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
                    0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
                    0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
                    0x86 => Some(Instruction::ADD(ArithmeticTarget::HL)),
                    0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
                    0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
                    0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
                    0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
                    0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
                    0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
                    0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
                    0x8E => Some(Instruction::ADC(ArithmeticTarget::HL)),
                    0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),
                    0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
                    0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
                    0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
                    0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
                    0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
                    0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
                    0x96 => Some(Instruction::SUB(ArithmeticTarget::HL)),
                    0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
                    0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
                    0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
                    0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
                    0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
                    0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
                    0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
                    0x9E => Some(Instruction::SBC(ArithmeticTarget::HL)),
                    0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),
                    0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
                    0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
                    0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
                    0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
                    0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
                    0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
                    0xA6 => Some(Instruction::AND(ArithmeticTarget::HL)),
                    0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
                    0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
                    0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
                    0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
                    0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
                    0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
                    0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
                    0xAE => Some(Instruction::XOR(ArithmeticTarget::HL)),
                    0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
                    0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
                    0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
                    0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
                    0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
                    0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
                    0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
                    0xB6 => Some(Instruction::OR(ArithmeticTarget::HL)),
                    0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
                    0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
                    0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
                    0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
                    0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
                    0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
                    0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
                    0xBE => Some(Instruction::CP(ArithmeticTarget::HL)),
                    0xBF => Some(Instruction::CP(ArithmeticTarget::A)),
                    // 0xC0 => Some(Instruction::()),
                    // 0xC1 => Some(Instruction::()),
                    // 0xC2 => Some(Instruction::()),
                    // 0xC3 => Some(Instruction::()),
                    // 0xC4 => Some(Instruction::()),
                    // 0xC5 => Some(Instruction::()),
                    0xC6 => Some(Instruction::ADD(ArithmeticTarget::U8)),
                    // 0xC7 => Some(Instruction::()),
                    // 0xC8 => Some(Instruction::()),
                    // 0xC9 => Some(Instruction::()),
                    // 0xCA => Some(Instruction::()),
                    // 0xCB => Some(Instruction::()),
                    // 0xCC => Some(Instruction::()),
                    // 0xCD => Some(Instruction::()),
                    0xCE => Some(Instruction::ADC(ArithmeticTarget::U8)),
                    // 0xCF => Some(Instruction::()),
                    // 0xD0 => Some(Instruction::()),
                    // 0xD1 => Some(Instruction::()),
                    // 0xD2 => Some(Instruction::()),
                    // 0xD3 => Some(Instruction::()),
                    // 0xD4 => Some(Instruction::()),
                    // 0xD5 => Some(Instruction::()),
                    0xD6 => Some(Instruction::SUB(ArithmeticTarget::U8)),
                    // 0xD7 => Some(Instruction::()),
                    // 0xD8 => Some(Instruction::()),
                    // 0xD9 => Some(Instruction::()),
                    // 0xDA => Some(Instruction::()),
                    // 0xDB => Some(Instruction::()),
                    // 0xDC => Some(Instruction::()),
                    // 0xDD => Some(Instruction::()),
                    0xDE => Some(Instruction::SBC(ArithmeticTarget::U8)),
                    // 0xDF => Some(Instruction::()),
                    // 0xE0 => Some(Instruction::()),
                    // 0xE1 => Some(Instruction::()),
                    // 0xE2 => Some(Instruction::()),
                    // 0xE3 => Some(Instruction::()),
                    // 0xE4 => Some(Instruction::()),
                    // 0xE5 => Some(Instruction::()),
                    0xE6 => Some(Instruction::AND(ArithmeticTarget::U8)),
                    // 0xE7 => Some(Instruction::()),
                    // 0xE8 => Some(Instruction::()),
                    // 0xE9 => Some(Instruction::()),
                    // 0xEA => Some(Instruction::()),
                    // 0xEB => Some(Instruction::()),
                    // 0xEC => Some(Instruction::()),
                    // 0xED => Some(Instruction::()),
                    0xEE => Some(Instruction::XOR(ArithmeticTarget::U8)),
                    // 0xEF => Some(Instruction::()),
                    // 0xF0 => Some(Instruction::()),
                    // 0xF1 => Some(Instruction::()),
                    // 0xF2 => Some(Instruction::()),
                    // 0xF3 => Some(Instruction::()),
                    // 0xF4 => Some(Instruction::()),
                    // 0xF5 => Some(Instruction::()),
                    0xF6 => Some(Instruction::OR(ArithmeticTarget::U8)),
                    // 0xF7 => Some(Instruction::()),
                    // 0xF8 => Some(Instruction::()),
                    // 0xF9 => Some(Instruction::()),
                    // 0xFA => Some(Instruction::()),
                    // 0xFB => Some(Instruction::()),
                    // 0xFC => Some(Instruction::()),
                    // 0xFD => Some(Instruction::()),
                    0xFE => Some(Instruction::CP(ArithmeticTarget::U8)),
                    // 0xFF => Some(Instruction::())
                }
        }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction>
        {
            match byte
                {
                    0x00 => Some(Instruction::RLC(ArithmeticTarget::B)),
                    0x01 => Some(Instruction::RLC(ArithmeticTarget::C)),
                    0x02 => Some(Instruction::RLC(ArithmeticTarget::D)),
                    0x03 => Some(Instruction::RLC(ArithmeticTarget::E)),
                    0x04 => Some(Instruction::RLC(ArithmeticTarget::H)),
                    0x05 => Some(Instruction::RLC(ArithmeticTarget::L)),
                    0x06 => Some(Instruction::RLC(ArithmeticTarget::HL)),
                    0x07 => Some(Instruction::RLC(ArithmeticTarget::A)),
                    0x08 => Some(Instruction::RRC(ArithmeticTarget::B)),
                    0x09 => Some(Instruction::RRC(ArithmeticTarget::C)),
                    0x0A => Some(Instruction::RRC(ArithmeticTarget::D)),
                    0x0B => Some(Instruction::RRC(ArithmeticTarget::E)),
                    0x0C => Some(Instruction::RRC(ArithmeticTarget::H)),
                    0x0D => Some(Instruction::RRC(ArithmeticTarget::L)),
                    0x0E => Some(Instruction::RRC(ArithmeticTarget::HL)),
                    0x0F => Some(Instruction::RRC(ArithmeticTarget::A)),
                    0x10 => Some(Instruction::RL(ArithmeticTarget::B)),
                    0x11 => Some(Instruction::RL(ArithmeticTarget::C)),
                    0x12 => Some(Instruction::RL(ArithmeticTarget::D)),
                    0x13 => Some(Instruction::RL(ArithmeticTarget::E)),
                    0x14 => Some(Instruction::RL(ArithmeticTarget::H)),
                    0x15 => Some(Instruction::RL(ArithmeticTarget::L)),
                    0x16 => Some(Instruction::RL(ArithmeticTarget::HL)),
                    0x17 => Some(Instruction::RL(ArithmeticTarget::A)),
                    0x18 => Some(Instruction::RR(ArithmeticTarget::B)),
                    0x19 => Some(Instruction::RR(ArithmeticTarget::C)),
                    0x1A => Some(Instruction::RR(ArithmeticTarget::D)),
                    0x1B => Some(Instruction::RR(ArithmeticTarget::E)),
                    0x1C => Some(Instruction::RR(ArithmeticTarget::H)),
                    0x1D => Some(Instruction::RR(ArithmeticTarget::L)),
                    0x1E => Some(Instruction::RR(ArithmeticTarget::HL)),
                    0x1F => Some(Instruction::RR(ArithmeticTarget::A)),
                    0x20 => Some(Instruction::SLA(ArithmeticTarget::B)),
                    0x21 => Some(Instruction::SLA(ArithmeticTarget::C)),
                    0x22 => Some(Instruction::SLA(ArithmeticTarget::D)),
                    0x23 => Some(Instruction::SLA(ArithmeticTarget::E)),
                    0x24 => Some(Instruction::SLA(ArithmeticTarget::H)),
                    0x25 => Some(Instruction::SLA(ArithmeticTarget::L)),
                    0x26 => Some(Instruction::SLA(ArithmeticTarget::HL)),
                    0x27 => Some(Instruction::SLA(ArithmeticTarget::A)),
                    0x28 => Some(Instruction::SRA(ArithmeticTarget::B)),
                    0x29 => Some(Instruction::SRA(ArithmeticTarget::C)),
                    0x2A => Some(Instruction::SRA(ArithmeticTarget::D)),
                    0x2B => Some(Instruction::SRA(ArithmeticTarget::E)),
                    0x2C => Some(Instruction::SRA(ArithmeticTarget::H)),
                    0x2D => Some(Instruction::SRA(ArithmeticTarget::L)),
                    0x2E => Some(Instruction::SRA(ArithmeticTarget::HL)),
                    0x2F => Some(Instruction::SRA(ArithmeticTarget::A)),
                    0x30 => Some(Instruction::SWAP(ArithmeticTarget::B)),
                    0x31 => Some(Instruction::SWAP(ArithmeticTarget::C)),
                    0x32 => Some(Instruction::SWAP(ArithmeticTarget::D)),
                    0x33 => Some(Instruction::SWAP(ArithmeticTarget::E)),
                    0x34 => Some(Instruction::SWAP(ArithmeticTarget::H)),
                    0x35 => Some(Instruction::SWAP(ArithmeticTarget::L)),
                    0x36 => Some(Instruction::SWAP(ArithmeticTarget::HL)),
                    0x37 => Some(Instruction::SWAP(ArithmeticTarget::A)),
                    0x38 => Some(Instruction::SRL(ArithmeticTarget::B)),
                    0x39 => Some(Instruction::SRL(ArithmeticTarget::C)),
                    0x3A => Some(Instruction::SRL(ArithmeticTarget::D)),
                    0x3B => Some(Instruction::SRL(ArithmeticTarget::E)),
                    0x3C => Some(Instruction::SRL(ArithmeticTarget::H)),
                    0x3D => Some(Instruction::SRL(ArithmeticTarget::L)),
                    0x3E => Some(Instruction::SRL(ArithmeticTarget::HL)),
                    0x3F => Some(Instruction::SRL(ArithmeticTarget::A)),
                    0x40 => Some(Instruction::BIT(ArithmeticTarget::B, 0)),
                    0x41 => Some(Instruction::BIT(ArithmeticTarget::C, 0)),
                    0x42 => Some(Instruction::BIT(ArithmeticTarget::D, 0)),
                    0x43 => Some(Instruction::BIT(ArithmeticTarget::E, 0)),
                    0x44 => Some(Instruction::BIT(ArithmeticTarget::H, 0)),
                    0x45 => Some(Instruction::BIT(ArithmeticTarget::L, 0)),
                    0x46 => Some(Instruction::BIT(ArithmeticTarget::HL, 0)),
                    0x47 => Some(Instruction::BIT(ArithmeticTarget::A, 0)),
                    0x48 => Some(Instruction::BIT(ArithmeticTarget::B, 1)),
                    0x49 => Some(Instruction::BIT(ArithmeticTarget::C, 1)),
                    0x4A => Some(Instruction::BIT(ArithmeticTarget::D, 1)),
                    0x4B => Some(Instruction::BIT(ArithmeticTarget::E, 1)),
                    0x4C => Some(Instruction::BIT(ArithmeticTarget::H, 1)),
                    0x4D => Some(Instruction::BIT(ArithmeticTarget::L, 1)),
                    0x4E => Some(Instruction::BIT(ArithmeticTarget::HL, 1)),
                    0x4F => Some(Instruction::BIT(ArithmeticTarget::A, 1)),
                    0x50 => Some(Instruction::BIT(ArithmeticTarget::B, 2)),
                    0x51 => Some(Instruction::BIT(ArithmeticTarget::C, 2)),
                    0x52 => Some(Instruction::BIT(ArithmeticTarget::D, 2)),
                    0x53 => Some(Instruction::BIT(ArithmeticTarget::E, 2)),
                    0x54 => Some(Instruction::BIT(ArithmeticTarget::H, 2)),
                    0x55 => Some(Instruction::BIT(ArithmeticTarget::L, 2)),
                    0x56 => Some(Instruction::BIT(ArithmeticTarget::HL, 2)),
                    0x57 => Some(Instruction::BIT(ArithmeticTarget::A, 2)),
                    0x58 => Some(Instruction::BIT(ArithmeticTarget::B, 3)),
                    0x59 => Some(Instruction::BIT(ArithmeticTarget::C, 3)),
                    0x5A => Some(Instruction::BIT(ArithmeticTarget::D, 3)),
                    0x5B => Some(Instruction::BIT(ArithmeticTarget::E, 3)),
                    0x5C => Some(Instruction::BIT(ArithmeticTarget::H, 3)),
                    0x5D => Some(Instruction::BIT(ArithmeticTarget::L, 3)),
                    0x5E => Some(Instruction::BIT(ArithmeticTarget::HL, 3)),
                    0x5F => Some(Instruction::BIT(ArithmeticTarget::A, 3)),
                    0x60 => Some(Instruction::BIT(ArithmeticTarget::B, 4)),
                    0x61 => Some(Instruction::BIT(ArithmeticTarget::C, 4)),
                    0x62 => Some(Instruction::BIT(ArithmeticTarget::D, 4)),
                    0x63 => Some(Instruction::BIT(ArithmeticTarget::E, 4)),
                    0x64 => Some(Instruction::BIT(ArithmeticTarget::H, 4)),
                    0x65 => Some(Instruction::BIT(ArithmeticTarget::L, 4)),
                    0x66 => Some(Instruction::BIT(ArithmeticTarget::HL, 4)),
                    0x67 => Some(Instruction::BIT(ArithmeticTarget::A, 4)),
                    0x68 => Some(Instruction::BIT(ArithmeticTarget::B, 5)),
                    0x69 => Some(Instruction::BIT(ArithmeticTarget::C, 5)),
                    0x6A => Some(Instruction::BIT(ArithmeticTarget::D, 5)),
                    0x6B => Some(Instruction::BIT(ArithmeticTarget::E, 5)),
                    0x6C => Some(Instruction::BIT(ArithmeticTarget::H, 5)),
                    0x6D => Some(Instruction::BIT(ArithmeticTarget::L, 5)),
                    0x6E => Some(Instruction::BIT(ArithmeticTarget::HL, 5)),
                    0x6F => Some(Instruction::BIT(ArithmeticTarget::A, 5)),
                    0x70 => Some(Instruction::BIT(ArithmeticTarget::B, 6)),
                    0x71 => Some(Instruction::BIT(ArithmeticTarget::C, 6)),
                    0x72 => Some(Instruction::BIT(ArithmeticTarget::D, 6)),
                    0x73 => Some(Instruction::BIT(ArithmeticTarget::E, 6)),
                    0x74 => Some(Instruction::BIT(ArithmeticTarget::H, 6)),
                    0x75 => Some(Instruction::BIT(ArithmeticTarget::L, 6)),
                    0x76 => Some(Instruction::BIT(ArithmeticTarget::HL, 6)),
                    0x77 => Some(Instruction::BIT(ArithmeticTarget::A, 6)),
                    0x78 => Some(Instruction::BIT(ArithmeticTarget::B, 7)),
                    0x79 => Some(Instruction::BIT(ArithmeticTarget::C, 7)),
                    0x7A => Some(Instruction::BIT(ArithmeticTarget::D, 7)),
                    0x7B => Some(Instruction::BIT(ArithmeticTarget::E, 7)),
                    0x7C => Some(Instruction::BIT(ArithmeticTarget::H, 7)),
                    0x7D => Some(Instruction::BIT(ArithmeticTarget::L, 7)),
                    0x7E => Some(Instruction::BIT(ArithmeticTarget::HL, 7)),
                    0x7F => Some(Instruction::BIT(ArithmeticTarget::A, 7)),
                    0x80 => Some(Instruction::RES(ArithmeticTarget::B, 0)), 
                    0x81 => Some(Instruction::RES(ArithmeticTarget::C, 0)),
                    0x82 => Some(Instruction::RES(ArithmeticTarget::D, 0)),
                    0x83 => Some(Instruction::RES(ArithmeticTarget::E, 0)),
                    0x84 => Some(Instruction::RES(ArithmeticTarget::H, 0)),
                    0x85 => Some(Instruction::RES(ArithmeticTarget::L, 0)),
                    0x86 => Some(Instruction::RES(ArithmeticTarget::HL, 0)),
                    0x87 => Some(Instruction::RES(ArithmeticTarget::A, 0)),
                    0x88 => Some(Instruction::RES(ArithmeticTarget::B, 1)),
                    0x89 => Some(Instruction::RES(ArithmeticTarget::C, 1)),
                    0x8A => Some(Instruction::RES(ArithmeticTarget::D, 1)),
                    0x8B => Some(Instruction::RES(ArithmeticTarget::E, 1)),
                    0x8C => Some(Instruction::RES(ArithmeticTarget::H, 1)),
                    0x8D => Some(Instruction::RES(ArithmeticTarget::L, 1)),
                    0x8E => Some(Instruction::RES(ArithmeticTarget::HL, 1)),
                    0x8F => Some(Instruction::RES(ArithmeticTarget::A, 1)),
                    0x90 => Some(Instruction::RES(ArithmeticTarget::B, 2)),
                    0x91 => Some(Instruction::RES(ArithmeticTarget::C, 2)),
                    0x92 => Some(Instruction::RES(ArithmeticTarget::D, 2)),
                    0x93 => Some(Instruction::RES(ArithmeticTarget::E, 2)),
                    0x94 => Some(Instruction::RES(ArithmeticTarget::H, 2)),
                    0x95 => Some(Instruction::RES(ArithmeticTarget::L, 2)),
                    0x96 => Some(Instruction::RES(ArithmeticTarget::HL, 2)),
                    0x97 => Some(Instruction::RES(ArithmeticTarget::A, 2)),
                    0x98 => Some(Instruction::RES(ArithmeticTarget::B, 3)),
                    0x99 => Some(Instruction::RES(ArithmeticTarget::C, 3)),
                    0x9A => Some(Instruction::RES(ArithmeticTarget::D, 3)),
                    0x9B => Some(Instruction::RES(ArithmeticTarget::E, 3)),
                    0x9C => Some(Instruction::RES(ArithmeticTarget::H, 3)),
                    0x9D => Some(Instruction::RES(ArithmeticTarget::L, 3)),
                    0x9E => Some(Instruction::RES(ArithmeticTarget::HL, 3)),
                    0x9F => Some(Instruction::RES(ArithmeticTarget::A, 3)),
                    0xA0 => Some(Instruction::RES(ArithmeticTarget::B, 4)),
                    0xA1 => Some(Instruction::RES(ArithmeticTarget::C, 4)),
                    0xA2 => Some(Instruction::RES(ArithmeticTarget::D, 4)),
                    0xA3 => Some(Instruction::RES(ArithmeticTarget::E, 4)),
                    0xA4 => Some(Instruction::RES(ArithmeticTarget::H, 4)),
                    0xA5 => Some(Instruction::RES(ArithmeticTarget::L, 4)),
                    0xA6 => Some(Instruction::RES(ArithmeticTarget::HL, 4)),
                    0xA7 => Some(Instruction::RES(ArithmeticTarget::A, 4)),
                    0xA8 => Some(Instruction::RES(ArithmeticTarget::B, 5)),
                    0xA9 => Some(Instruction::RES(ArithmeticTarget::C, 5)),
                    0xAA => Some(Instruction::RES(ArithmeticTarget::D, 5)),
                    0xAB => Some(Instruction::RES(ArithmeticTarget::E, 5)),
                    0xAC => Some(Instruction::RES(ArithmeticTarget::H, 5)),
                    0xAD => Some(Instruction::RES(ArithmeticTarget::L, 5)),
                    0xAE => Some(Instruction::RES(ArithmeticTarget::HL, 5)),
                    0xAF => Some(Instruction::RES(ArithmeticTarget::A, 5)),
                    0xB0 => Some(Instruction::RES(ArithmeticTarget::B, 6)),
                    0xB1 => Some(Instruction::RES(ArithmeticTarget::C, 6)),
                    0xB2 => Some(Instruction::RES(ArithmeticTarget::D, 6)),
                    0xB3 => Some(Instruction::RES(ArithmeticTarget::E, 6)),
                    0xB4 => Some(Instruction::RES(ArithmeticTarget::H, 6)),
                    0xB5 => Some(Instruction::RES(ArithmeticTarget::L, 6)),
                    0xB6 => Some(Instruction::RES(ArithmeticTarget::HL, 6)),
                    0xB7 => Some(Instruction::RES(ArithmeticTarget::A, 6)),
                    0xB8 => Some(Instruction::RES(ArithmeticTarget::B, 7)),
                    0xB9 => Some(Instruction::RES(ArithmeticTarget::C, 7)),
                    0xBA => Some(Instruction::RES(ArithmeticTarget::D, 7)),
                    0xBB => Some(Instruction::RES(ArithmeticTarget::E, 7)),
                    0xBC => Some(Instruction::RES(ArithmeticTarget::H, 7)),
                    0xBD => Some(Instruction::RES(ArithmeticTarget::L, 7)),
                    0xBE => Some(Instruction::RES(ArithmeticTarget::HL, 7)),
                    0xBF => Some(Instruction::RES(ArithmeticTarget::A, 7)),
                    0xC0 => Some(Instruction::SET(ArithmeticTarget::B, 0)), 
                    0xC1 => Some(Instruction::SET(ArithmeticTarget::C, 0)),
                    0xC2 => Some(Instruction::SET(ArithmeticTarget::D, 0)),
                    0xC3 => Some(Instruction::SET(ArithmeticTarget::E, 0)),
                    0xC4 => Some(Instruction::SET(ArithmeticTarget::H, 0)),
                    0xC5 => Some(Instruction::SET(ArithmeticTarget::L, 0)),
                    0xC6 => Some(Instruction::SET(ArithmeticTarget::HL, 0)),
                    0xC7 => Some(Instruction::SET(ArithmeticTarget::A, 0)),
                    0xC8 => Some(Instruction::SET(ArithmeticTarget::B, 1)),
                    0xC9 => Some(Instruction::SET(ArithmeticTarget::C, 1)),
                    0xCA => Some(Instruction::SET(ArithmeticTarget::D, 1)),
                    0xCB => Some(Instruction::SET(ArithmeticTarget::E, 1)),
                    0xCC => Some(Instruction::SET(ArithmeticTarget::H, 1)),
                    0xCD => Some(Instruction::SET(ArithmeticTarget::L, 1)),
                    0xCE => Some(Instruction::SET(ArithmeticTarget::HL, 1)),
                    0xCF => Some(Instruction::SET(ArithmeticTarget::A, 1)),
                    0xD0 => Some(Instruction::SET(ArithmeticTarget::B, 2)),
                    0xD1 => Some(Instruction::SET(ArithmeticTarget::C, 2)),
                    0xD2 => Some(Instruction::SET(ArithmeticTarget::D, 2)),
                    0xD3 => Some(Instruction::SET(ArithmeticTarget::E, 2)),
                    0xD4 => Some(Instruction::SET(ArithmeticTarget::H, 2)),
                    0xD5 => Some(Instruction::SET(ArithmeticTarget::L, 2)),
                    0xD6 => Some(Instruction::SET(ArithmeticTarget::HL, 2)),
                    0xD7 => Some(Instruction::SET(ArithmeticTarget::A, 2)),
                    0xD8 => Some(Instruction::SET(ArithmeticTarget::B, 3)),
                    0xD9 => Some(Instruction::SET(ArithmeticTarget::C, 3)),
                    0xDA => Some(Instruction::SET(ArithmeticTarget::D, 3)),
                    0xDB => Some(Instruction::SET(ArithmeticTarget::E, 3)),
                    0xDC => Some(Instruction::SET(ArithmeticTarget::H, 3)),
                    0xDD => Some(Instruction::SET(ArithmeticTarget::L, 3)),
                    0xDE => Some(Instruction::SET(ArithmeticTarget::HL, 3)),
                    0xDF => Some(Instruction::SET(ArithmeticTarget::A, 3)),
                    0xE0 => Some(Instruction::SET(ArithmeticTarget::B, 4)),
                    0xE1 => Some(Instruction::SET(ArithmeticTarget::C, 4)),
                    0xE2 => Some(Instruction::SET(ArithmeticTarget::D, 4)),
                    0xE3 => Some(Instruction::SET(ArithmeticTarget::E, 4)),
                    0xE4 => Some(Instruction::SET(ArithmeticTarget::H, 4)),
                    0xE5 => Some(Instruction::SET(ArithmeticTarget::L, 4)),
                    0xE6 => Some(Instruction::SET(ArithmeticTarget::HL, 4)),
                    0xE7 => Some(Instruction::SET(ArithmeticTarget::A, 4)),
                    0xE8 => Some(Instruction::SET(ArithmeticTarget::B, 5)),
                    0xE9 => Some(Instruction::SET(ArithmeticTarget::C, 5)),
                    0xEA => Some(Instruction::SET(ArithmeticTarget::D, 5)),
                    0xEB => Some(Instruction::SET(ArithmeticTarget::E, 5)),
                    0xEC => Some(Instruction::SET(ArithmeticTarget::H, 5)),
                    0xED => Some(Instruction::SET(ArithmeticTarget::L, 5)),
                    0xEE => Some(Instruction::SET(ArithmeticTarget::HL, 5)),
                    0xEF => Some(Instruction::SET(ArithmeticTarget::A, 5)),
                    0xF0 => Some(Instruction::SET(ArithmeticTarget::B, 6)),
                    0xF1 => Some(Instruction::SET(ArithmeticTarget::C, 6)),
                    0xF2 => Some(Instruction::SET(ArithmeticTarget::D, 6)),
                    0xF3 => Some(Instruction::SET(ArithmeticTarget::E, 6)),
                    0xF4 => Some(Instruction::SET(ArithmeticTarget::H, 6)),
                    0xF5 => Some(Instruction::SET(ArithmeticTarget::L, 6)),
                    0xF6 => Some(Instruction::SET(ArithmeticTarget::HL, 6)),
                    0xF7 => Some(Instruction::SET(ArithmeticTarget::A, 6)),
                    0xF8 => Some(Instruction::SET(ArithmeticTarget::B, 7)),
                    0xF9 => Some(Instruction::SET(ArithmeticTarget::C, 7)),
                    0xFA => Some(Instruction::SET(ArithmeticTarget::D, 7)),
                    0xFB => Some(Instruction::SET(ArithmeticTarget::E, 7)),
                    0xFC => Some(Instruction::SET(ArithmeticTarget::H, 7)),
                    0xFD => Some(Instruction::SET(ArithmeticTarget::L, 7)),
                    0xFE => Some(Instruction::SET(ArithmeticTarget::HL, 7)),
                    0xFF => Some(Instruction::SET(ArithmeticTarget::A, 7)),
                }
        }
}

impl CPU
{
    fn read_next_byte(&mut self) -> u8
        {
            self.bus.memory[(self.pc + 1) as usize]
        }
    fn read_next_word(&mut self) -> u16
        {
            let lo = self.bus.read_byte(self.pc + 1) as u16;
            let hi = (self.bus.read_byte(self.pc + 2) << 8) as u16;
            let word = lo | hi;
            word
        }
    fn step(&mut self)
        {
            let mut instruction_byte = self.bus.read_byte(self.pc);
            let prefixed = instruction_byte == 0xCB;
            if prefixed {
              instruction_byte = self.bus.read_byte(self.pc + 1);
            }
        
            let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
              self.execute(instruction)
            } else {
              let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
              panic!("Unkown instruction found for: {}", description)
            };
        
            self.pc = next_pc;
          }

    fn execute(&mut self, instruction: Instruction) -> u16
        {
            match instruction
            {
                Instruction::ADD(target) => 
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.add(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.registers.a = self.add(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.registers.a = self.add(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.registers.a = self.add(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.registers.a = self.add(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.registers.a = self.add(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.registers.a = self.add(self.registers.l); self.pc.wrapping_add(1)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.registers.a = self.add(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.registers.a = self.add(byte); self.pc.wrapping_add(2)}
                    }
                }
                Instruction::ADDHL(target) =>
                {
                    match target
                    {
                        ArithmeticTarget16::AF => {let af = self.registers.get_af(); let result = self.addhl(af); self.registers.set_hl(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::BC => {let bc = self.registers.get_bc(); let result = self.addhl(bc); self.registers.set_hl(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::DE => {let de = self.registers.get_de(); let result = self.addhl(de); self.registers.set_hl(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::HL => {let hl = self.registers.get_hl(); let result = self.addhl(hl); self.registers.set_hl(result); self.pc.wrapping_add(1)}
                    }
                }
                Instruction::ADC(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.adc(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.registers.a = self.adc(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.registers.a = self.adc(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.registers.a = self.adc(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.registers.a = self.adc(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.registers.a = self.adc(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.registers.a = self.adc(self.registers.l); self.pc.wrapping_add(1)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.registers.a = self.adc(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.registers.a = self.adc(byte); self.pc.wrapping_add(2)}      
                    }
                }
                Instruction::SUB(target) => 
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sub(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.registers.a = self.sub(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.registers.a = self.sub(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.registers.a = self.sub(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.registers.a = self.sub(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.registers.a = self.sub(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.registers.a = self.sub(self.registers.l); self.pc.wrapping_add(1)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.registers.a = self.sub(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.registers.a = self.sub(byte); self.pc.wrapping_add(2)}
                    }
                }
                Instruction::SBC(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sbc(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.registers.a = self.sbc(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.registers.a = self.sbc(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.registers.a = self.sbc(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.registers.a = self.sbc(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.registers.a = self.sbc(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.registers.a = self.sbc(self.registers.l); self.pc.wrapping_add(1)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.registers.a = self.sbc(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.registers.a = self.sbc(byte); self.pc.wrapping_add(2)}    
                    }
                }
                Instruction::AND(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.and(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.and(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.and(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.and(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.and(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.and(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.and(self.registers.l); self.pc.wrapping_add(1)}  
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.and(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.and(byte); self.pc.wrapping_add(2)}    
                    }
                } 
                Instruction::OR(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.or(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.or(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.or(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.or(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.or(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.or(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.or(self.registers.l); self.pc.wrapping_add(1)}  
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.or(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.or(byte); self.pc.wrapping_add(2)}    
                    }
                } 
                Instruction::XOR(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.xor(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.xor(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.xor(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.xor(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.xor(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.xor(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.xor(self.registers.l); self.pc.wrapping_add(1)}  
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.xor(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.xor(byte); self.pc.wrapping_add(2)}    
                    }
                }
                Instruction::CP(target) => 
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.cp(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.cp(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.cp(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.cp(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.cp(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.cp(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.cp(self.registers.l); self.pc.wrapping_add(1)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.cp(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.cp(byte); self.pc.wrapping_add(2)}    
                    }
                }
                Instruction::INC8(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.inc_8(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.registers.b = self.inc_8(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.registers.c = self.inc_8(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.registers.d = self.inc_8(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.registers.e = self.inc_8(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.registers.h = self.inc_8(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.registers.l = self.inc_8(self.registers.l); self.pc.wrapping_add(1)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.inc_8(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::INC16(target) =>
                {
                    match target
                    {
                        ArithmeticTarget16::AF => {let af = self.registers.get_af(); let result = self.inc_16(af); self.registers.set_af(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::BC => {let bc = self.registers.get_bc(); let result = self.inc_16(bc); self.registers.set_bc(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::DE => {let de = self.registers.get_de(); let result = self.inc_16(de); self.registers.set_de(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::HL => {let hl = self.registers.get_hl(); let result = self.inc_16(hl); self.registers.set_hl(result); self.pc.wrapping_add(1)}
                    }
                }
                Instruction::DEC8(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.dec_8(self.registers.a); self.pc.wrapping_add(1)}
                        ArithmeticTarget::B => {self.registers.b = self.dec_8(self.registers.b); self.pc.wrapping_add(1)}
                        ArithmeticTarget::C => {self.registers.c = self.dec_8(self.registers.c); self.pc.wrapping_add(1)}
                        ArithmeticTarget::D => {self.registers.d = self.dec_8(self.registers.d); self.pc.wrapping_add(1)}
                        ArithmeticTarget::E => {self.registers.e = self.dec_8(self.registers.e); self.pc.wrapping_add(1)}
                        ArithmeticTarget::H => {self.registers.h = self.dec_8(self.registers.h); self.pc.wrapping_add(1)}
                        ArithmeticTarget::L => {self.registers.l = self.dec_8(self.registers.l); self.pc.wrapping_add(1)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.dec_8(byte); self.pc.wrapping_add(1)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::DEC16(target) =>
                {
                    match target
                    {
                        ArithmeticTarget16::AF => {let af = self.registers.get_af(); let result = self.dec_16(af); self.registers.set_af(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::BC => {let bc = self.registers.get_bc(); let result = self.dec_16(bc); self.registers.set_bc(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::DE => {let de = self.registers.get_de(); let result = self.dec_16(de); self.registers.set_de(result); self.pc.wrapping_add(1)}
                        ArithmeticTarget16::HL => {let hl = self.registers.get_hl(); let result = self.dec_16(hl); self.registers.set_hl(result); self.pc.wrapping_add(1)}
                    }
                }
                Instruction::CCF() =>
                {
                    self.ccf();
                    self.pc.wrapping_add(1)
                }
                Instruction::SCF() =>
                {
                    self.scf();
                    self.pc.wrapping_add(1)
                }
                Instruction::RRA() =>
                {
                    self.rra();
                    self.pc.wrapping_add(1)
                }
                Instruction::RLA() =>
                {
                    self.rla();
                    self.pc.wrapping_add(1)
                }
                Instruction::RRCA() =>
                {
                    self.rrca();
                    self.pc.wrapping_add(1)
                }
                Instruction::RLCA() =>
                {
                    self.rlca();
                    self.pc.wrapping_add(1)
                }
                Instruction::CPL() =>
                {
                    self.cpl();
                    self.pc.wrapping_add(1)
                }
                Instruction::BIT(target, bit) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.bit(bit, self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.bit(bit, self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.bit(bit, self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.bit(bit, self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.bit(bit, self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.bit(bit, self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.bit(bit, self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bit(bit, byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::RES(target, bit) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.res(bit, self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.res(bit, self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.res(bit, self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.res(bit, self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.res(bit, self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.res(bit, self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.res(bit, self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.res(bit, byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}    
                    }
                }
                Instruction::SET(target, bit) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.set(bit, self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.set(bit, self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.set(bit, self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.set(bit, self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.set(bit, self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.set(bit, self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.set(bit, self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.set(bit, byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}    
                    }
                }
                Instruction::SRL(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.srl(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.srl(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.srl(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.srl(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.srl(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.srl(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.srl(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.srl(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::RR(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rr(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.rr(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.rr(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.rr(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.rr(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.rr(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.rr(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.rr(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::RL(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rl(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.rl(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.rl(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.rl(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.rl(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.rl(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.rl(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.rl(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::RRC(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rrc(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.rrc(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.rrc(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.rrc(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.rrc(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.rrc(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.rrc(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.rrc(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::RLC(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rlc(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.rlc(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.rlc(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.rlc(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.rlc(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.rlc(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.rlc(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.rlc(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::SRA(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sra(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.sra(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.sra(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.sra(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.sra(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.sra(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.sra(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.sra(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::SLA(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sla(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.sla(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.sla(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.sla(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.sla(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.sla(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.sla(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.sla(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::SWAP(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.swap(self.registers.a); self.pc.wrapping_add(2)}
                        ArithmeticTarget::B => {self.registers.b = self.swap(self.registers.b); self.pc.wrapping_add(2)}
                        ArithmeticTarget::C => {self.registers.c = self.swap(self.registers.c); self.pc.wrapping_add(2)}
                        ArithmeticTarget::D => {self.registers.d = self.swap(self.registers.d); self.pc.wrapping_add(2)}
                        ArithmeticTarget::E => {self.registers.e = self.swap(self.registers.e); self.pc.wrapping_add(2)}
                        ArithmeticTarget::H => {self.registers.h = self.swap(self.registers.h); self.pc.wrapping_add(2)}
                        ArithmeticTarget::L => {self.registers.l = self.swap(self.registers.l); self.pc.wrapping_add(2)}
                        ArithmeticTarget::HL => {let byte = self.bus.read_byte(self.registers.get_hl()); self.bus.memory[self.registers.get_hl() as usize] = self.swap(byte); self.pc.wrapping_add(2)}
                        ArithmeticTarget::U8 => {self.pc}
                    }
                }
                Instruction::DAA() =>
                {
                    self.daa();
                    self.pc.wrapping_add(1)
                }
                Instruction::JP(test) => 
                {
                    let jump_condition = match test 
                    {
                        JumpTest::NotZero => !self.registers.f.zero,
                        JumpTest::NotCarry => !self.registers.f.carry,
                        JumpTest::Zero => self.registers.f.zero,
                        JumpTest::Carry => self.registers.f.carry,
                        JumpTest::Always => true
                    };
                    self.jump(jump_condition)
                }
                Instruction::LD(load_type) => 
                {
                    match load_type 
                    {
                        LoadType::Byte(target, source) => 
                            {
                                let source_value = match source 
                                {
                                    LoadByteSource::A => self.registers.a,
                                    LoadByteSource::B => self.registers.b,
                                    LoadByteSource::C => self.registers.c,
                                    LoadByteSource::D => self.registers.d,
                                    LoadByteSource::E => self.registers.e,
                                    LoadByteSource::H => self.registers.h,
                                    LoadByteSource::L => self.registers.l,
                                    LoadByteSource::D8 => self.read_next_byte(),
                                    LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl())
                                };
                                match target 
                                {
                                    LoadByteTarget::A => self.registers.a = source_value,
                                    LoadByteTarget::B => self.registers.b = source_value,
                                    LoadByteTarget::C => self.registers.c = source_value,
                                    LoadByteTarget::D => self.registers.d = source_value,
                                    LoadByteTarget::E => self.registers.e = source_value,
                                    LoadByteTarget::H => self.registers.h = source_value,
                                    LoadByteTarget::L => self.registers.l = source_value,
                                    LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                                };
                                match source 
                                {
                                    LoadByteSource::D8  => self.pc.wrapping_add(2),
                                    _                   => self.pc.wrapping_add(1),
                                }
                            }
                        LoadType::Word(target, source) =>
                            {
                                let source_value = match source
                                {
                                    LoadWordSource::AF => self.registers.get_af(),
                                    LoadWordSource::BC => self.registers.get_bc(),
                                    LoadWordSource::DE => self.registers.get_de(),
                                    LoadWordSource::HL => self.registers.get_hl(),
                                    LoadWordSource::D16 => self.read_next_word(),
                                    LoadWordSource::HLI => self.bus.read_word(self.registers.get_hl())
                                };
                                match target
                                {
                                    LoadWordTarget::AF => self.registers.set_af(source_value),
                                    LoadWordTarget::BC => self.registers.set_bc(source_value),
                                    LoadWordTarget::DE => self.registers.set_de(source_value),
                                    LoadWordTarget::HL => self.registers.set_hl(source_value),
                                    LoadWordTarget::HLI => self.bus.write_word(self.registers.get_hl(), source_value)
                                };
                                match source
                                {
                                    LoadWordSource::D16 => self.pc.wrapping_add(3),
                                    _                   => self.pc.wrapping_add(2),
                                }
                            }
                        LoadType::AFromIndirect(source) =>
                            {
                                match source
                                {
                                    LoadByteIndirect::BC => {self.registers.a = self.bus.read_byte(self.registers.get_bc());},
                                    LoadByteIndirect::DE => {self.registers.a = self.bus.read_byte(self.registers.get_de());},
                                    LoadByteIndirect::HLP => {self.registers.a = self.bus.read_byte(self.registers.get_hl()); let hl_add = self.registers.get_hl().wrapping_add(1); self.registers.set_hl(hl_add);},
                                    LoadByteIndirect::HLN => {self.registers.a = self.bus.read_byte(self.registers.get_hl()); let hl_add = self.registers.get_hl().wrapping_sub(1); self.registers.set_hl(hl_add);},
                                }
                                self.pc.wrapping_add(1)
                            }
                        LoadType::IndirectFromA(target) =>
                            {
                                match target
                                {
                                    LoadByteIndirect::BC => self.bus.write_byte(self.registers.get_bc(), self.registers.a),
                                    LoadByteIndirect::DE => self.bus.write_byte(self.registers.get_de(), self.registers.a),
                                    LoadByteIndirect::HLP => {self.bus.write_byte(self.registers.get_hl(), self.registers.a); let hl_add = self.registers.get_hl().wrapping_add(1); self.registers.set_hl(hl_add)},
                                    LoadByteIndirect::HLN => {self.bus.write_byte(self.registers.get_hl(), self.registers.a); let hl_add = self.registers.get_hl().wrapping_sub(1); self.registers.set_hl(hl_add)},
                                }
                                self.pc.wrapping_add(1)
                            }
                        LoadType::AFromByteAddress(source) =>
                            {
                                match source
                                {
                                    LoadByteAddress::C => self.registers.a = self.bus.read_byte(0xFF00 | (self.registers.c as u16)),
                                    LoadByteAddress::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.registers.a = self.bus.read_byte(0xFF00 | byte as u16);}
                                }
                                match source
                                {
                                    LoadByteAddress::C => self.pc + 1,
                                    _                  => self.pc + 2
                                }
                            }
                        LoadType::ByteAddressFromA(target) =>
                            {
                                match target
                                {
                                    LoadByteAddress::C => self.bus.write_byte((0xFF00 | self.registers.c as u16), self.registers.a),
                                    LoadByteAddress::U8 => {let byte = self.bus.read_byte(self.pc + 1); self.bus.write_byte((0xFF00 | byte as u16), self.registers.a);}
                                }
                                match target
                                {
                                    LoadByteAddress::C => self.pc + 1,
                                    _                  => self.pc + 2
                                }
                            }
                    }
                }
            }
        }
    fn add(&mut self, value: u8) -> u8
        {
            let (new_value, overflow) = self.registers.a.overflowing_add(value);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F; 
            self.registers.f.carry = overflow;
            new_value
        }
    fn addhl(&mut self, value: u16) -> u16
        {
            let (new_value, overflow) = self.registers.get_hl().overflowing_add(value);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = (self.registers.get_hl() & 0x0FFF) + (value & 0x0FFF) > 0x0FFF; 
            self.registers.f.carry = overflow;
            new_value
        }
    fn adc(&mut self, value: u8) -> u8
        {
            let (new_value, overflow) = self.registers.a.overflowing_add(value);
            let (final_value, second_overflow) = new_value.overflowing_add(self.registers.f.carry as u8);
            self.registers.f.zero = final_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) > 0x0F; 
            self.registers.f.carry = overflow | second_overflow;
            final_value
        }
    fn sub(&mut self, value: u8) -> u8
        {
            let (new_value, overflow) = self.registers.a.overflowing_sub(value);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = true; 
            self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F); 
            self.registers.f.carry = overflow;
            new_value
        }
    fn sbc(&mut self, value: u8) -> u8
        {
            let (new_value, overflow) = self.registers.a.overflowing_sub(value);
            let (final_value, second_overflow) = new_value.overflowing_sub(self.registers.f.carry as u8);
            self.registers.f.zero = final_value == 0; 
            self.registers.f.subtract = true; 
            self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F); 
            self.registers.f.carry = overflow | second_overflow;
            final_value
        }
    fn and(&mut self, value: u8)
        {
            self.registers.a = self.registers.a & value;
            self.registers.f.zero = self.registers.a == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = true; 
            self.registers.f.carry = false;
        }
    fn or(&mut self, value: u8)
        {
            self.registers.a = self.registers.a | value;
            self.registers.f.zero = self.registers.a == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = true; 
            self.registers.f.carry = false;
        }
    fn xor(&mut self, value: u8)
        {
            self.registers.a = (self.registers.a | value) & !(self.registers.a & value);
            self.registers.f.zero = self.registers.a == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = true; 
            self.registers.f.carry = false;
        }
    fn cp(&mut self, value: u8)
        {
            let (new_value, overflow) = self.registers.a.overflowing_sub(value);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = true; 
            self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F); 
            self.registers.f.carry = overflow;
        }
    fn inc_8(&mut self, value: u8) -> u8
        {
            let (new_value, overflow) = value.overflowing_add(1);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = (value & 0x0F) == 0x0F; 
            self.registers.f.carry = overflow;
            new_value
        }
    fn inc_16(&mut self, value: u16) -> u16
        {
            let (new_value, overflow) = value.overflowing_add(1);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = (value & 0x0FFF) == 0x0FFF; 
            self.registers.f.carry = overflow;
            new_value
        }
    fn dec_8(&mut self, value: u8) -> u8
        {
            let (new_value, overflow) = value.overflowing_sub(1);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = (value & 0x0F) < 1; 
            self.registers.f.carry = overflow;
            new_value
        }
    fn dec_16(&mut self, value: u16) -> u16
        {
            let (new_value, overflow) = value.overflowing_sub(1);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = 1 > (value & 0x0FFF); 
            self.registers.f.carry = overflow;
            new_value
        }
    fn ccf(&mut self)
        {
            self.registers.f.carry = !self.registers.f.carry;
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
        }
    fn scf(&mut self)
        {
            self.registers.f.carry = true;
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
        }
    fn rra(&mut self)
        {
            let oldcarry = (self.registers.f.carry as u8) << 7;
            let newcarry = (self.registers.a & 0x01) != 0;
            self.registers.a = (self.registers.a >> 1) | oldcarry;
            self.registers.f.carry = newcarry;
            self.registers.f.zero = false; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
        }
    fn rla(&mut self)
        {
            let oldcarry = self.registers.f.carry as u8;
            let newcarry = (self.registers.a & 0b10000000) != 0;
            self.registers.a = (self.registers.a << 1) | oldcarry;
            self.registers.f.carry = newcarry;
            self.registers.f.zero = false; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
        }
    fn rrca(&mut self)
        {
            let highest = (self.registers.a & 0x01) << 7;
            self.registers.f.carry = (highest >> 7) != 0;
            self.registers.a = (self.registers.a >> 1) | highest;
            self.registers.f.zero = false; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
        }
    fn rlca(&mut self)
        {
            let lowest = (self.registers.a & 0b10000000) >> 7;
            self.registers.f.carry = (lowest) != 0;
            self.registers.a = (self.registers.a << 1) | lowest;
            self.registers.f.zero = false; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
        }
    fn cpl(&mut self)
        {
            self.registers.a = !self.registers.a;
            self.registers.f.subtract = true; 
            self.registers.f.half_carry = true;
        }
    fn bit(&mut self, bit: u8, value: u8)
        {
            let to_check = (value >> bit) & 0x01;
            self.registers.f.zero = to_check == 0;
            self.registers.f.subtract = false;
            self.registers.f.half_carry = true;
        }
    fn res(&mut self, bit: u8, value: u8) -> u8
        {
            let bit_num = !(0x01 << bit);
            let new_value = value & bit_num;
            new_value
        }
    fn set(&mut self, bit: u8, value: u8) -> u8
        {
            let bit_num = 0x01 << bit;
            let new_value = value | bit_num;
            new_value
        }
    fn srl(&mut self, value: u8) -> u8
        {
            self.registers.f.carry = (value & 0x01) != 0;
            let new_value = value >> 1;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            new_value
        }
    fn rr(&mut self, value: u8) -> u8
        {
            let old_carry = (self.registers.f.carry as u8) << 7;
            self.registers.f.carry = (value & 0x01) != 0;
            let new_value = (value >> 1) | old_carry;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            new_value
        }
    fn rl(&mut self, value: u8) -> u8
        {
            let old_carry = self.registers.f.carry as u8;
            self.registers.f.carry = (value & 0b10000000) != 0;
            let new_value = (value << 1) | old_carry;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            new_value
        }
    fn rrc(&mut self, value: u8) -> u8
        {
            self.registers.f.carry = (value & 0x01) != 0;
            let new_value = (value >> 1) | (self.registers.f.carry as u8) << 7;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            new_value
        }
    fn rlc(&mut self, value: u8) -> u8
        {
            self.registers.f.carry = (value & 0b10000000) != 0;
            let new_value = (value << 1) | self.registers.f.carry as u8;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            new_value
        }
    fn sra(&mut self, value: u8) -> u8
        {
            let msb = value & 0b10000000;
            self.registers.f.carry = value & 0x01 != 0;
            let new_value = (value >> 1) | msb;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            new_value
        }
    fn sla(&mut self, value: u8) -> u8
        {
            self.registers.f.carry = (value & 0b10000000) != 0;
            let new_value = value << 1;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            new_value
        }
    fn swap(&mut self, value: u8) -> u8
        {
            let new_value = value >> 4 | value << 4;
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = false;
            self.registers.f.carry = false;
            new_value
        }
    fn daa(&mut self) 
        {
            let mut correction: u8 = 0;
    
            if self.registers.f.half_carry || (self.registers.a & 0x0F) > 9 {
                correction |= 0x06;
            }
    
            if self.registers.f.carry || (self.registers.a > 0x99) {
                correction |= 0x60;
                self.registers.f.carry = true;
            } else {
                self.registers.f.carry = false;
            }
    
            if self.registers.f.subtract {
                self.registers.a = self.registers.a.wrapping_sub(correction);
            } else {
                self.registers.a = self.registers.a.wrapping_add(correction);
            }
    
            self.registers.f.half_carry = false;
            self.registers.f.zero = self.registers.a == 0;
        }
    fn jump(&mut self, should_jump: bool) -> u16 {
        if should_jump {
          // Gameboy is little endian so read pc + 2 as most significant bit
          // and pc + 1 as least significant bit
          let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
          let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
          (most_significant_byte << 8) | least_significant_byte
        } else {
          // If we don't jump we need to still move the program
          // counter forward by 3 since the jump instruction is
          // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
          self.pc.wrapping_add(3)
        }
      }
}