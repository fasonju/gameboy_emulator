#![allow(dead_code)]
#![allow(unused_variables)]

mod gameboy;
mod utils;

fn main() {
    let memory = gameboy::Memory::new();
    let cpu = gameboy::Cpu::new();
}
