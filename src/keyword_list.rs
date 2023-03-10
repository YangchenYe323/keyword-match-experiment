pub const MIN_KEY_LENGTH: usize = 2;
pub const MAX_KEY_LENGTH: usize = 16;

pub const MIN_JS_KEYWORD_LENGTH: usize = 2;
pub const MAX_JS_KEYWORD_LENGTH: usize = 11;

pub static KW_IS: &str = "is";
pub static KW_AS: &str = "as";
pub static KW_DO: &str = "do";
pub static KW_IF: &str = "if";
pub static KW_IN: &str = "in";
pub static KW_OF: &str = "of";
pub static KW_ANY: &str = "any";
pub static KW_FOR: &str = "for";
pub static KW_GET: &str = "get";
pub static KW_LET: &str = "let";
pub static KW_NEW: &str = "new";
pub static KW_OUT: &str = "out";
pub static KW_SET: &str = "set";
pub static KW_TRY: &str = "try";
pub static KW_VAR: &str = "var";
pub static KW_CASE: &str = "case";
pub static KW_ELSE: &str = "else";
pub static KW_ENUM: &str = "enum";
pub static KW_FROM: &str = "from";
pub static KW_META: &str = "meta";
pub static KW_NULL: &str = "null";
pub static KW_THIS: &str = "this";
pub static KW_TRUE: &str = "true";
pub static KW_TYPE: &str = "type";
pub static KW_VOID: &str = "void";
pub static KW_WITH: &str = "with";
pub static KW_ASYNC: &str = "async";
pub static KW_AWAIT: &str = "await";
pub static KW_BREAK: &str = "break";
pub static KW_CATCH: &str = "catch";
pub static KW_CLASS: &str = "class";
pub static KW_CONST: &str = "const";
pub static KW_FALSE: &str = "false";
pub static KW_INFER: &str = "infer";
pub static KW_KEYOF: &str = "keyof";
pub static KW_NEVER: &str = "never";
pub static KW_SUPER: &str = "super";
pub static KW_THROW: &str = "throw";
pub static KW_WHILE: &str = "while";
pub static KW_YIELD: &str = "yield";
pub static KW_ASSERT: &str = "assert";
pub static KW_BIGINT: &str = "bigint";
pub static KW_DELETE: &str = "delete";
pub static KW_EXPORT: &str = "export";
pub static KW_GLOBAL: &str = "global";
pub static KW_IMPORT: &str = "import";
pub static KW_MODULE: &str = "module";
pub static KW_NUMBER: &str = "number";
pub static KW_OBJECT: &str = "object";
pub static KW_PUBLIC: &str = "public";
pub static KW_RETURN: &str = "return";
pub static KW_STATIC: &str = "static";
pub static KW_STRING: &str = "string";
pub static KW_SWITCH: &str = "switch";
pub static KW_SYMBOL: &str = "symbol";
pub static KW_TARGET: &str = "target";
pub static KW_TYPEOF: &str = "typeof";
pub static KW_UNIQUE: &str = "unique";
pub static KW_ASSERTS: &str = "asserts";
pub static KW_BOOLEAN: &str = "boolean";
pub static KW_DECLARE: &str = "declare";
pub static KW_DEFAULT: &str = "default";
pub static KW_EXTENDS: &str = "extends";
pub static KW_FINALLY: &str = "finally";
pub static KW_PACKAGE: &str = "package";
pub static KW_PRIVATE: &str = "private";
pub static KW_REQUIRE: &str = "require";
pub static KW_UNKNOWN: &str = "unknown";
pub static KW_ABSTRACT: &str = "abstract";
pub static KW_ACCESSOR: &str = "accessor";
pub static KW_CONTINUE: &str = "continue";
pub static KW_DEBUGGER: &str = "debugger";
pub static KW_FUNCTION: &str = "function";
pub static KW_OVERRIDE: &str = "override";
pub static KW_READONLY: &str = "readonly";
pub static KW_INTERFACE: &str = "interface";
pub static KW_INTRINSIC: &str = "intrinsic";
pub static KW_NAMESPACE: &str = "namespace";
pub static KW_PROTECTED: &str = "protected";
pub static KW_SATISFIES: &str = "satisfies";
pub static KW_UNDEFINED: &str = "undefined";
pub static KW_IMPLEMENTS: &str = "implements";
pub static KW_INSTANCEOF: &str = "instanceof";
pub static KW_CONSTRUCTOR: &str = "constructor";

pub static KEYWORDS: [&'static str; 84] = [
    &KW_IS,
    &KW_AS,
    &KW_DO,
    &KW_IF,
    &KW_IN,
    &KW_OF,
    &KW_ANY,
    &KW_FOR,
    &KW_GET,
    &KW_LET,
    &KW_NEW,
    &KW_OUT,
    &KW_SET,
    &KW_TRY,
    &KW_VAR,
    &KW_CASE,
    &KW_ELSE,
    &KW_ENUM,
    &KW_FROM,
    &KW_META,
    &KW_NULL,
    &KW_THIS,
    &KW_TRUE,
    &KW_TYPE,
    &KW_VOID,
    &KW_WITH,
    &KW_ASYNC,
    &KW_AWAIT,
    &KW_BREAK,
    &KW_CATCH,
    &KW_CLASS,
    &KW_CONST,
    &KW_FALSE,
    &KW_INFER,
    &KW_KEYOF,
    &KW_NEVER,
    &KW_SUPER,
    &KW_THROW,
    &KW_WHILE,
    &KW_YIELD,
    &KW_ASSERT,
    &KW_BIGINT,
    &KW_DELETE,
    &KW_EXPORT,
    &KW_GLOBAL,
    &KW_IMPORT,
    &KW_MODULE,
    &KW_NUMBER,
    &KW_OBJECT,
    &KW_PUBLIC,
    &KW_RETURN,
    &KW_STATIC,
    &KW_STRING,
    &KW_SWITCH,
    &KW_SYMBOL,
    &KW_TARGET,
    &KW_TYPEOF,
    &KW_UNIQUE,
    &KW_ASSERTS,
    &KW_BOOLEAN,
    &KW_DECLARE,
    &KW_DEFAULT,
    &KW_EXTENDS,
    &KW_FINALLY,
    &KW_PACKAGE,
    &KW_PRIVATE,
    &KW_REQUIRE,
    &KW_UNKNOWN,
    &KW_ABSTRACT,
    &KW_ACCESSOR,
    &KW_CONTINUE,
    &KW_DEBUGGER,
    &KW_FUNCTION,
    &KW_OVERRIDE,
    &KW_READONLY,
    &KW_INTERFACE,
    &KW_INTRINSIC,
    &KW_NAMESPACE,
    &KW_PROTECTED,
    &KW_SATISFIES,
    &KW_UNDEFINED,
    &KW_IMPLEMENTS,
    &KW_INSTANCEOF,
    &KW_CONSTRUCTOR,
];
