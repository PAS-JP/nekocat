use super::prelude::*;

pub fn cipher_app(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_clone = input.clone();
    //let struct_name = get_struct_name(input);
    //let impl_block = get_impl(input);
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;

    let field_methods: Vec<_> = fields
        .iter()
        .filter(|f| is_string(&f.ty) || is_str_ref(&f.ty))
        .map(|f| cipher_field_methods(input, f))
        .collect();

    quote! {
        #(#field_methods)*
    }
}
