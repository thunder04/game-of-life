#[macro_use]
extern crate tracing;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

pub fn add(a: i32, b: i32) -> i32 {
    debug!("{} + {} = {}", a, b, a + b);

    a + b
}
