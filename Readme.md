## What is it? 

A very simple (but not super efficient) implementation of Conway's Game of Life in Rust, with periodic boundary conditions and random state flip.

## Requirememnts

* a Rust 2021 compiler (rustc version 1.56.0 or up is recommended)
* Cargo
* (optionally) a terminal emulator with a square font

## Build 

Run `cargo build --release`, or (if you have `make` and all the reauired Rust crates installed) `make`. 
The runnable executables will be found in `target/release`.

## Use

This crate comes with several example executables. Those named `collision`, `glider`, and `pulsar` are simple example running on the terminal (they work better with a square font). The executable `collision_w` is a simple example using a graphical interface. The executable `random` choses a starting configuration ar random and shows its evolution using the graphical interface. Finally, the main executable (`game_of_life`) starts from the configuration in `initial_state.csv` and shows its evolution using the graphical interface.

The last three one use the config file `Config.csv` to set the waiting time between frames, the pixel size, board dimension (if not specified by the initial state; this part of the config file is currently only used by `random`), colours, and robability of random flip per pixel and per frame.
