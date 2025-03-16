//! Comment out the "cdylib" crate type and any #[wasm_bindgen] usages
//! prior to running the benchmark.

#![feature(test)]
extern crate test;

#[cfg(test)]
mod benches {
    use game_of_life_web_native_lib::universe::Universe;
    use test::{black_box, Bencher};

    #[bench]
    fn universe_creation(b: &mut Bencher) {
        b.iter(|| black_box(Universe::new(256, 256)));
    }

    #[bench]
    fn universe_ticks(b: &mut Bencher) {
        let mut universe = Universe::new(256, 256);

        b.iter(|| universe.tick());
    }
}
