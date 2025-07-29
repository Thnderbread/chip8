#![allow(dead_code)]

use crate::DISPLAY_SIZE;

const MEMORY_SIZE: usize = 4096;
// 64x32 display area

pub struct Chip8 {
    ram: [u8; MEMORY_SIZE],
    display: Display,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut em = Self {
            ram: [0; MEMORY_SIZE],
            display: Display::new(),
            // Emulate original space limitation (16 2-byte entries)
            stack: Vec::with_capacity(16),
            delay_timer: 0,
            sound_timer: 0,
        };
        em.load_font();
        em
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1
        }
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
            self.ram[idx] = *char;
            idx += 1;
        }
    }
}

struct Display {
    pixels: [bool; DISPLAY_SIZE],
}

impl Display {
    fn new() -> Self {
        Self {
            pixels: [false; DISPLAY_SIZE],
        }
    }

    fn update_display(&mut self) {
        todo!();
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
            assert_ne!(em.ram[idx], 0);
            idx += 1;
        }
    }

    fn decrements_timer() {}
}
