pub enum Frequency
{
    //Different timer frequencies available on gameboy
    F4096,
    F16384,
    F65536,
    F262144
}

impl Frequency
{
    //how many cycles makes the timer tick over
    fn cycles_per_tick(&self) -> usize
    {
        match self
        {
            Frequency::F4096 => 1024,
            Frequency::F16384 => 256,
            Frequency::F65536 => 64,
            Frequency::F262144 => 16,
        }
    }
}

pub struct Timer
{
    //Frequency for obvious reasons, value is timer value (duh), cycles is current cycles, modulo is what value is set to if there is overflow.
    //Kinda like a baseline for the timer, like how Void Fiend from Risk of Rain 2 has a baseline on its corruption meter.
    //Enabled flag for obvious reasons
    pub frequency: Frequency,
    pub value: u8,
    pub cycles: usize,
    pub modulo: u8,
    pub enabled: bool
}
impl Timer
{
    pub fn new(freq: Frequency) -> Self
    {
        Timer
        {
            frequency: freq,
            value: 0,
            cycles: 0,
            modulo: 0,
            enabled: false,
        }
    }
    //Advance the timer by how many cycles the cpu just did with all them instructions n that
    pub fn step(&mut self, cycles: u8) -> bool
    {
        if !self.enabled
        {
            return false;
        }
        self.cycles += cycles as usize;
        let cpt = self.frequency.cycles_per_tick();
        let overflowed = if self.cycles > cpt 
        {
            self.cycles = self.cycles % cpt;
            let (new, overflow) = self.value.overflowing_add(1);
            self.value = new;
            overflow
        }
        else
        {
            return false;
        };
        if overflowed
        {
            self.value = self.modulo;
        }
        overflowed
    }
}