use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::{
    keyword_generated::{PerfectKeywordHasherBuilder, KEYWORD_HASH_TABLE},
    keyword_list, MAX_JS_KEYWORD_LENGTH, MIN_JS_KEYWORD_LENGTH,
};

lazy_static! {
    pub static ref KEYWORD_RUST_HASH_TABLE_CUSTOM_HASH: HashSet<&'static [u8], PerfectKeywordHasherBuilder> = {
        let mut set = HashSet::with_hasher(PerfectKeywordHasherBuilder);
        for key in keyword_list::KEYWORDS.iter().copied() {
            set.insert(key.as_bytes());
        }
        set
    };
    pub static ref KEYWORD_RUST_HASH_TABLE: HashSet<&'static str> =
        keyword_list::KEYWORDS.iter().copied().collect();
}

#[inline]
pub fn match_keyword_rust_custom_hash(s: &str) -> bool {
    if s.len() < MIN_JS_KEYWORD_LENGTH || s.len() > MAX_JS_KEYWORD_LENGTH {
        return false;
    }
    KEYWORD_RUST_HASH_TABLE_CUSTOM_HASH.contains(s.as_bytes())
}

#[inline]
pub fn match_keyword_rust_hash(s: &str) -> bool {
    KEYWORD_RUST_HASH_TABLE.contains(s)
}

#[inline]
pub fn match_keyword_baseline(s: &str) -> bool {
    match s {
        "as" => true,
        "do" => true,
        "if" => true,
        "in" => true,
        "is" => true,
        "of" => true,

        "any" => true,
        "for" => true,
        "get" => true,
        "let" => true,
        "new" => true,
        "out" => true,
        "set" => true,
        "try" => true,
        "var" => true,

        "case" => true,
        "else" => true,
        "enum" => true,
        "from" => true,
        "meta" => true,
        "null" => true,
        "this" => true,
        "true" => true,
        "type" => true,
        "void" => true,
        "with" => true,

        "async" => true,
        "await" => true,
        "break" => true,
        "catch" => true,
        "class" => true,
        "const" => true,
        "false" => true,
        "infer" => true,
        "keyof" => true,
        "never" => true,
        "super" => true,
        "throw" => true,
        "while" => true,
        "yield" => true,

        "assert" => true,
        "bigint" => true,
        "delete" => true,
        "export" => true,
        "global" => true,
        "import" => true,
        "module" => true,
        "number" => true,
        "object" => true,
        "public" => true,
        "return" => true,
        "static" => true,
        "string" => true,
        "switch" => true,
        "symbol" => true,
        "target" => true,
        "typeof" => true,
        "unique" => true,

        "asserts" => true,
        "boolean" => true,
        "declare" => true,
        "default" => true,
        "extends" => true,
        "finally" => true,
        "package" => true,
        "private" => true,
        "require" => true,
        "unknown" => true,

        "abstract" => true,
        "accessor" => true,
        "continue" => true,
        "debugger" => true,
        "function" => true,
        "override" => true,
        "readonly" => true,

        "interface" => true,
        "intrinsic" => true,
        "namespace" => true,
        "protected" => true,
        "satisfies" => true,
        "undefined" => true,

        "implements" => true,
        "instanceof" => true,

        "constructor" => true,
        _ => false,
    }
}

#[inline]
pub fn match_keyword_perfect_hash(s: &str) -> bool {
    KEYWORD_HASH_TABLE.is_keyword(s)
}

#[cfg(test)]
#[test]
fn test_hash_correctness() {
    for key in crate::utils::read_keys_from_file("mixed.txt") {
        let key = unsafe { std::str::from_utf8_unchecked(&key) };
        let expected = match_keyword_baseline(key);
        assert_eq!(expected, match_keyword_perfect_hash(key));
        assert_eq!(expected, match_keyword_rust_hash(key));
        assert_eq!(expected, match_keyword_rust_custom_hash(key));
    }
}
