use crate::emulator::Chip8;
use crate::emulator::KeyMapValue;
use crate::emulator::TestRoms;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{thread, time::Duration};

mod emulator;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const EXECUTION_DELAY: u64 = 1430;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

// https://www.reddit.com/r/bashonubuntuonwindows/comments/17jpeu4/getting_audio_through_wslg_from_alsaportaudio/

fn main() {
    println!("Hello goat");
    let mut em = Chip8::new();
    let mut rom_path = std::env::current_dir().unwrap();
    rom_path.push("roms");
    rom_path.push("tests");
    rom_path.push(TestRoms::new().flags);

    em.load_rom(rom_path);

    let mut window = Window::new(
        "Chip-8",
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    while !window.is_open() {
        thread::sleep(Duration::from_micros(10));
        println!("Waiting for window to open...");
    }

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        em.keys.iter_mut().for_each(|(key, data)| {
            *data = KeyMapValue(window.is_key_pressed(*key, KeyRepeat::No), data.1);
        });
        em.run();
        window
            .update_with_buffer(em.get_display(), DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .unwrap();
        thread::sleep(Duration::from_micros(EXECUTION_DELAY)); // ~700 Hz
    }
}
