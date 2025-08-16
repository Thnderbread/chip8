use crate::{emulator::Chip8, roms::romfiles::get_desired_rom};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{thread, time::Duration};

mod emulator;
mod roms;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT;

fn main() {
    let mut em = Chip8::new();
    let rom_path = get_desired_rom();
    em.load_rom(rom_path);

    let mut window = Window::new(
        "Chip-8",
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X8,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    while window.is_open() && !window.is_key_pressed(Key::Escape, KeyRepeat::No) {
        em.decrement_timers();

        for _ in 0..12 {
            em.keys
                .iter_mut()
                .for_each(|(key, data)| data.pressed = window.is_key_down(*key));
            em.run();
        }

        thread::sleep(Duration::from_millis(16));
        if em.update_display {
            window
                .update_with_buffer(em.get_display(), DISPLAY_WIDTH, DISPLAY_HEIGHT)
                .unwrap();
            em.update_display = false;
        } else {
            window.update();
        }
    }
}
