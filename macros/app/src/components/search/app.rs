use super::prelude::*;

pub fn search_app(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_clone = input.clone();
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;
    let field_methods: Vec<_> = fields
        .iter()
        .map(|f| search_field_methods(input, f))
        .collect();

    quote! {
        #(#field_methods)*
    }
}
