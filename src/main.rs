#![allow(dead_code)]
#![allow(unused_variables)]

mod utils;
mod gameboy;

fn main() {
    let memory = gameboy::Memory::new();
    let cpu = gameboy::Cpu::new();

}
