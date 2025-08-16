use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};

pub fn get_desired_rom() -> PathBuf {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Supply a path to the rom you would like to run. e.g.: chip8 <path_to_rom>");
        exit(1);
    }

    Path::new(&args[1]).to_path_buf()
}
