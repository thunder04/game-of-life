#![feature(test)]

extern crate test;

#[cfg(test)]
mod benches {
    use game_of_life_web_native_lib::universe::Universe;
    use test::Bencher;

    #[bench]
    fn universe_ticks(b: &mut Bencher) {
        let mut universe = Universe::new(128, 128);

        b.iter(|| universe.tick());
    }
}
