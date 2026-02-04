use super::prelude::*;

pub fn math_app(input: &DeriveInput) -> proc_macro2::TokenStream {
    let impl_block = get_impl(input);
    let mut methods: Vec<proc_macro2::TokenStream> = Vec::new();

    for field in get_named_fields(input).unwrap().named.iter() {
        methods.push(math_discount(field));
        methods.push(math_div(field));
        methods.push(math_mul(field));
        methods.push(math_sub(field));
        methods.push(math_sum(field));
        methods.push(math_inflate(field));
        methods.push(math_approach(field));
    }

    quote! {
        impl #impl_block {
            #(#methods)*
        }
    }
}
