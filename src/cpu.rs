#[path = "opcodes.rs"]
mod opcodes;

pub struct Registers {
    a: u8,
    f: u8,
    // flag register
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    // stack counter
    pc: u16
    // program counter
}

pub struct Bus {
    // format => [u8; size of memory section in hex]
    rom: [u8; 0x8000],
    vram: [u8; 0x2000],
    sram: [u8; 0x2000],
    wram0: [u8; 0x1000],
    wramx: [u8; 0x1000],
    echo_ram: [u8; 0x1EFF],
    oam: [u8; 0x00A0],
    prohibited: [u8; 0x0060],
    io: [u8; 0x0080],
    hram: [u8; 0x007F],
    ie: [u8; 0x0001]
}

impl Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.sram[(addr - 0xA000) as usize],
            0xC000..=0xCFFF => self.wram0[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => self.wramx[(addr - 0xD000) as usize],
            0xE000..=0xFDFF => self.echo_ram[(addr - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => self.prohibited[(addr - 0xFEA0) as usize],
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie[0],
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize] = value,
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.sram[(addr - 0xA000) as usize] = value,
            0xC000..=0xCFFF => self.wram0[(addr - 0xC000) as usize] = value,
            0xD000..=0xDFFF => self.wramx[(addr - 0xD000) as usize] = value,
            0xE000..=0xFDFF => self.echo_ram[(addr - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => self.prohibited[(addr - 0xFEA0) as usize] = value,
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie[0] = value,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Instruction {
    high_nibble: u8,
    low_nibble: u8,
}

pub struct GameBoy {

}

impl GameBoy {
    fn new() -> Self {
        GameBoy {

        }
    }
}
