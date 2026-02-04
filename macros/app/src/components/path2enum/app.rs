pub use super::prelude::*;

pub fn path2enum_app(
    attr: proc_macro::TokenStream,
    input_enum: ItemEnum,
) -> proc_macro::TokenStream {
    let attr_ts2: TokenStream2 = attr.into();
    let attr: Attribute = syn::parse_quote!(#[magic(#attr_ts2)]);

    let mut root = None;
    let mut ext: Option<Vec<String>> = None;
    let mut prefix = String::new();

    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("path") {
            let value = meta.value()?.parse::<syn::LitStr>()?;
            root = Some(value.value());
            Ok(())
        } else if meta.path.is_ident("ext") {
            let value = meta.value()?.parse::<syn::LitStr>()?;
            let exts = value
                .value()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            ext = Some(exts);
            Ok(())
        } else if meta.path.is_ident("prefix") {
            let value = meta.value()?.parse::<syn::LitStr>()?;
            prefix = value.value();
            Ok(())
        } else {
            Err(meta.error("Only `path`, `ext`, and `prefix` are supported"))
        }
    });

    // Default to project root if no path is provided
    let root = root.unwrap_or_else(|| ".".to_string());
    let ext = ext.unwrap_or_else(|| vec!["svg".to_string()]);
    let root_path = PathBuf::from(&root);

    let enum_ident = &input_enum.ident;

    let mut variants = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    collect_paths(&root_path, &ext, &mut variants, "", &prefix, &mut seen);

    // Sort by readable ident (string form of Ident) for deterministic output
    variants.sort_by(|a, b| a.0.to_string().cmp(&b.0.to_string()));

    let variant_defs = variants.iter().map(|(ident, _)| quote! { #ident, });

    let match_arms = variants.iter().map(|(ident, original_path)| {
        let lit = syn::LitStr::new(original_path, Span::call_site());
        quote! {
            Self::#ident => #lit,
        }
    });

    let expanded = quote! {
        #[allow(mixed_script_confusables)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_ident {
            #(#variant_defs)*
        }

        impl #enum_ident {
            pub fn to_str(&self) -> &'static str {
                match self {
                    #(#match_arms)*
                    _ => unreachable!("Unrecognized variant in generated enum {}", stringify!(#enum_ident)),
                }
            }

            pub fn to_string(&self) -> String {
                self.to_str().to_string()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
