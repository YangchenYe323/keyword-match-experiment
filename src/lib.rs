#![feature(portable_simd)]

use std::simd::{Simd, SimdPartialEq, ToBitMask};

pub const ELEMENTS: usize = 16;

type SimdVec = Simd<u8, ELEMENTS>;

pub fn slice_match(candidate: &[u8], key: &[u8]) -> bool {
    candidate == key
}

pub fn str_match(candidate: &str, key: &str) -> bool {
    candidate == key
}

pub fn simd_slice_match(candidate: &[u8], key: &[u8]) -> bool {
    if candidate.len() != key.len() {
        return false;
    }
    unsafe { simd_slice_match_inner(candidate, key, candidate.len()) }
}

pub fn simd_str_match(candidate: &str, key: &str) -> bool {
    simd_slice_match(candidate.as_bytes(), key.as_bytes())
}

/// Compare two slices by loading them as a Simd<u8, 16>. This function assumes
/// `candidate` and `key` both points to a readable memory of at least 16 bytes long.
#[inline]
unsafe fn simd_slice_match_inner(candidate: &[u8], key: &[u8], size: usize) -> bool {
    let candidate = std::slice::from_raw_parts(candidate.as_ptr(), ELEMENTS);
    let key = std::slice::from_raw_parts(key.as_ptr(), ELEMENTS);
    let candidate = SimdVec::from_slice(candidate);
    let key = SimdVec::from_slice(key);
    let non_equal_mask = candidate.simd_ne(key).to_bitmask();
    let size_mask = !(!0u16 << size);
    non_equal_mask & size_mask == 0
}
