# Intro

As my first foray into emulation and Rust, I wrote a Chip8 interpreter.

## Usage

Rust is required to run the emulator. You can download it [from the Rust website](https://www.rust-lang.org/tools/install).

Next, clone the repo:

```code
git clone https://github.com/thnderbread/chip8/
```

If you don't have git, you can just [navigate to the repo](https://github.com/thnderbread/chip8/), click the Code dropdown > download ZIP and extract it.

Finally, run the emulator like so:

```code
cargo run <path-to-emulator>
```

For example:

```code
cargo run ./roms/games/pong
```

## Known Issues

- There is an issue with the sound timer and boundary detection. It seems to trigger a sound a bit earlier than it should - this is most noticeable in a game like Pong, where a beep will play any time the ball collides with a paddle.
  - The beep can safely be disabled by moving or deleting the beep.mp3 file located in /src.

## Resources

Followed a great guide from [Tobias](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
Test Roms sourced from [Timendus](https://github.com/Timendus/chip8-test-suite)
