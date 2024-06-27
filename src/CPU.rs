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
    SWAP(ArithmeticTarget)
}
enum ArithmeticTarget
{
    A, B, C, D, E, H, L
}
enum ArithmeticTarget16
{
    HL, BC, DE, AF
}

impl Instruction
{
    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction>
        {
            match byte
                {
                    0x03 => Some(Instruction::INC16(ArithmeticTarget16::BC)),
                    0x04 => Some(Instruction::INC8(ArithmeticTarget::B)),
                    0x05 => Some(Instruction::DEC8(ArithmeticTarget::B)),
                    0x07 => Some(Instruction::RLCA()),
                    0x09 => Some(Instruction::ADDHL(ArithmeticTarget16::BC)),
                    0x0B => Some(Instruction::DEC16(ArithmeticTarget16::BC)),
                    0x0C => Some(Instruction::INC8(ArithmeticTarget::C)),
                    0x0D => Some(Instruction::DEC8(ArithmeticTarget::C)),
                    0x0F => Some(Instruction::RRCA()),
                    0x13 => Some(Instruction::INC16(ArithmeticTarget16::DE)),
                    0x14 => Some(Instruction::INC8(ArithmeticTarget::D)),
                    0x15 => Some(Instruction::DEC8(ArithmeticTarget::D)),
                    0x17 => Some(Instruction::RLA()),
                    0x19 => Some(Instruction::ADDHL(ArithmeticTarget16::DE)),
                    0x1B => Some(Instruction::DEC16(ArithmeticTarget16::DE)),
                    0x1C => Some(Instruction::INC8(ArithmeticTarget::E)),
                    0x1D => Some(Instruction::DEC8(ArithmeticTarget::E)),
                    0x1F => Some(Instruction::RRA()),
                    0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),

                }
        }
}

