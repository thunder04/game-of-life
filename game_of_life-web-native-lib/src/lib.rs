#[macro_use]
extern crate tracing;

mod error;
mod utils;

pub use error::{Error, Result};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    debug!("Called greet function");
    alert("Hello, stranger!");
}
