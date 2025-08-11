use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween,
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
};
use minifb::Key;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::DISPLAY_SIZE;

const FONT_STARTING_ADDR: usize = 0x50;

// memeory available to the emulator
const MEMORY_SIZE: usize = 4096;

// Programs start at address 200
const PROGRAM_STARTING_ADDR: usize = 0x200;

pub struct Beep {
    manager: AudioManager,
    sound_data: StaticSoundData,
    beep_sound: Option<StaticSoundHandle>,
}

impl Beep {
    pub fn new() -> Self {
        let mut beep_path = std::env::current_dir().unwrap();
        beep_path.push("src");
        beep_path.push("beep.mp3");

        Self {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .expect("Could not create audio manager."),
            sound_data: StaticSoundData::from_file(beep_path)
                .expect("Could not decode sound data."),
            beep_sound: None,
        }
    }

    pub fn play(&mut self) {
        if let Some(sound) = self.beep_sound.as_mut() {
            sound.seek_to(0.0);
            sound.resume(Tween::default());
        } else {
            let sound = self.manager.play(self.sound_data.clone()).unwrap();
            self.beep_sound = Some(sound);
        }
    }

    pub fn stop(&mut self) {
        if let Some(sound) = self.beep_sound.as_mut() {
            sound.pause(Tween::default());
        }
    }
}