impl CPU
{
    fn step(&mut self)
        {
            let mut instruction_byte = self.bus.read_byte(self.pc);

            let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte)
                {
                    self.execute(instruction)
                }
            else
                {
                    panic!("Unknown instruction found for 0x{:x}", instruction_byte);
                };
            self.pc = next_pc;
        }

    fn execute(&mut self, instruction: Instruction)
        {
            match instruction
            {
                Instruction::ADD(target) => 
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.add(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.a = self.add(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.a = self.add(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.a = self.add(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.a = self.add(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.a = self.add(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.a = self.add(self.registers.l);}
                    }
                }
                Instruction::ADDHL(target) =>
                {
                    match target
                    {
                        ArithmeticTarget16::AF => {let af = self.registers.get_af(); let result = self.addhl(af); self.registers.set_hl(result);}
                        ArithmeticTarget16::BC => {let bc = self.registers.get_bc(); let result = self.addhl(bc); self.registers.set_hl(result);}
                        ArithmeticTarget16::DE => {let de = self.registers.get_de(); let result = self.addhl(de); self.registers.set_hl(result);}
                        ArithmeticTarget16::HL => {let hl = self.registers.get_hl(); let result = self.addhl(hl); self.registers.set_hl(result);}
                    }
                }
                Instruction::ADC(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.adc(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.a = self.adc(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.a = self.adc(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.a = self.adc(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.a = self.adc(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.a = self.adc(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.a = self.adc(self.registers.l);}   
                    }
                }
                Instruction::SUB(target) => 
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sub(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.a = self.sub(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.a = self.sub(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.a = self.sub(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.a = self.sub(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.a = self.sub(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.a = self.sub(self.registers.l);}
                    }
                }
                Instruction::SBC(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sbc(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.a = self.sbc(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.a = self.sbc(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.a = self.sbc(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.a = self.sbc(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.a = self.sbc(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.a = self.sbc(self.registers.l);}  
                    }
                }
                Instruction::AND(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.and(self.registers.a);}
                        ArithmeticTarget::B => {self.and(self.registers.b);}
                        ArithmeticTarget::C => {self.and(self.registers.c);}
                        ArithmeticTarget::D => {self.and(self.registers.d);}
                        ArithmeticTarget::E => {self.and(self.registers.e);}
                        ArithmeticTarget::H => {self.and(self.registers.h);}
                        ArithmeticTarget::L => {self.and(self.registers.l);}  
                    }
                } 
                Instruction::OR(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.or(self.registers.a);}
                        ArithmeticTarget::B => {self.or(self.registers.b);}
                        ArithmeticTarget::C => {self.or(self.registers.c);}
                        ArithmeticTarget::D => {self.or(self.registers.d);}
                        ArithmeticTarget::E => {self.or(self.registers.e);}
                        ArithmeticTarget::H => {self.or(self.registers.h);}
                        ArithmeticTarget::L => {self.or(self.registers.l);}  
                    }
                } 
                Instruction::XOR(target) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.xor(self.registers.a);}
                        ArithmeticTarget::B => {self.xor(self.registers.b);}
                        ArithmeticTarget::C => {self.xor(self.registers.c);}
                        ArithmeticTarget::D => {self.xor(self.registers.d);}
                        ArithmeticTarget::E => {self.xor(self.registers.e);}
                        ArithmeticTarget::H => {self.xor(self.registers.h);}
                        ArithmeticTarget::L => {self.xor(self.registers.l);}  
                    }
                }
                Instruction::CP(target) => 
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.cp(self.registers.a);}
                        ArithmeticTarget::B => {self.cp(self.registers.b);}
                        ArithmeticTarget::C => {self.cp(self.registers.c);}
                        ArithmeticTarget::D => {self.cp(self.registers.d);}
                        ArithmeticTarget::E => {self.cp(self.registers.e);}
                        ArithmeticTarget::H => {self.cp(self.registers.h);}
                        ArithmeticTarget::L => {self.cp(self.registers.l);}
                    }
                }
                Instruction::INC8(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.inc_8(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.inc_8(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.inc_8(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.inc_8(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.inc_8(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.inc_8(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.inc_8(self.registers.l);}
                    }
                }
                Instruction::INC16(target) =>
                {
                    match target
                    {
                        ArithmeticTarget16::AF => {let af = self.registers.get_af(); let result = self.inc_16(af); self.registers.set_af(result);}
                        ArithmeticTarget16::BC => {let bc = self.registers.get_bc(); let result = self.inc_16(bc); self.registers.set_bc(result);}
                        ArithmeticTarget16::DE => {let de = self.registers.get_de(); let result = self.inc_16(de); self.registers.set_de(result);}
                        ArithmeticTarget16::HL => {let hl = self.registers.get_hl(); let result = self.inc_16(hl); self.registers.set_hl(result);}
                    }
                }
                Instruction::DEC8(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.dec_8(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.dec_8(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.dec_8(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.dec_8(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.dec_8(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.dec_8(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.dec_8(self.registers.l);}
                    }
                }
                Instruction::DEC16(target) =>
                {
                    match target
                    {
                        ArithmeticTarget16::AF => {let af = self.registers.get_af(); let result = self.dec_16(af); self.registers.set_af(result);}
                        ArithmeticTarget16::BC => {let bc = self.registers.get_bc(); let result = self.dec_16(bc); self.registers.set_bc(result);}
                        ArithmeticTarget16::DE => {let de = self.registers.get_de(); let result = self.dec_16(de); self.registers.set_de(result);}
                        ArithmeticTarget16::HL => {let hl = self.registers.get_hl(); let result = self.dec_16(hl); self.registers.set_hl(result);}
                    }
                }
                Instruction::CCF() =>
                {
                    self.ccf();
                }
                Instruction::SCF() =>
                {
                    self.scf();
                }
                Instruction::RRA() =>
                {
                    self.rra();
                }
                Instruction::RLA() =>
                {
                    self.rla();
                }
                Instruction::RRCA() =>
                {
                    self.rrca();
                }
                Instruction::RLCA() =>
                {
                    self.rlca();
                }
                Instruction::CPL() =>
                {
                    self.cpl();
                }
                Instruction::BIT(target, bit) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.bit(bit, self.registers.a);}
                        ArithmeticTarget::B => {self.bit(bit, self.registers.b);}
                        ArithmeticTarget::C => {self.bit(bit, self.registers.c);}
                        ArithmeticTarget::D => {self.bit(bit, self.registers.d);}
                        ArithmeticTarget::E => {self.bit(bit, self.registers.e);}
                        ArithmeticTarget::H => {self.bit(bit, self.registers.h);}
                        ArithmeticTarget::L => {self.bit(bit, self.registers.l);}
                    }
                }
                Instruction::RES(target, bit) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.res(bit, self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.res(bit, self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.res(bit, self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.res(bit, self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.res(bit, self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.res(bit, self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.res(bit, self.registers.l);}    
                    }
                }
                Instruction::SET(target, bit) =>
                {
                    match target 
                    {
                        ArithmeticTarget::A => {self.registers.a = self.set(bit, self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.set(bit, self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.set(bit, self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.set(bit, self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.set(bit, self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.set(bit, self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.set(bit, self.registers.l);}    
                    }
                }
                Instruction::SRL(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.srl(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.srl(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.srl(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.srl(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.srl(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.srl(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.srl(self.registers.l);}
                    }
                }
                Instruction::RR(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rr(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.rr(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.rr(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.rr(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.rr(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.rr(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.rr(self.registers.l);}
                    }
                }
                Instruction::RL(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rl(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.rl(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.rl(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.rl(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.rl(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.rl(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.rl(self.registers.l);}
                    }
                }
                Instruction::RRC(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rrc(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.rrc(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.rrc(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.rrc(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.rrc(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.rrc(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.rrc(self.registers.l);}
                    }
                }
                Instruction::RLC(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.rlc(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.rlc(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.rlc(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.rlc(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.rlc(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.rlc(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.rlc(self.registers.l);}
                    }
                }
                Instruction::SRA(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sra(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.sra(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.sra(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.sra(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.sra(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.sra(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.sra(self.registers.l);}
                    }
                }
                Instruction::SLA(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.sla(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.sla(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.sla(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.sla(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.sla(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.sla(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.sla(self.registers.l);}
                    }
                }
                Instruction::SWAP(target) =>
                {
                    match target
                    {
                        ArithmeticTarget::A => {self.registers.a = self.swap(self.registers.a);}
                        ArithmeticTarget::B => {self.registers.b = self.swap(self.registers.b);}
                        ArithmeticTarget::C => {self.registers.c = self.swap(self.registers.c);}
                        ArithmeticTarget::D => {self.registers.d = self.swap(self.registers.d);}
                        ArithmeticTarget::E => {self.registers.e = self.swap(self.registers.e);}
                        ArithmeticTarget::H => {self.registers.h = self.swap(self.registers.h);}
                        ArithmeticTarget::L => {self.registers.l = self.swap(self.registers.l);}
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
}