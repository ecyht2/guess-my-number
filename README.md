# Guess My Number
A number guessing game written in Rust.

The C++ implementation of the library can be found [here](https://github.com/ecyht2/EEEE2065-cw2).

## Installation

### Dependency
The installation tutorial assumes that you have [rust](http://www.rust-lang.org) and it's package manager [cargo](https://doc.rust-lang.org/cargo/) installed on your working computer. If they aren't installed on your computer, you can follow the installation instructions on [rust website](https://www.rust-lang.org/tools/install). Cargo should be installed when you installed rust. Howerver, you can compiled it from source by following the instructions on [cargo documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Compiling Executable

This crate provides an executable binary . 

``` sh
git clone https://github.com/ecyht2/guess-my-number-rs.git
cd guess-my-number-rs
cargo build --release
```

The compiled executable would be located in `target/release/guess-my-number-rs`

## Usage

The executable takes arguments to customize the game.

Using `cargo run`

``` sh
git clone https://github.com/ecyht2/guess-my-number-rs.git
cd guess-my-number-rs
cargo run --release -- -i
```

or if it is already compiled

``` sh
target/release/guess-my-number-rs --human
```
