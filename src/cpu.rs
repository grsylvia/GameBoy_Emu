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
    // stack pointer
    pc: u16
    // program counter
}

impl Registers {
    pub fn new() -> Self {
        // set initial DMG register values
        // register values a and b used by game to detect hardware
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
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
    fn new() -> Self {
        Bus {
            rom: [0x00; 0x8000],
            vram: [0x00; 0x2000],
            sram: [0x00; 0x2000],
            wram0: [0x00; 0x1000],
            wramx: [0x00; 0x1000],
            echo_ram: [0x00; 0x1EFF],
            oam: [0x00; 0x00A0],
            prohibited: [0x00; 0x0060],
            io: [0x00; 0x0080],
            hram: [0x00; 0x007F],
            ie: [0x00; 0x0001]
        }
    }

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
    reg: Registers,
    bus: Bus,
}


impl GameBoy {
    fn new() -> Self {
        GameBoy {
            reg: Registers::new(),
            bus: Bus::new()
        }
    }

    pub fn fetch_opcode(&mut self) -> u8 {
        // for the gameboy, opcodes are a single byte, and operands are separate bytes in the stream
        // pull opcode from memory
        let opcode: u8 = self.bus.read(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(0x0001);
        opcode
    }

    pub fn fetch_operands(&mut self, operand_count: u8) -> Vec<u8> {
        // based on number of trailing operands, pull the operands into the vector
        let mut operands: Vec<u8> = Vec::new();
        for byte_count in 0..operand_count {
            operands.push(self.bus.read(self.reg.pc.wrapping_add(byte_count as u16)));
        }

        self.reg.pc = self.reg.pc.wrapping_add(operands.len() as u16);
        operands
    }
    
    pub fn decode(&mut self, opcode: u8) {
        opcodes::OPCODE_TABLE[opcode as usize](self);
    }
}
