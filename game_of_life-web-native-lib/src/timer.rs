#![allow(dead_code)]

#[cfg(not(debug_assertions))]
pub use dummy::*;
#[cfg(all(debug_assertions, not(target_arch = "wasm32")))]
pub use fallback::*;
#[cfg(all(debug_assertions, target_arch = "wasm32"))]
pub use wasm32::*;

mod dummy {
    use std::marker::PhantomData;

    pub struct Timer<'a> {
        _phantom: PhantomData<&'a ()>,
    }

    impl<'a> Timer<'a> {
        pub fn new(_: &'a str) -> Timer<'a> {
            Self {
                _phantom: PhantomData,
            }
        }
    }

    impl Drop for Timer<'_> {
        fn drop(&mut self) {}
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm32 {
    use web_sys::console;

    pub struct Timer<'a> {
        name: &'a str,
    }

    impl<'a> Timer<'a> {
        pub fn new(name: &'a str) -> Timer<'a> {
            console::time_with_label(name);

            Timer { name }
        }
    }

    impl Drop for Timer<'_> {
        fn drop(&mut self) {
            console::time_end_with_label(self.name);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod fallback {
    use std::time::Instant;

    pub struct Timer<'a> {
        instant: Instant,
        name: &'a str,
    }

    impl<'a> Timer<'a> {
        pub fn new(name: &'a str) -> Timer<'a> {
            println!("Timer: {}", name);

            Timer {
                instant: Instant::now(),
                name,
            }
        }
    }

    impl Drop for Timer<'_> {
        fn drop(&mut self) {
            println!("Timer: {} ({:#?})", self.name, self.instant.elapsed());
        }
    }
}
