#![allow(clippy::new_without_default)]

#[macro_use]
extern crate tracing;

mod error;
pub mod timer;
pub mod universe;

pub use error::{Error, Result};
use wasm_bindgen::prelude::*;

// NOTE: Only two states can exist, since in `./universe.rs` a
// transmution from `bool` to `Self` takes place.
//
// For the same reason, only #[repr(u8)] can be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
#[repr(u8)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    Ok(())
}