/// Represents the value in the keys store
///
/// - `0`: Whether the key is pressed (bool).
/// - `1`: The corresponding Chip8 Key (&'static str).
#[derive(Debug)]
pub struct KeyMapValue(pub bool, pub u8);

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    display: [u32; DISPLAY_SIZE],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    pub keys: HashMap<Key, KeyMapValue>,
    pc: usize,
    v: [u8; 16],
    i: u16,
    beep: Beep,
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
            beep: Beep::new(),
        };
        em.load_font();
        em.set_keys();
        em
    }

    fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn run(&mut self) {
        let next_opcode = self.fetch_next_opcode();
        self.decode(next_opcode);
        self.decrement_timers();
    }

    pub fn load_rom(&mut self, path: PathBuf) {
        println!("Loading rom {:?} into memory...", path.file_name().unwrap());

        let rom_buf = fs::read(path).unwrap_or_else(|e| {
            panic!("Couldn't read rom: {e}");
        });

        let mut ptr = PROGRAM_STARTING_ADDR;
        for byte in &rom_buf {
            self.memory[ptr] = *byte;
            ptr += 1;
        }

        println!("Done.");
    }

    fn set_keys(&mut self) {
        self.keys.insert(Key::Key1, KeyMapValue(false, 0x1));
        self.keys.insert(Key::Key2, KeyMapValue(false, 0x2));
        self.keys.insert(Key::Key3, KeyMapValue(false, 0x3));
        self.keys.insert(Key::Key4, KeyMapValue(false, 0xC));
        self.keys.insert(Key::Q, KeyMapValue(false, 0x4));
        self.keys.insert(Key::W, KeyMapValue(false, 0x5));
        self.keys.insert(Key::E, KeyMapValue(false, 0x6));
        self.keys.insert(Key::R, KeyMapValue(false, 0xD));
        self.keys.insert(Key::A, KeyMapValue(false, 0x7));
        self.keys.insert(Key::S, KeyMapValue(false, 0x8));
        self.keys.insert(Key::D, KeyMapValue(false, 0x9));
        self.keys.insert(Key::F, KeyMapValue(false, 0xE));
        self.keys.insert(Key::A, KeyMapValue(false, 0xA));
        self.keys.insert(Key::O, KeyMapValue(false, 0x0));
        self.keys.insert(Key::B, KeyMapValue(false, 0xB));
        self.keys.insert(Key::F, KeyMapValue(false, 0xF));
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
        let mut idx = FONT_STARTING_ADDR;
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
        let x = ((opcode & 0x0F00) >> 8) as usize;
        // third nibble
        let y = ((opcode & 0x00F0) >> 4) as usize;
        // fourth nibble
        let n = (opcode & 0xF) as u8;
        // 3rd & 4th nibbles
        let nn = (opcode & 0x00FF) as u8;
        // 2nd 3rd & 4th nibbles
        let nnn = opcode & 0x0FFF;

        match high_nibble {
            0x0 => match opcode {
                0x00E0 => self.op_00e0(),
                0x00EE => self.op_00ee(),
                _ => self.pc += 2, // 0NNN, ignore this instruction
            },
            0x1 => self.op_1nnn(nnn),
            0x2 => self.op_2nnn(nnn),
            0x3 => self.op_3xnn(x, nn),
            0x4 => self.op_4xnn(x, nn),
            0x5 => self.op_5xy0(x, y),
            0x6 => self.op_6xnn(x, nn),
            0x7 => self.op_7xnn(x, nn),
            0x8 => match n {
                0x0 => self.op_8xy0(x, y),
                0x1 => self.op_8xy1(x, y),
                0x2 => self.op_8xy2(x, y),
                0x3 => self.op_8xy3(x, y),
                0x4 => self.op_8xy4(x, y),
                0x5 => self.op_8xy5(x, y),
                0x6 => self.op_8xy6(x, y),
                0x7 => self.op_8xy7(x, y),
                0xE => self.op_8xye(x, y),
                _ => {
                    self.op_unknown(opcode);
                }
            },
            0x9 => self.op_9xy0(x, y),
            0xA => self.op_annn(nnn),
            0xB => self.op_bnnn(nnn),
            0xC => self.op_cxnn(nn, x),
            0xD => self.op_dxyn(n, x, y),
            0xE => match n {
                0xE => self.op_ex9e(x),
                0x1 => self.op_exa1(x),
                _ => self.op_unknown(opcode),
            },
            0xF => match nn {
                0x07 => self.op_fx07(x),
                0x0a => self.op_fx0a(),
                0x15 => self.op_fx15(x),
                0x18 => self.op_fx18(x),
                0x1e => self.op_fx1e(x),
                0x29 => self.op_fx29(x),
                0x33 => self.op_fx33(x),
                0x55 => self.op_fx55(x),
                0x65 => self.op_fx65(x),
                _ => self.op_unknown(opcode),
            },
            _ => {
                // satisfies u16::MIN & u16::MAX
                self.op_unknown(opcode);
            }
        };
    }

    /// Clear the display - turn all pixels off to 0 (false)
    fn op_00e0(&mut self) {
        self.display.iter_mut().for_each(|pixel| *pixel = 0);
    }

    // Computer specific instruction - not needed
    // fn op_0nnn() {}

    /// Sets PC to ```nnn```,
    /// Does not increment the PC afterwards.
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn as usize;
    }

    /// Return from subroutine by setting pc to popped stack address
    fn op_00ee(&mut self) {
        let last_instruction = self.stack.pop().unwrap();
        self.pc = last_instruction as usize;
    }

    /// Saves current pc to stack before setting pc to ```nnn```
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.pc as u16);
        self.pc = nnn as usize;
    }

    /// Skips one instruction if V```x``` is equal to ```nn```
    fn op_3xnn(&mut self, x: usize, nn: u8) {
        if self.v[x] == nn {
            self.pc += 2;
        }
    }

    /// Skips one instruction if V```x``` is not equal to ```nn```
    fn op_4xnn(&mut self, x: usize, nn: u8) {
        if self.v[x] != nn {
            self.pc += 2;
        }
    }

    /// Skips one instruction if
    /// values in V```x``` and V```y``` are equal
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    /// Looks up register ```x``` and sets its value to ```nn```
    fn op_6xnn(&mut self, x: usize, nn: u8) {
        self.v[x] = nn;
    }

    /// Add the value ```nn``` to VX.
    fn op_7xnn(&mut self, x: usize, nn: u8) {
        let result = self.v[x].overflowing_add(nn);
        self.v[x] = result.0;
    }

    /// Sets V```x``` to value of V```y```
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    /// Sets V```x``` to bitwise OR of V```x``` and V```y```
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    /// Sets V```x``` to bitwise AND of V```x``` and V```y```
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    /// Sets V```x``` to bitwise XOR of V```x``` and V```y```
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    /// Sets V```x``` to the sum of V```x``` and V```y```
    /// VF is set to 1 if the addition would overflow, 0 otherwise.
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let result = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = result.0;

        // assign it directly because I'm lazy
        self.v[0xF] = result.1 as u8;
    }

    /// sets V```x``` to the difference of V```x``` and V```y```.
    /// Sets VF to 0 if the subtraction would overflow, 1 otherwise.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        let result = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = result.0;

        self.v[0xF] = if result.1 { 0 } else { 1 }
    }

    /// Stores V```y``` into V```x```, right shifts V```x```,
    /// and sets VF to the LSB that is shifted out.
    /// Uses COSMAC VIP implementation.
    fn op_8xy6(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];

        // Figure out if bit to be shifted out is set, set vF to that value
        let lsb = 1 & self.v[x];
        self.v[x] >>= 1;
        self.v[0xF] = lsb;
    }

    /// sets V```x``` to difference V```y``` and V```x```.
    /// Sets VF to 0 if the subtraction would overflow, 1 otherwise.
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let result = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = result.0;

        self.v[0xF] = if result.1 { 0 } else { 1 };
    }

    /// Stores V```y``` into V```x```, left shifts V```x```.
    /// Sets VF to the MSB that was shifted out.
    /// Uses COSMAC VIP implementation.
    fn op_8xye(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];

        let msb = (0b1000_0000 & self.v[x]) >> 7;
        self.v[x] <<= 1;
        self.v[0xF] = msb;
    }

    /// Skips one instruction if
    /// V```x``` and V```y``` are not equal.
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    /// Sets the index register I to ```nnn```
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /// Jumps to address ```nnn``` plus the value in register V[0].
    /// Uses COSMAC VIP implementation.
    fn op_bnnn(&mut self, nnn: u16) {
        let jump_destination = nnn + self.v[0] as u16;
        self.op_1nnn(jump_destination);
    }

    /// Generates a random number, binary ANDs it with ```nn```,
    /// and puts the result in V```x```.
    fn op_cxnn(&mut self, nn: u8, x: usize) {
        let num = rand::random::<u8>();
        self.v[x] = num & nn;
    }

    ///
    ///
    /// ## Arguments
    ///
    /// * `n` - Number of pixels in a sprite
    /// * `x` - Coordinate x from V[```x```] register
    /// * `y` - Coordinate y from V[```y```] register
    fn op_dxyn(&mut self, n: u8, x: usize, y: usize) {
        // using literals instead of DISPLAY constants
        // to minimize casting
        let pos_x = self.v[x] % 64;
        let pos_y = self.v[y] % 32;
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

                // which bits in this pixel row are set? Turn off the ones that are
                // and turn on the ones that aren't.
                if pixel_row & (mask >> sprite_bit) != 0 {
                    // get row and column offsets, multiply to get actual position
                    let display_idx = (pos_y + pixel) as usize * 64 + (pos_x + sprite_bit) as usize;
                    if self.display[display_idx] != 0 {
                        self.display[display_idx] = 0;
                        self.v[0xF] = 1;
                    } else {
                        // each member of display signals a pixel to be toggled on or off.
                        // for the library in use, each pixel must be toggled "on"
                        // by using 0 or 255.
                        self.display[display_idx] = 0xFF;
                    }
                }
            }
        }
    }

    /// Skips one instruction if key in value V```x``` is pressed.
    /// checks if key is currently being held.
    fn op_ex9e(&mut self, x: usize) {
        let stored_key = self.v[x];
        self.keys.iter_mut().for_each(|(_, data)| {
            if stored_key == data.1 && data.0 {
                self.pc += 2;
            }
        });
    }

    /// Skips one instruction if key in value V```x``` is not pressed.
    /// Executes if it's currently being held.
    fn op_exa1(&mut self, x: usize) {
        let stored_key = self.v[x];
        self.keys.iter_mut().for_each(|(_, data)| {
            if stored_key == data.1 && !data.0 {
                self.pc += 2;
            }
        });
    }

    /// sets V```x``` to current delay timer value.
    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    /// sets delay timer to the value in V```x```.
    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    /// sets the sound timer to the value in V```x```.
    /// Beeps if the sound timer is still above 0.
    fn op_fx18(&mut self, x: usize) {
        self.sound_timer = self.v[x];
        if self.sound_timer > 0 {
            self.beep.play()
        } else {
            self.beep.stop()
        }
    }

    /// Adds value of V```x``` to index register.
    fn op_fx1e(&mut self, x: usize) {
        self.i += (self.v[x]) as u16;
    }

    /// Blocks until a key input is received.
    fn op_fx0a(&mut self) {
        if self.keys.iter().any(|(_, data)| data.0) {
            self.pc -= 2;
        }
    }

    /// Sets index register to the address of hexadecimal character in V```x```.
    fn op_fx29(&mut self, x: usize) {
        // isolate lower nibble to get needed char
        let num = 0xf & self.v[x];

        // multiply by 5 since each char is 5 bytes apart to get offset & font's starting idx
        self.i = self.memory[FONT_STARTING_ADDR + (5 * num as usize)] as u16;
    }

    /// Convert value in V```x``` to three decimal digits
    /// and store them in memory at address in index register i.
    fn op_fx33(&mut self, x: usize) {
        // since any given number in v is u8 (<= 255), we only need to modulo 3 times
        let mut num = self.v[x];
        let address = self.i;

        // 156 -> 1 in i, 5 in i + 1, 6 in i + 2
        // num will be truncated toward zero
        self.memory[(address + 2) as usize] = num % 10;
        num /= 10;

        self.memory[(address + 1) as usize] = num % 10;
        num /= 10;

        self.memory[address as usize] = num % 10;
    }

    /// Reads values in V registers and stores them in
    /// successive memory addresses starting from i
    fn op_fx55(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.memory[self.i as usize + i] = self.v[i];
        }
    }

    /// Takes values stored successively in memory
    /// starting from i and then loads them
    /// into V registers
    fn op_fx65(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.v[i] = self.memory[self.i as usize + i];
        }
    }

    fn op_unknown(&self, opcode: u16) {
        eprintln!("Received unknown opcode! {opcode:X?}");
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
}
