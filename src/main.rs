use std::collections::HashMap;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

#[allow(unused_imports)]
use crate::emulator::Chip8;

mod emulator;

const DISPLAY_WIDTH: usize = 32;
const DISPLAY_HEIGHT: usize = 64;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

fn main() {
    println!("Hello goat");
    let mut em = Chip8::new();
    // em.load_roms(arg)

    let mut window = Window::new(
        "Chip-8",
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            topmost: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let handle = window.get_window_handle();
    println!("Window is open: {handle:#?}");

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        window
            .get_keys_pressed(KeyRepeat::No)
            .iter()
            .for_each(|key| em.handle_keypress(key));
        window.update();
    }
}

// listen for keypresses / keyboard scancodes
// translate the button presses into the fkn instruction
// Key  -> Translation
// 1234 -> 123C
// QWER -> 456D
// ASDF -> 789E
// AOBF -> ZXCV
// minifb will create a window and display a 32-bit pixel buffer
