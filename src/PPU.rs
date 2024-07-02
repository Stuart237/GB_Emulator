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
enum ObjectPalette
{
    Zero, One
}
struct Object
{
    x: i16,
    y: i16,
    tile: u8,
    x_flip: bool,
    y_flip: bool,
    pallette: ObjectPalette
}

fn empty_tile() -> Tile
{
    [[TilePixelValue::Zero; 8]; 8]
}
pub const VRAM_SIZE: usize = 0x2000;
pub const TILE_COUNT: usize = 384;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const NUMBER_OF_OBJECTS: u8 = 40;
pub struct PPU
{
    vram: [u8; VRAM_SIZE],
    tiles: [Tile; TILE_COUNT],
    mode: PPUModes,
}

pub enum PPUModes
{
    OAMScan,
    DrawingPixels,
    VBlank,
    HBlank
}
impl PPU
{
    pub fn new() -> PPU
    {
        PPU
        {
            vram: [0; VRAM_SIZE],
            tiles: [empty_tile(); TILE_COUNT],
            mode: PPUModes::OAMScan,
        }
    }
    pub fn write_to_vram(&mut self, address: usize, value: u8)
    {
        self.vram[address] = value;
        if address >= 1800
        {
            return;
        }
        //We need to recreate the tile row if we change one of its bytes. Remember, tiles' rows start at even addresses.
        let tile_start_add = address & 0xFFFE;
        let byte1 = self.vram[tile_start_add];
        let byte2 = self.vram[tile_start_add + 1];

        let tile = address / 16;
        let tile_row = (address % 16) / 2;
        for i in 0..8
        {
            let msb = (byte2 << (7 - i));
            let lsb = (byte1 << (7 - i));
            let pixel_colour = match (msb != 0, lsb != 0)
            {
                (true, true) => TilePixelValue::Three,
                (true, false) => TilePixelValue::Two,
                (false, true) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero
            };
            self.tiles[i][tile_row][tile] = pixel_colour;
        }
    }
    pub fn read_from_vram(&mut self, address: usize) -> u8
    {
        self.vram[address]
    }
}