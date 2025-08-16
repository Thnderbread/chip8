#![allow(dead_code)]

pub mod romfiles {
    use std::{env, fs, path::PathBuf, process::exit};

    /// Timendus' test suite for the Chip8 Emulator.
    /// https://github.com/Timendus/chip8-test-suite/tree/main
    struct TestRomFilePaths {
        /// Simple splash screen.
        chip8_logo: PathBuf,
        /// Classic IBM ROM.
        ibm_logo: PathBuf,
        /// Tests various opcodes.
        corax: PathBuf,
        /// Tests correctness of math operations, and checks
        /// correctness of vF flag register when running those
        /// opcodes.
        flags: PathBuf,
        /// Allows testing all 3 key CHIP-8 input opcodes.
        keypad: PathBuf,
        /// Tests if the buzzer is working.
        beep: PathBuf,
        /// Tests fundamental functions of the emulator.
        /// https://github.com/cj1128/chip8-emulator/tree/master
        test_opcode: PathBuf,
        /// Tests various codes.
        /// https://github.com/metteo/chip8-test-rom
        metteo_test: PathBuf,
        /// Tests the quirks for our emulator
        quirks: PathBuf,
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
                metteo_test: filepaths[7].clone(),
                quirks: filepaths[8].clone(),
            }
        }
    }

    /// Some Game roms to run on the emulator.
    /// https://github.com/cj1128/chip8-emulator/tree/master/rom
    struct GameRomFilePaths {
        blinky: PathBuf,
        cave: PathBuf,
        maze: PathBuf,
        pong: PathBuf,
        tetris: PathBuf,
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

    /// Reads desired rom from program arguments and returns its filepath
    pub fn get_desired_rom() -> PathBuf {
        let valid_roms = [
            "chip8_logo",
            "ibm_logo",
            "corax",
            "flags",
            "keypad",
            "beep",
            "test_opcode",
            "metteo_test",
            "blinky",
            "cave",
            "maze",
            "pong",
            "tetris",
        ];

        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!("Please choose a rom to run. Valid roms are:");
            valid_roms.iter().for_each(|r| eprintln!("{r}"));
            eprintln!("E.g. chip8 test_opcode");
            exit(1);
        }

        let requested_rom = &args[1];
        let tests = TestRomFilePaths::new();
        let games = GameRomFilePaths::new();

        match requested_rom.to_lowercase().as_str() {
            "chip8_logo" => tests.chip8_logo,
            "ibm_logo" => tests.ibm_logo,
            "corax" => tests.corax,
            "flags" => tests.flags,
            "keypad" => tests.keypad,
            "beep" => tests.beep,
            "test_opcode" => tests.test_opcode,
            "metteo_test" => tests.metteo_test,
            "quirks" => tests.quirks,
            "blinky" => games.blinky,
            "cave" => games.cave,
            "maze" => games.maze,
            "pong" => games.pong,
            "tetris" => games.tetris,
            _ => {
                eprintln!("Invalid entry '{requested_rom}'. Valid roms are: {valid_roms:?}");
                exit(1);
            }
        }
    }
}
