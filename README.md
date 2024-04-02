# Common Library for Rust

## Usage
 - `Cargo.toml`
```
[dependencies]
common-library = { git = "https://github.com/common-library/rust.git", branch = "main" }
```
<br/>

## Features
 - file
 - socket

<br/>

## Document
 - `cargo doc`
 - run `./target/doc/common_library/index.html` in your browser

<br/>

## Build
 - `cargo build`

<br/>

## Test
 - `cargo test`

<br/>

## Coverage
 - `cargo install grcov`
 - `rustup component add llvm-tools`
 - `CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='./target/coverage/test.profraw' cargo test`
 - `grcov . --binary-path ./target/debug/deps/ --source-dir . --output-types html --output-path ./target/coverage/`
 - run `./target/coverage/html/index.html` in your browser
