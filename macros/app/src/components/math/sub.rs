use super::import::*;

pub fn math_sub(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_sub = format_ident!("sub_{field_name}");

    quote! {
        pub fn #method_name_sub(&mut self, other: #field_type) {
            self.#field_name -= other;
        }
    }
}
