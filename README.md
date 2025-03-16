# Rust Application Template

My template for creating a basic Rust application.

It features logging using the [`tracing`](https://crates.io/crates/tracing) library,
[`color_eyre`](https://crates.io/crates/color_eyre) for pretty panic reports,
and [`thiserror`](https://crates.io/crates/thiserror) for specialized error handling.

## Usage

To use it, install [`cargo-generate`](https://cargo-generate.github.io/cargo-generate/installation.html)
and run:

```sh
cargo generate --git https://github.com/thunder04/t-rust-app
```

## Folder Structure

- `game_of_life`: The back-end part of the application.
- `game_of_life-lib`: The library for the back-end.
- `game_of_life-web`: The front-end part of the application.
- `game_of_life-web-lib`: The library for the front-end. *Note: It doesn't actually exist.*
- `game_of_life-web-native-lib`: The native (Rust + WASM) library for the front-end.
