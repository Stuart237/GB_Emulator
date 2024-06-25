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
    registers: Registers
}

enum Instructions
{
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget16),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget)
}
enum ArithmeticTarget
{
    A, B, C, D, E, H, L
}
enum ArithmeticTarget16
{
    HL, BC, DE, AF
}

impl CPU
{
    fn execute(&mut self, instruction: Instructions)
        {
            match instruction
            {
                Instructions::ADD(target) => 
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
                Instructions::ADDHL(target) =>
                {
                    match target
                    {
                        ArithmeticTarget16::AF => {let af = self.registers.get_af(); let result = self.addhl(af); self.registers.set_hl(result);}
                        ArithmeticTarget16::BC => {let bc = self.registers.get_bc(); let result = self.addhl(bc); self.registers.set_hl(result);}
                        ArithmeticTarget16::DE => {let de = self.registers.get_de(); let result = self.addhl(de); self.registers.set_hl(result);}
                        ArithmeticTarget16::HL => {let hl = self.registers.get_hl(); let result = self.addhl(hl); self.registers.set_hl(result);}
                    }
                }
                Instructions::ADC(target) =>
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
                Instructions::SUB(target) => 
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
                Instructions::SBC(target) =>
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
                Instructions::AND(target) =>
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
                Instructions::OR(target) =>
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
                Instructions::XOR(target) =>
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
            let (new_value, overflow) = (self.registers.a as u16).overflowing_add(value);
            self.registers.f.zero = new_value == 0; 
            self.registers.f.subtract = false; 
            self.registers.f.half_carry = ((self.registers.a as u16) & 0x0FFF) + (value & 0x0FFF) > 0x0FFF; 
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
}