use std::fs::File;
use std::io::Read;
use chip8::Chip8;

mod memory;
mod chip8;

fn main() {
    let digits = [0xF0, 0x90, 0x90, 0x90, 0xF0,
                  0x20, 0x60, 0x20, 0x20, 0x70, 
                  0xF0, 0x10, 0xF0, 0x80, 0xF0,    
                  0xF0, 0x10, 0xF0, 0x10, 0xF0, 
                  0x90, 0x90, 0xF0, 0x10, 0x10,  
                  0xF0, 0x80, 0xF0, 0x10, 0xF0,  
                  0xF0, 0x80, 0xF0, 0x90, 0xF0,  
                  0xF0, 0x10, 0x20, 0x40, 0x40,
                  0xF0, 0x90, 0xF0, 0x90, 0xF0,
                  0xF0, 0x90, 0xF0, 0x10, 0xF0,
                  0xF0, 0x90, 0xF0, 0x90, 0x90,
                  0xE0, 0x90, 0xE0, 0x90, 0xE0,
                  0xF0, 0x80, 0x80, 0x80, 0xF0,
                  0xE0, 0x90, 0x90, 0x90, 0xE0,
                  0xF0, 0x80, 0xF0, 0x80, 0xF0,
                  0xF0, 0x80, 0xF0, 0x80, 0x80];
    let mut file = File::open("roms/TANK").unwrap();
    let mut rom = Vec::<u8>::new();
    file.read_to_end(&mut rom).unwrap();
    let mut chip8 = Chip8::new();
    chip8.load_digits(&digits);
    chip8.load_cart(&rom);
    //print!("{:#X?}", &chip8.ram.mem[0x200 as usize .. rom.len() as usize])
    print!("{:X?}", &chip8.ram.mem[..])
}
