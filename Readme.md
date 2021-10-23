## What is it? 

A very simple (but not super efficient) implementation of Conway's Game of Life in Rust, with periodic boundary conditions.

## Requirememnts

* a Rust 2018 compiler (rustc version 1.51.0 or up is recommended)
* Cargo
* (optionally) a terminal emulator with a square font

## Build 

Run `cargo build --release`, or (if you have `make` and all the reauired Rust crates installed) `make`. 
the runnable executable will be found in `target/release`.
