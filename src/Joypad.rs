#[derive(Clone, Copy)]
pub struct Joypad
{
    select_buttons: bool,
    select_dpad: bool,
    start_down: bool,
    select_up: bool,
    b_left: bool,
    a_right: bool
}
impl std::convert::From<u8> for Joypad
{
    fn from(value: u8) -> Self 
    {
        Joypad
        {
            select_buttons: (value & 32) == 0,
            select_dpad: (value & 16) == 0,
            start_down: (value & 8) == 0,
            select_up: (value & 4) == 0,
            b_left: (value & 2) == 0,
            a_right: (value & 1) == 0,
        }    
    }
}
impl std::convert::From<Joypad> for u8
{
    fn from(value: Joypad) -> Self 
    {
        let reg: u8 = ((value.select_buttons as u8) << 5) | ((value.select_dpad as u8) << 4) | ((value.start_down as u8) << 3) | ((value.select_up as u8) << 2 | ((value.b_left as u8) << 1)) | (value.a_right as u8);
        reg
    }
}
impl Joypad
{
    pub fn new() -> Self
    {
        Joypad 
        { 
            select_buttons: true,
            select_dpad: true,
            start_down: true,
            select_up: true,
            b_left: true,
            a_right: true 
        }
    }
    pub fn reset_joypad(&mut self)
    {
        self.select_buttons = true;
        self.select_dpad = true;
        self.start_down = true;
        self.select_up = true;
        self.b_left = true;
        self.a_right = true;
    }
}