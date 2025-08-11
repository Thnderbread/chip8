#![allow(dead_code)]

pub mod romfiles {
    use std::{fs, path::PathBuf};

    /// Timendus' test suite for the Chip8 Emulator. Uses a few other tests.
    /// https://github.com/Timendus/chip8-test-suite/tree/main
    pub struct TestRomFilePaths {
        /// Simple splash screen.
        pub chip8_logo: PathBuf,
        /// Classic IBM ROM.
        pub ibm_logo: PathBuf,
        /// Tests various opcodes.
        pub corax: PathBuf,
        /// Tests correctness of math operations, and checks
        /// correctness of vF flag register when running those
        /// opcodes.
        pub flags: PathBuf,
        /// Allows testing all 3 key CHIP-8 input opcodes.
        pub keypad: PathBuf,
        /// Tests if the buzzer is working.
        pub beep: PathBuf,
        /// Tests fundamental functions of the emulator.
        /// https://github.com/cj1128/chip8-emulator/tree/master
        pub test_opcode: PathBuf,
    }

    impl TestRomFilePaths {
        pub fn new() -> TestRomFilePaths {
            let mut roms_dir = std::env::current_dir().unwrap();
            roms_dir.push("roms");
            roms_dir.push("tests");

            let mut filepaths = Vec::new();

            for path in fs::read_dir(roms_dir).unwrap() {
                let file = path.unwrap();
                filepaths.push(file.path());
            }

            Self {
                chip8_logo: filepaths[0].clone(),
                ibm_logo: filepaths[1].clone(),
                corax: filepaths[2].clone(),
                flags: filepaths[3].clone(),
                keypad: filepaths[4].clone(),
                beep: filepaths[5].clone(),
                test_opcode: filepaths[6].clone(),
            }
        }
    }

    /// Some Game roms to run on the emulator.
    /// https://github.com/cj1128/chip8-emulator/tree/master/rom
    pub struct GameRomFilePaths {
        pub blinky: PathBuf,
        pub cave: PathBuf,
        pub maze: PathBuf,
        pub pong: PathBuf,
        pub tetris: PathBuf,
    }

    impl GameRomFilePaths {
        pub fn new() -> GameRomFilePaths {
            let mut games_dir = std::env::current_dir().unwrap();
            games_dir.push("roms");
            games_dir.push("games");

            let mut filepaths = Vec::new();

            for path in fs::read_dir(games_dir).unwrap() {
                let file = path.unwrap();
                filepaths.push(file.path());
            }

            Self {
                blinky: filepaths[0].clone(),
                cave: filepaths[1].clone(),
                maze: filepaths[2].clone(),
                pong: filepaths[3].clone(),
                tetris: filepaths[4].clone(),
            }
        }
    }
}
