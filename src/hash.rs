//! Copy of hash functions from https://github.com/quick-lint/quick-lint-js
//! The hash takes advantage of the fact that the first two and last two characters of
//! javascript keywords uniquely identifies them and uses it to produce an efficient
//! hash function.

pub fn select(key: &str) -> u32 {
    let bytes = key.as_bytes();
    let len = key.len();
    // assert len >= 2;
    fn read_16_bits(buf: &[u8]) -> u32 {
        (buf[0] as u32) | (buf[1] as u32) << 8
    }
    // Take the first 2 characters.
    let lo = read_16_bits(bytes);
    // Take the last 2 characters.
    let hi = read_16_bits(&bytes[len - 2..]);
    hi << 16 | lo
}

pub fn mix(selection: u32, seed: u64) -> u32 {
    /// A very fast hash function but generates a lot of collisions.
    /// We will try at build time different seed until we find one without
    /// collisions.
    const MAGIC: u64 = 4292484099903637661;
    let x = (selection as u64) ^ seed;
    let res = multiply_u64_get_top_64(x, MAGIC);
    res as u32
}

fn multiply_u64_get_top_64(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) >> 64) as u64
}
