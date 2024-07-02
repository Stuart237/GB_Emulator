struct Joypad
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
        
    }
}