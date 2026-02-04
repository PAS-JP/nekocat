#[cfg(any(
    feature = "builder",
    feature = "parser",
    feature = "cipher",
    feature = "math",
    feature = "print",
    feature = "search",
    feature = "sql",
    feature = "pqcrypto",
    feature = "path2enum"
))]
pub use syn::parse_macro_input;
