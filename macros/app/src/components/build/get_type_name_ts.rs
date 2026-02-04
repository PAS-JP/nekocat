use super::prelude::*;

pub fn build_get_type_name_ts(
    struct_name: &Ident,
    field_ident: &Ident,
    name: Option<TokenStream>,
) -> TokenStream {
    match name {
        Some(ts) => ts,
        None => {
            let pascal = field_ident
                .to_string()
                .split('_')
                .filter(|s| !s.is_empty())
                .map(|seg| {
                    let mut chars = seg.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<String>();
            let ident = format_ident!("{struct_name}{pascal}");
            quote! { #ident }
        }
    }
}
