pub use super::prelude::*;

pub fn get_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let (impl_generics, type_generics, where_clause) = generics_split_for_impl(input);
    let struct_name = get_struct_name(input);

    quote! {
        #impl_generics #struct_name #type_generics #where_clause
    }
}
