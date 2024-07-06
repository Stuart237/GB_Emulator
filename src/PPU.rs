pub const VRAM_SIZE: usize = 0x1800;
pub const TILE_COUNT: usize = 384;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const NUMBER_OF_OBJECTS: usize = 40;
pub const OBJECT_ATTRIBUTE_MEMORY_SIZE: usize = 0xA0;

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
pub struct Palette(Colour, Colour, Colour, Colour);

impl Palette
{
    fn new() -> Palette
    {
        Palette
        (
            Colour::White,
            Colour::LightGray,
            Colour::DarkGray,
            Colour::Black,
        )
    }
}
//Palette can be altered, so this must be implemented. Also why above is 4 colours as opposed to using Colour again.
impl std::convert::From<u8> for Palette
{
    fn from(value: u8) -> Self
    {
        Palette
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
#[derive(Clone, Copy)]
enum ObjectPalette
{
    Zero, One
}
enum TileMapArea
{
    X9800,
    X9C00
}
enum BGWindowTiles
{
    X8000,
    X8800
}
enum ObjectSize
{
    O8x8,
    O8x16
}
#[derive(Clone, Copy)]
struct Object
{
    x: u8,
    y: u8,
    tile: u8,
    x_flip: bool,
    y_flip: bool,
    pallette: ObjectPalette,
    priority: bool,
    tile_index: u8,
}
impl Object
{
    fn new() -> Object
    {
        Object
        {
            x: 0,
            y: 0,
            tile: 0,
            x_flip: false,
            y_flip: false,
            pallette: ObjectPalette::Zero,
            priority: false,
            tile_index: 0,
        }
    }
}
#[derive(Eq, PartialEq)]
pub enum Interrupts
{
    None,
    VBlank,
    LCD,
    Both,
}
impl Interrupts
{
    pub fn add(&mut self, interr: Interrupts)
    {
        match self
        {
            Interrupts::LCD => if interr == Interrupts::VBlank {*self = Interrupts::Both},
            Interrupts::VBlank => if interr == Interrupts::LCD {*self = Interrupts::Both},
            Interrupts::None => *self = interr,
            _ => {},
        }
    }
}
pub struct PPU
{
    vram: [u8; VRAM_SIZE],
    oam: [Object; NUMBER_OF_OBJECTS],
    tiles: [Tile; TILE_COUNT],
    cycles: u16,
    mode: PPUModes,
    obp0: Palette,
    obp1: Palette,
    lcd_enabled: bool,
    window_tilemap: TileMapArea,
    window_enabled: bool,
    bg_window_tiles: BGWindowTiles,
    bg_tilemap: TileMapArea,
    object_size: ObjectSize,
    object_enabled: bool,
    bg_window_enabled: bool,
    ly: u8,
    lyc: u8,
    ly_is_lyc: bool,
    lyc_selected: bool,
    oamscan_selected: bool,
    vblank_selected: bool,
    hblank_selected: bool,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
}
#[derive(Clone, Copy)]
pub enum PPUModes
{
    OAMScan,
    PixelTransfer,
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
            oam: [Object::new(); NUMBER_OF_OBJECTS],
            tiles: [empty_tile(); TILE_COUNT],
            cycles: 0,
            mode: PPUModes::OAMScan,
            obp0: Palette::new(),
            obp1: Palette::new(),
            lcd_enabled: true,
            window_tilemap: TileMapArea::X9800,
            window_enabled: false,
            bg_window_tiles: BGWindowTiles::X8000,
            bg_tilemap: TileMapArea::X9800,
            object_size: ObjectSize::O8x8,
            object_enabled: false,
            bg_window_enabled: true,
            ly: 0,
            lyc: 0,
            ly_is_lyc: false,
            lyc_selected: false,
            oamscan_selected: false,
            vblank_selected: false,
            hblank_selected: false,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
        }
    }
    pub fn write_to_vram(&mut self, address: usize, value: u8)
    {
        self.vram[address] = value;
        if address >= 0x1800
        {
            return;
        }
        else
        {
            self.write_oam(address, value);   
        }
        //We need to recreate the tile row if we change one of its bytes. Remember, tiles' rows start at even addresses.
        let tile_start_add = address & 0xFFFE;
        let byte1 = self.vram[tile_start_add];
        let byte2 = self.vram[tile_start_add + 1];

        let tile = address / 16;
        let tile_row = (address % 16) / 2;
        for i in 0..8
        {
            let msb = byte2 & (1 << (7 - i));
            let lsb = byte1 & (1 << (7 - i));
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
    pub fn write_oam(&mut self, address: usize, value: u8)
    {
        let byte = address % 4;
        let index = address / 4;
        match byte
        {
            0   => self.oam[index].y = value,
            1   => self.oam[index].x = value,
            2   => {
                        match self.object_size
                        {
                            ObjectSize::O8x8 => {self.oam[index].tile_index = value;},
                            ObjectSize::O8x16 => {self.oam[index].tile_index = value & 0xFE;},
                        }
                    }
            3   =>  {
                        self.oam[index].priority = (value & 0x80) != 0;
                        self.oam[index].y_flip = (value & 0x40) != 0;
                        self.oam[index].x_flip = (value & 0x20) != 0;
                        if (value & 0x10) != 0
                        {
                            self.oam[index].pallette = ObjectPalette::Zero;
                        }
                        else 
                        {
                            self.oam[index].pallette = ObjectPalette::One;    
                        }
                    }
            _   => panic!("WRITING TO UNKNOWN OBJECT 0X{:x}", address),
        }
    }
    pub fn read_oam(&mut self, address: usize) -> u8
    {
        let byte = address % 4;
        let index = address / 4;
        match byte
        {
            0   => self.oam[index].y,
            1   => self.oam[index].x,
            2   =>  {
                        match self.object_size
                        {
                            ObjectSize::O8x8 => {self.oam[index].tile_index},
                            ObjectSize::O8x16 => {self.oam[index].tile_index & 0xFE},
                        }
                    }
            3   =>  {
                        0x0 | ((self.oam[index].priority as u8) << 7)
                        | ((self.oam[index].y_flip as u8) << 6)
                        | ((self.oam[index].x_flip as u8) << 5)
                        | match self.oam[index].pallette {ObjectPalette::One => {1 << 4}, ObjectPalette::Zero => {0 << 4}}
                    },
            _   => panic!("READING FROM UNKNOWN LINE 0X{:x}", address),
        }
    }
    pub fn step(&mut self, cycles: u8) -> Interrupts
    {
        let mut request = Interrupts::None;
        let mode = self.mode;
        self.cycles += cycles as u16;
        match mode
        {
            PPUModes::OAMScan => 
            {
                if self.cycles >= 80
                {
                    self.cycles = self.cycles % 80;
                    self.mode = PPUModes::PixelTransfer;
                }
            },
            PPUModes::PixelTransfer => 
            {
                if self.cycles >= 172
                {
                    self.cycles = self.cycles % 172;
                    if self.hblank_selected
                    {
                        request.add(Interrupts::LCD);
                    }
                    self.mode = PPUModes::HBlank;
                    //Render a scan line here
                }
            },
            PPUModes::HBlank => 
            {
                if self.cycles >= 284
                {
                    self.cycles = self.cycles % 284;
                    self.ly += 1;
                    if self.ly >= 144
                    {
                        if self.vblank_selected
                        {
                            self.mode = PPUModes::VBlank;
                            request.add(Interrupts::VBlank);
                            if self.vblank_selected
                            {
                                request.add(Interrupts::LCD);
                            }
                        }
                        else
                        {
                            self.mode = PPUModes::OAMScan;
                            if self.oamscan_selected
                            {
                                request.add(Interrupts::LCD);
                            }
                        }
                        request = self.lyc_check(request);
                    }
                }
            },
            PPUModes::VBlank => 
            {
                if self.cycles >= 456
                {
                    self.cycles = self.cycles % 456;
                    self.ly += 1;
                    if self.ly == 154
                    {
                        self.mode = PPUModes::OAMScan;
                        self.ly = 0;
                        if self.oamscan_selected
                        {
                            request.add(Interrupts::LCD);
                        }
                    }
                    request = self.lyc_check(request);
                }
            },
        }
        request
    }
    pub fn lyc_check(&mut self, mut request: Interrupts) -> Interrupts
    {
        let check = self.ly == self.lyc;
        if check & self.lyc_selected
        {
            request.add(Interrupts::LCD);
        }
        self.ly_is_lyc = check;
        request
    }
}