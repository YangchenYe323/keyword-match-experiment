use crate::hash;
use crate::keyword_list;
const HASH_TABLE_SIZE: usize = 512;
const HASH_TABLE_SEED: u64 = 6842106309907142171;
const STRING_TABLE_SIZE: usize = 461;

pub struct KeywordTableEntry {
    key_start: usize, /*Index of the string table */
    len: usize,
}

pub struct KeywordTable {
    entries: [KeywordTableEntry; HASH_TABLE_SIZE], /*The Hash Table, each entry indexing into the string table. */
    bytes: [u8; STRING_TABLE_SIZE], /*The string table, padded to ensure 16 bytes slice available for every keyword. */
}

impl KeywordTable {
    pub fn is_keyword(&self, candidate: &str) -> bool {
        let slice = candidate.as_bytes();
        let clen = candidate.len();
        if clen < keyword_list::MIN_JS_KEYWORD_LENGTH || clen > keyword_list::MAX_JS_KEYWORD_LENGTH
        {
            return false;
        }

        // run the hash function
        let selection = hash::select(candidate);
        let hash_code = hash::mix(selection, HASH_TABLE_SEED);
        let idx = hash_code as usize % HASH_TABLE_SIZE;
        let KeywordTableEntry { key_start, len } = self.entries[idx];

        let keyword = &self.bytes[key_start..key_start + len];
        slice == keyword
    }
}

pub static KEYWORD_HASH_TABLE: KeywordTable = KeywordTable {
    entries: [
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 185,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 94,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 275,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 34,
            len: 9,
        },
        KeywordTableEntry {
            key_start: 125,
            len: 8,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 162,
            len: 8,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 133,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 43,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 103,
            len: 9,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 118,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 342,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 71,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 428,
            len: 8,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 381,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 325,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 52,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 65,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 402,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 425,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 252,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 408,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 335,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 418,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 43,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 9,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 173,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 436,
            len: 8,
        },
        KeywordTableEntry {
            key_start: 56,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 365,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 242,
            len: 10,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 170,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 257,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 376,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 390,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 47,
            len: 2,
        },
        KeywordTableEntry {
            key_start: 321,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 197,
            len: 9,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 296,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 49,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 39,
            len: 2,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 233,
            len: 9,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 28,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 61,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 155,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 88,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 224,
            len: 9,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 63,
            len: 2,
        },
        KeywordTableEntry {
            key_start: 77,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 365,
            len: 11,
        },
        KeywordTableEntry {
            key_start: 357,
            len: 8,
        },
        KeywordTableEntry {
            key_start: 293,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 192,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 349,
            len: 8,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 212,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 273,
            len: 2,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 206,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 329,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 189,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 139,
            len: 9,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 219,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 100,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 216,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 281,
            len: 8,
        },
        KeywordTableEntry {
            key_start: 68,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 14,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 418,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 83,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 179,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 263,
            len: 10,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 128,
            len: 2,
        },
        KeywordTableEntry {
            key_start: 315,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 386,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 303,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 413,
            len: 5,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 308,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 395,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 378,
            len: 2,
        },
        KeywordTableEntry {
            key_start: 5,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 21,
            len: 7,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 289,
            len: 4,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 112,
            len: 6,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 18,
            len: 3,
        },
        KeywordTableEntry {
            key_start: 0,
            len: 0,
        },
        KeywordTableEntry {
            key_start: 148,
            len: 7,
        },
    ],
    // A compact byte array where all the keywords are pcaked together
    bytes: [
        121, 105, 101, 108, 100, 119, 105, 116, 104, 119, 104, 105, 108, 101, 118, 111, 105, 100,
        118, 97, 114, 117, 110, 107, 110, 111, 119, 110, 117, 110, 105, 113, 117, 101, 117, 110,
        100, 101, 102, 105, 110, 101, 100, 116, 121, 112, 101, 111, 102, 116, 114, 121, 116, 114,
        117, 101, 116, 104, 114, 111, 119, 116, 104, 105, 115, 116, 97, 114, 103, 101, 116, 115,
        121, 109, 98, 111, 108, 115, 119, 105, 116, 99, 104, 115, 117, 112, 101, 114, 115, 116,
        114, 105, 110, 103, 115, 116, 97, 116, 105, 99, 115, 101, 116, 115, 97, 116, 105, 115, 102,
        105, 101, 115, 114, 101, 116, 117, 114, 110, 114, 101, 113, 117, 105, 114, 101, 114, 101,
        97, 100, 111, 110, 108, 121, 112, 117, 98, 108, 105, 99, 112, 114, 111, 116, 101, 99, 116,
        101, 100, 112, 114, 105, 118, 97, 116, 101, 112, 97, 99, 107, 97, 103, 101, 111, 118, 101,
        114, 114, 105, 100, 101, 111, 117, 116, 111, 98, 106, 101, 99, 116, 110, 117, 109, 98, 101,
        114, 110, 117, 108, 108, 110, 101, 119, 110, 101, 118, 101, 114, 110, 97, 109, 101, 115,
        112, 97, 99, 101, 109, 111, 100, 117, 108, 101, 109, 101, 116, 97, 108, 101, 116, 107, 101,
        121, 111, 102, 105, 110, 116, 114, 105, 110, 115, 105, 99, 105, 110, 116, 101, 114, 102,
        97, 99, 101, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 105, 110, 102, 101, 114, 105,
        109, 112, 111, 114, 116, 105, 109, 112, 108, 101, 109, 101, 110, 116, 115, 105, 102, 103,
        108, 111, 98, 97, 108, 102, 117, 110, 99, 116, 105, 111, 110, 102, 114, 111, 109, 102, 111,
        114, 102, 105, 110, 97, 108, 108, 121, 102, 97, 108, 115, 101, 101, 120, 116, 101, 110,
        100, 115, 101, 120, 112, 111, 114, 116, 101, 110, 117, 109, 101, 108, 115, 101, 100, 101,
        108, 101, 116, 101, 100, 101, 102, 97, 117, 108, 116, 100, 101, 99, 108, 97, 114, 101, 100,
        101, 98, 117, 103, 103, 101, 114, 99, 111, 110, 116, 105, 110, 117, 101, 99, 111, 110, 115,
        116, 114, 117, 99, 116, 111, 114, 99, 108, 97, 115, 115, 99, 97, 116, 99, 104, 99, 97, 115,
        101, 98, 114, 101, 97, 107, 98, 111, 111, 108, 101, 97, 110, 98, 105, 103, 105, 110, 116,
        97, 119, 97, 105, 116, 97, 115, 121, 110, 99, 97, 115, 115, 101, 114, 116, 115, 97, 110,
        121, 97, 99, 99, 101, 115, 115, 111, 114, 97, 98, 115, 116, 114, 97, 99, 116, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
