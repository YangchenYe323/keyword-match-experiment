use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};

use keyword_match_experiment::generate_keyword::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct KeywordTableEntry(u32, u32);

fn generate_keyword_table_entries(
    hash_table: KeywordHashTable,
    string_table: &KeywordStringTable,
) -> Vec<KeywordTableEntry> {
    hash_table
        .into_entries()
        .map(|entry| match entry {
            Entry::Keyword(key) => {
                let key_start = string_table.find_word_start(key).unwrap();
                KeywordTableEntry(key_start as u32, key.len() as u32)
            }
            Entry::None => KeywordTableEntry(0, 0),
        })
        .collect()
}

/// Usage: cargo run --bin generator -- ./src/keyword_generated.rs
fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let dir: PathBuf = args.next().unwrap().into();

    let (hash_table, string_table) = generate_tables();
    let hash_table_size = hash_table.len();
    let hash_table_seed = hash_table.seed();
    let keyword_bytes = string_table.bytes();

    let file = std::fs::File::create(dir.as_path()).unwrap();
    let mut w = BufWriter::new(file);
    write!(
        &mut w,
        "
      use crate::hash;
      use crate::keyword_list;
      const HASH_TABLE_SIZE: usize = {};
      const HASH_TABLE_SEED: u64 = {};
      const STRING_TABLE_SIZE: usize = {};
    ",
        hash_table_size,
        hash_table_seed,
        keyword_bytes.len()
    )
    .unwrap();

    write!(
        &mut w,
        "
      pub struct KeywordTableEntry(u32, u32);

      pub struct KeywordTable {{
        entries: [KeywordTableEntry; HASH_TABLE_SIZE],
        bytes: [u8; STRING_TABLE_SIZE],
      }}

      impl KeywordTable {{
        pub fn is_keyword(&self, candidate: &str) -> bool {{
          let slice = candidate.as_bytes();
          let clen = candidate.len();
          if clen < keyword_list::MIN_KEY_LENGTH || clen > keyword_list::MAX_KEY_LENGTH {{
            return false;
          }}

          let selection = unsafe {{ 
            hash::select(candidate)
          }};

          let hash_code = hash::mix(selection, HASH_TABLE_SEED);
          let idx = hash_code as usize % HASH_TABLE_SIZE;
          let KeywordTableEntry(key_start, len) = self.entries[idx];
          let key_start = key_start as usize;
          let len = len as usize;
          
          let keyword = &self.bytes[key_start..key_start + len];
          slice == keyword
        }}
      }}

      pub static KEYWORD_HASH_TABLE: KeywordTable = KeywordTable {{
        entries: {:?},
        bytes: {:?},
      }};
      ",
        generate_keyword_table_entries(hash_table, &string_table),
        keyword_bytes
    )
    .unwrap();

    w.flush().unwrap();
}
