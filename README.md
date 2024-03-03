# IAM.mml

## What is this?

Online AM/FM Synthesizer for Generating and Playing WAV from MML.

## Build

The main program is written in Rust and operates on WASM.
The following command is used for building WASM.

```
rustc --crate-type=cdylib --target=wasm32-unknown-unknown -o ./pages/iam-mml.wasm ./wasm/lib.rs
```
