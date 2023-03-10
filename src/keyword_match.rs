use crate::keyword_generated::KEYWORD_HASH_TABLE;

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

pub fn match_keyword(s: &str) -> bool {
    KEYWORD_HASH_TABLE.is_keyword(s)
}

#[cfg(test)]
#[test]
fn baseline_test() {
    for key in crate::keyword_list::KEYWORDS {
        let key = key.as_bytes();
        let klen = key.len();
        let mut key_padded = key.to_vec();
        key_padded.resize(16, 0);
        let slice = &key_padded[..klen];
        let key = unsafe { std::str::from_utf8_unchecked(slice) };
        assert!(match_keyword(key));
        assert!(match_keyword_baseline(key));
    }
}
