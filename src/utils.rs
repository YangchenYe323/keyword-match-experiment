use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::ELEMENTS;

pub fn generate_keywords(n: usize) -> Vec<Vec<u8>> {
    use rand::RngCore;
    let mut vec = vec![vec![0; ELEMENTS]; n];
    for i in 0..n {
        let byte = &mut vec[i];
        rand::thread_rng().fill_bytes(byte);
    }

    vec
}

pub fn read_keys_from_file(file: impl AsRef<Path>) -> Vec<Vec<u8>> {
    let file = File::open(file.as_ref()).unwrap();
    let mut r = BufReader::new(file);
    let mut buf = vec![];
    r.read_to_end(&mut buf).unwrap();
    buf.split(|b| b.is_ascii_whitespace())
        .map(|slice| slice.to_vec())
        .collect()
}
