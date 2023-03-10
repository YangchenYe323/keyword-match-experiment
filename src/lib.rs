#![feature(portable_simd, hasher_prefixfree_extras)]

mod hash;
mod keyword_generated;
pub mod generate_keyword;
mod keyword_list;
mod keyword_match;
pub mod utils;

use std::simd::{Simd, SimdPartialEq, ToBitMask};

pub use keyword_list::{MAX_JS_KEYWORD_LENGTH, MIN_JS_KEYWORD_LENGTH};
pub use keyword_match::{match_keyword_perfect_hash, match_keyword_baseline, match_keyword_rust_hash, match_keyword_rust_custom_hash};

pub const ELEMENTS: usize = 16;

pub type SimdVec = Simd<u8, ELEMENTS>;

pub fn slice_match(candidate: &[u8], key: &[u8]) -> bool {
    candidate == key
}

pub fn str_match(candidate: &str, key: &str) -> bool {
    candidate == key
}

/// Preconditions:
/// candidate[0..16] is readable
/// key[0..16] is readable
/// 1 <= candidate.len() <= 16
/// 1 <= key.len() <= 16
pub fn simd_slice_match(candidate: &[u8], key: &[u8]) -> bool {
    if candidate.len() != key.len() {
        return false;
    }
    unsafe { simd_slice_match_inner(candidate, key, candidate.len()) }
}

/// Preconditions:
/// candidate[0..16] is readable
/// key[0..16] is readable
pub fn simd_str_match(candidate: &str, key: &str) -> bool {
    simd_slice_match(candidate.as_bytes(), key.as_bytes())
}

/// Compare two slices by loading them as a Simd<u8, 16>. This function assumes
/// `candidate` and `key` both points to a readable memory of at least 16 bytes long.
///
/// Preconditions:
/// candidate[0..16] is readable
/// key[0..16] is readable
/// candidate.len() == key.len() == size
#[inline]
unsafe fn simd_slice_match_inner(candidate: &[u8], key: &[u8], size: usize) -> bool {
    // Very unsafe magic, by which we cast candidate and key to a slice of ELEMENTS size. The
    // user should make sure candidate and key both point to a start of at least ELEMENTS readable
    // bytes.
    let candidate = std::slice::from_raw_parts(candidate.as_ptr(), ELEMENTS);
    let key = std::slice::from_raw_parts(key.as_ptr(), ELEMENTS);

    let candidate = SimdVec::from_slice(candidate);
    let key = SimdVec::from_slice(key);
    let non_equal_mask = candidate.simd_ne(key).to_bitmask();
    let size_mask = !(!0u16 << size);
    non_equal_mask & size_mask == 0
}
