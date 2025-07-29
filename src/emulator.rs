#![allow(dead_code)]

use std::collections::HashMap;

use minifb::{Key, Window, WindowOptions};

use crate::DISPLAY_SIZE;

const MEMORY_SIZE: usize = 4096;
// 64x32 display area

pub struct Chip8 {
    ram: [u8; MEMORY_SIZE],
    display: Display,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    keys: HashMap<Key, &'static str>,
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
            keys: HashMap::with_capacity(16),
        };
        em.load_font();
        em.set_keys();
        em
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

    // instruction is 2 bytes, so read 2 successive bytes
    // and combine them into a 16 bit instruction
    fn fetch() {
        // read instruction that pc is pointing at from memory

        // should read 2 successive bytes, combine them into
        // 16 bit instruction

        // increment PC by 2 to fetch next opcode
        todo!()
    }
}

struct Display {
    pixels: [bool; DISPLAY_SIZE],
}

// Draw a window
// window should be able to capture input

// Chip8 inits the window as a member
// member handles drawing and clearing
// emulator.display.draw(x)
// emulator.display.clear()

fn fb() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // Initialize with black pixels

    let mut window = Window::new(
        "minifb Example - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit update rate to 60fps
    window.set_target_fps(16600);

    // Drawing a simple pattern:
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let index = i + j * WIDTH;
            // Example: Gradient from top-left (black) to bottom-right (white)
            let r = (i as f32 / WIDTH as f32 * 255.0) as u32;
            let g = (j as f32 / HEIGHT as f32 * 255.0) as u32;
            let b = ((i + j) as f32 / (WIDTH + HEIGHT) as f32 * 255.0) as u32;
            buffer[index] = (0xFF << 24) | (r << 16) | (g << 8) | b; // ARGB format
        }
    }

    // Main loop to keep the window open and update
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the window with the buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
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
