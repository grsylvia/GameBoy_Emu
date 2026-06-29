const FRAMES_PER_SECOND: f32 = 59.7;
const CPU_HZ: f32 = 4.19e6;
const CYCLES_PER_FRAME: f32 = CPU_HZ / FRAMES_PER_SECOND;

// load CPU core module
mod cpu;

fn main() {
    let mut dmg = cpu::GameBoy::new();

    dmg.dump_registers();
    dmg.dump_memory();
}
