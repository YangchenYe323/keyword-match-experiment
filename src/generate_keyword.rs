use std::cmp::Reverse;

use rand::RngCore;

use crate::keyword_list::*;

#[derive(Debug)]
pub struct KeywordHashTable {
    gen: i32, // Current generation
    seed: u64,
    // todo: put Syntax Kind here.
    entries: Vec<(&'static str, i32)>,
}

#[derive(Debug)]
pub enum Entry {
    Keyword(&'static str),
    None,
}

impl KeywordHashTable {
    pub fn new(size: usize) -> Self {
        Self {
            gen: 0,
            seed: rand::thread_rng().next_u64(), /*Random Start */
            entries: vec![("", -1); size],
        }
    }

    pub fn try_fill(&mut self, keys: &[&'static str]) -> bool {
        self.gen += 1;
        self.seed += 1;
        for &key in keys {
            let slice = key.as_bytes();
            let selection = unsafe { crate::hash::select(slice) };
            let hash = crate::hash::mix(selection, self.seed);
            let idx = hash as usize % self.entries.len();
            let old_entry = self.entries[idx];
            if old_entry.1 < self.gen {
                // no collision, take this slot
                self.entries[idx] = (key, self.gen);
            } else {
                // collision, try another seed maybe
                return false;
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn into_entries(self) -> impl Iterator<Item = Entry> {
        let target_gen = self.gen;
        self.entries.into_iter().map(move |(s, gen)| {
            if gen == target_gen {
                Entry::Keyword(s)
            } else {
                Entry::None
            }
        })
    }
}

/// All the bytes of the keywords are packed compactly with paddings in the end.
#[derive(Debug)]
pub struct KeywordStringTable {
    pub bytes: Vec<u8>,
    pub offsets: Vec<usize>,
}

impl KeywordStringTable {
    pub fn new() -> Self {
        Self {
            bytes: vec![],
            offsets: vec![],
        }
    }

    pub fn find_word_start(&self, key: &str) -> Option<usize> {
        let len = key.len();
        self.bytes
            .windows(len)
            .position(|slice| slice == key.as_bytes())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn offsets(&self) -> &[usize] {
        &self.offsets
    }
}

fn make_hash_table(keys: &[&'static str]) -> KeywordHashTable {
    const ATTEMPTS_PER_TABLE_SIZE: i32 = 50_000;
    const MAX_TABLE_SIZE: u32 = 1024;

    let mut table_size = bit_ceil(keys.len() as u32);

    loop {
        let mut hash_table = KeywordHashTable::new(table_size as usize);
        for _ in 0..ATTEMPTS_PER_TABLE_SIZE {
            if hash_table.try_fill(keys) {
                return hash_table;
            }
            // try a different seed
        }

        // increase table size and retry
        table_size *= 2;
        if table_size > MAX_TABLE_SIZE {
            panic!("Hash Table Generation Failed");
        }
    }
}

fn make_string_table(keys: &[&'static str]) -> KeywordStringTable {
    let mut string_table = KeywordStringTable::new();
    for key in keys {
        let slice = key.as_bytes();
        let idx = string_table
            .bytes
            .windows(slice.len())
            .position(|s| s == slice);
        // Only insert new string patterns to save some space
        if idx.is_none() {
            string_table.offsets.push(string_table.bytes.len());
            string_table.bytes.extend_from_slice(slice);
        }
    }

    // final padding of 17 bytes.
    string_table.offsets.push(string_table.bytes.len());
    string_table.bytes.resize(string_table.bytes.len() + 17, 0);

    string_table
}

fn bit_ceil(num: u32) -> u32 {
    if num & (num - 1) == 0 {
        return num;
    }
    let width = 32;
    let lzs = num.leading_zeros();
    let shift = width - lzs;
    1 << shift
}

pub fn generate_tables() -> (KeywordHashTable, KeywordStringTable) {
    let mut keys = Vec::from_iter(KEYWORDS.iter().copied());
    // Sorts in reverse order to enable merging of common prefixes, like for
    // "as", "assert", and "asserts".
    keys.sort_unstable_by_key(|s| Reverse(*s));
    let hash_table = make_hash_table(&keys);
    let string_table = make_string_table(&keys);
    (hash_table, string_table)
}

#[cfg(test)]
#[test]
fn it_works() {
    let (_, _) = generate_tables();
}
