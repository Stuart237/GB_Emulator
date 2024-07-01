pub enum Colour
{
    White = 255,
    LightGray = 192,
    DarkGray = 96,
    Black = 0,
}
impl std::convert::From<u8> for Colour
{
    fn from(value: u8) -> Self 
    {
        match value
        {
            0 => Colour::White,
            1 => Colour::LightGray,
            2 => Colour::DarkGray,
            3 => Colour::Black,
            _ => panic!("Invalid conversion.")
        }
    }
}
pub struct BackgroundColors(Colour, Colour, Colour, Colour);

impl BackgroundColors
{
    fn new() -> BackgroundColors
    {
        BackgroundColors
        (
            Colour::White,
            Colour::LightGray,
            Colour::DarkGray,
            Colour::Black,
        )
    }
}
//Palette can be altered, so this must be implemented. Also why above is just 4 colours as opposed to using Colour again.
impl std::convert::From<u8> for BackgroundColors
{
    fn from(value: u8) -> Self
    {
        BackgroundColors
        (
            (value & 0b11).into(),
            (value >> 2 & 0b11).into(),
            (value >> 4 & 0b11).into(),
            (value >> 6 & 0b11).into(),
        )
    }
}
#[derive(Clone, Copy)]
enum TilePixelValue
{
    Zero,
    One,
    Two,
    Three
}
type Tile = [[TilePixelValue; 8]; 8];

fn empty_tile() -> Tile
{
    [[TilePixelValue::Zero; 8]; 8]
}
pub struct PPU
{
    
}