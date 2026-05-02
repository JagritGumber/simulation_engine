# particle_system_rs

real-time particle simulation in Rust using macroquad.

## what it is

a GPU-accelerated particle system built with [macroquad](https://github.com/not-fl3/macroquad) for real-time rendering. demonstrates ownership, borrowing, and lifetime patterns in a game-loop context.

this was my first Rust project. macroquad was the library that actually let me draw something on screen without two weeks of compile cycles, which is why I shipped it (and later wrote about macroquad on X).

## run

```bash
cargo run --release
```

requires Rust toolchain. macroquad handles the rest (no platform-specific setup).

## what it shows

- particle spawning, lifecycle, decay
- mouse-driven emission point
- GPU-batched draw calls via macroquad
- frame-loop architecture in idiomatic Rust

## status

finished, archived as a learning artifact. not maintained.

## license

see LICENSE file.
