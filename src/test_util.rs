use rand::{Rng, SeedableRng};
use std::iter::repeat;

pub fn random_strings(num: usize, max_repeat: u32, seed: [u8; 16]) -> Vec<String> {
    let mut rng = rand_xorshift::XorShiftRng::from_seed(seed);
    let gen = rand_regex::Regex::compile(r"[a-zA-Z]+", max_repeat).unwrap();
    (&mut rng)
        .sample_iter(&gen)
        .take(num)
        .collect::<Vec<String>>()
}

pub fn random_u32s(num: usize) -> Vec<u32> {
    let mut rng = rand_xorshift::XorShiftRng::from_seed(*b"seedseedseedseed");
    repeat(())
        .map(|_| (&mut rng).gen_range(0..1000))
        .take(num)
        .collect::<Vec<u32>>()
}
