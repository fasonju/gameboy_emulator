mod utils;
mod gameboy;

fn main() {
    let cpu = gameboy::Cpu::new();

    println!("AF: {:04X}", cpu.get_af());
}
