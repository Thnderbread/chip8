#![allow(dead_code)]

use std::{collections::HashMap, fs, path::PathBuf};

use minifb::{Key, Window, WindowOptions};

use crate::DISPLAY_SIZE;

// memeory available to the emulator
const MEMORY_SIZE: usize = 4096;

// Programs start at address 200
const PROGRAM_STARTING_ADDR: usize = 0x200;

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    display: [u32; DISPLAY_SIZE],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    keys: HashMap<Key, &'static str>,
    pc: usize,
    v: [u8; 16],
    i: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut em = Self {
            memory: [0; MEMORY_SIZE],
            display: [0; DISPLAY_SIZE],
            // Emulate original space limitation (16 2-byte entries)
            stack: Vec::with_capacity(16),
            delay_timer: 0,
            sound_timer: 0,
            keys: HashMap::with_capacity(16),
            pc: PROGRAM_STARTING_ADDR,
            v: [0; 16],
            i: 0,
        };
        em.load_font();
        em.set_keys();
        em
    }

    pub fn run(&mut self) {
        let next_opcode = self.fetch_next_opcode();
        self.decode(next_opcode);
    }

    pub fn load_rom(&mut self, path: PathBuf) {
        let rom_buf = fs::read(path).unwrap_or_else(|e| {
            panic!("Couldn't read rom: {e}");
        });

        let mut ptr = PROGRAM_STARTING_ADDR;
        for byte in &rom_buf {
            self.memory[ptr] = *byte;
            ptr += 1;
        }

        println!("Rom loaded into memory!");
    }

    fn set_keys(&mut self) {
        self.keys.insert(Key::Key1, "1");
        self.keys.insert(Key::Key2, "2");
        self.keys.insert(Key::Key3, "3");
        self.keys.insert(Key::Key4, "C");
        self.keys.insert(Key::Q, "4");
        self.keys.insert(Key::W, "5");
        self.keys.insert(Key::E, "6");
        self.keys.insert(Key::R, "D");
        self.keys.insert(Key::A, "7");
        self.keys.insert(Key::S, "8");
        self.keys.insert(Key::D, "9");
        self.keys.insert(Key::F, "E");
        self.keys.insert(Key::A, "Z");
        self.keys.insert(Key::O, "X");
        self.keys.insert(Key::B, "C");
        self.keys.insert(Key::F, "V");
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }
    }

    pub fn handle_keypress(&mut self, key: &Key) {
        if let Some(input) = self.keys.get(key) {
            println!("Chip 8 key: {input}");
        }
    }

    pub fn get_display(&self) -> &[u32; DISPLAY_SIZE] {
        &self.display
    }

    fn load_font(&mut self) {
        let font = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        // convention to put font data at 0x050 - 0x9F
        let mut idx = 0x050;
        for char in font.iter() {
            self.memory[idx] = *char;
            idx += 1;
        }
    }

    // instruction is 2 bytes, so read 2 successive bytes
    // and combine them into a 16 bit instruction
    fn fetch_next_opcode(&mut self) -> u16 {
        // read current & next instructions
        let first_byte = self.memory[self.pc] as u16;
        let second_byte = self.memory[self.pc + 1] as u16;

        // increment PC by 2 to fetch next opcode
        self.pc += 2;

        // combine into one 16-bit instruction
        // shift first byte over by 8 to
        // make room for the next byte
        (first_byte << 8) | second_byte
    }

    // SECTION - bitmask
    // use a bitmask - data that defines which bits
    // to keep and which bits to clear
    // masking - applies the bitmask to a value
    // bitwise AND extracts a subset of bits in the value
    // bitwise OR sets a subset of bits in the value
    // bitwise XOR toggles a subset of bits in the value
    fn decode(&mut self, opcode: u16) {
        // this mask translates to 1111_0000_0000_0000
        // the first 4 bits are covered by F, we zero out
        // the other 3 to make it a 16-bit value.
        // can't use an 8-bit value because that would
        // only mask for the smallest (rightmost) bits
        // and zero out the ones we care about (leftmost).
        // finally, we shift right by 12 to only get what
        // we care about (leftmost 4 bits).
        // first nibble
        let high_nibble = (opcode & 0xF000) >> 12;
        // second nibble - used for lookup into V (vx)
        let x = ((opcode & 0x0F00) >> 8) as u8;
        // third nibble
        let y = (opcode & 0x00F0) >> 4;
        // fourth nibble
        let n = (opcode & 0xF) as u8;
        // 3rd & 4th nibbles
        let nn = (opcode & 0x00FF) as u8;
        // 2nd 3rd & 4th nibbles
        let nnn = opcode & 0x0FFF;

        match high_nibble {
            0x0 => {
                self.op_00e0();
            }
            0x1 => {
                self.op_1nnn(nnn);
            }
            0x2 => {
                unimplemented!("I Should be doing something here! - 2");
                //
            }
            0x3 => {
                unimplemented!("I Should be doing something here! - 3");
                //
            }
            0x4 => {
                unimplemented!("I Should be doing something here! - 4");
                //
            }
            0x5 => {
                unimplemented!("I Should be doing something here! - 5");
                //
            }
            0x6 => {
                self.op_6xnn(x, nn);
                //
            }
            0x7 => {
                self.op_7xnn(x, nn);
                //
            }
            0x8 => {
                unimplemented!("I Should be doing something here! - 8");
                //
            }
            0x9 => {
                unimplemented!("I Should be doing something here! - 9");
                //
            }
            0xA => {
                self.op_annn(nnn);
                //
            }
            0xB => {
                unimplemented!("I Should be doing something here! - B");
                //
            }
            0xC => {
                unimplemented!("I Should be doing something here! - C");
                //
            }
            0xD => {
                self.op_dxyn(n, x, y);
            }
            0xE => {
                unimplemented!("I Should be doing something here! - E");
                //
            }
            0xF => {
                unimplemented!("I Should be doing something here! - F");
                //
            }
            _ => {
                // satisfies u16::MIN & u16::MAX
                println!("Bad things are happening bro");
            }
        };
    }

    /// Clear the display - turn all pixels off to 0 (false)
    fn op_00e0(&mut self) {
        self.display.iter_mut().for_each(|pixel| *pixel = 0);
    }

    /// Sets PC to ```nnn```,
    /// Does not increment the PC afterwards.
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn as usize;
    }

    /// looks up register ```x``` and sets its value to ```nn```
    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.v[x as usize] = nn;
    }

    /// Add the value ```nn``` to VX.
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        self.v[x as usize] += nn;
    }

    /// Sets the index register I to ```nnn```
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    ///
    ///
    /// ## Arguments
    ///
    /// * `n` - Number of pixels in a sprite
    /// * `x` - Coordinate x from V[```x```] register
    /// * `y` - Coordinate y from V[```y```] register
    fn op_dxyn(&mut self, n: u8, x: u8, y: u16) {
        // using literals instead of DISPLAY constants
        // to minimize casting
        let pos_x = self.v[x as usize] % 64;
        let pos_y = self.v[y as usize] % 32;
        self.v[0xF] = 0;

        for pixel in 0..n {
            if pos_y + pixel >= 32 {
                break;
            }

            for sprite_bit in 0..8 {
                if pos_x + sprite_bit >= 64 {
                    break;
                }

                let pixel_row = self.memory[self.i as usize + pixel as usize];
                let mask: u8 = 0b1000_0000;

                // which bits in this pixel row are set?
                if pixel_row & (mask >> sprite_bit) != 0 {
                    // get row and column offsets, multiply to get actual position
                    let display_idx = (pos_y + pixel) as usize * 64 + (pos_x + sprite_bit) as usize;
                    if self.display[display_idx] != 0 {
                        self.display[display_idx] = 0;
                        self.v[0xF] = 1;
                        println!("Turning off pixel @ {display_idx}");
                    } else {
                        // each member of display signals a pixel to be toggled on or off.
                        // for the library in use, each pixel must be toggled "on"
                        // by using 0 or 255.
                        self.display[display_idx] = 0xFF;
                        println!("Drawing at {display_idx}");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_font_data() {
        let em = Chip8::new();

        let mut idx = 0x50;

        for _ in 0..80 {
            assert_ne!(em.memory[idx], 0);
            idx += 1;
        }
    }

    fn decrements_timer() {}
}
