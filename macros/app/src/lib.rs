mod components;

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
use components::prelude::*;

#[cfg(feature = "builder")]
#[proc_macro_derive(Builder, attributes(opt))]
pub fn builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generate = build_app(&input);
    generate.into()
}

#[cfg(feature = "print")]
#[proc_macro_derive(Print, attributes(transporter))]
pub fn print(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut expanded = proc_macro2::TokenStream::new();
    for_extend_token_stream(
        &mut expanded,
        vec![print_by_field(&input), print_method(&input)],
    );
    expanded.into()
}

#[cfg(feature = "math")]
#[proc_macro_derive(Math)]
pub fn math(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_math = math_app(&input);
    generate_math.into()
}

#[cfg(feature = "cipher")]
#[proc_macro_derive(Cipher, attributes(opt))]
pub fn cipher(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_cipher = cipher_app(&input);
    generate_cipher.into()
}

#[cfg(feature = "parser")]
#[proc_macro_derive(Parser, attributes(opt))]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_into = parser_app(&input);
    generate_into.into()
}

#[cfg(feature = "search")]
#[proc_macro_derive(Search, attributes(opt))]
pub fn search(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_into = search_app(&input);
    generate_into.into()
}

#[cfg(feature = "sql")]
#[proc_macro_derive(Sql, attributes(opt))]
pub fn sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generate_into = sql_app(&input);
    generate_into.into()
}

//#[cfg(feature = "obf")]
#[proc_macro_attribute]
/// Procedural macro `path2enum` — generates enums from real filesystem paths.
///
/// ## Parameters
/// - `path: &str` — root directory to scan (default: `"."`).
/// - `ext: &str` — comma-separated list of allowed extensions (e.g. `"rs,svg,toml"`; default: `"svg"`).
/// - `prefix: &str` — optional logical prefix added to the paths returned by `to_str()`.
///
/// ## Behavior
/// - Recursively scans the given `path`.
/// - Generates an enum variant **for each directory** and **for each file** matching the allowed extensions.
///   Example: both `a/b/c` (directory) and `a/b/c/file.svg` (file) will appear as distinct variants.
/// - Variants are made readable:
///   - each segment is converted to PascalCase;
///   - path separators are represented by `ノ` (katakana no);
///   - the separator between filename and extension is `・` (middle dot).
/// - If a segment starts with invalid characters for a Rust identifier (e.g. dot, digit), it is prefixed with `_`.
/// - `Enum::to_str()` always returns the original logical path (with hyphens, underscores, extension, and prefix intact).
///
/// ## Examples (doc-tests)
///
/// ```rust
/// # #![allow(mixed_script_confusables)]
/// # use path2enum::path2enum;
///
/// #[path2enum(path = "tests/assets", ext = "svg,toml")]
/// pub enum PublicPaths {}
///
/// // file variants
/// assert_eq!(PublicPaths::ArrowLeft・svg.to_str(), "arrow-left.svg");
/// assert_eq!(PublicPaths::NestedDirノIcon・svg.to_str(), "nested_dir/icon.svg");
/// assert_eq!(PublicPaths::NestedDirノDeepDirノDeepIcon・svg.to_str(), "nested_dir/deep_dir/deep-icon.svg");
///
/// // directory variants
/// assert_eq!(PublicPaths::NestedDir.to_str(), "nested_dir");
/// assert_eq!(PublicPaths::NestedDirノDeepDir.to_str(), "nested_dir/deep_dir");
///
/// #[path2enum(ext = "rs,svg,toml")]
/// pub enum ProjectPaths {}
///
/// assert_eq!(ProjectPaths::SrcノLib・rs.to_str(), "src/lib.rs");
/// assert_eq!(ProjectPaths::TestsノAssetsノArrowLeft・svg.to_str(), "tests/assets/arrow-left.svg");
/// assert_eq!(ProjectPaths::Cargo・toml.to_str(), "Cargo.toml");
///
/// // directory variant inside project
/// assert_eq!(ProjectPaths::TestsノAssets.to_str(), "tests/assets");
///
/// #[path2enum(path = "tests/assets", ext = "svg", prefix = "assets")]
/// pub enum Icons {}
///
/// assert_eq!(Icons::AssetsノHome・svg.to_str(), "assets/home.svg");
/// assert_eq!(Icons::Assetsノ_11Testノ_11・svg.to_str(), "assets/11-test/11.svg");
/// assert_eq!(Icons::AssetsノNestedDirノDeepDirノDeepIcon・svg.to_str(), "assets/nested_dir/deep_dir/deep-icon.svg");
///
/// // directory variants with prefix
/// assert_eq!(Icons::Assetsノ_11Test.to_str(), "assets/11-test");
/// assert_eq!(Icons::AssetsノNestedDirノDeepDir.to_str(), "assets/nested_dir/deep_dir");
/// ```
///
/// ## Notes
/// - Variants use `ノ` and `・` for readability in code and tests.
/// - An ASCII-only mode (e.g. `ArrowLeft_svg`) could be added as an option if needed.
/// - Use `to_str()` at runtime to retrieve the original logical path.
pub fn path2enum(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemEnum);
    path2enum_app(attr, item)
}
