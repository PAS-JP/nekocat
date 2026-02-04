use super::import::*;

pub fn math_div(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_div = format_ident!("div_{field_name}");

    quote! {
        pub fn #method_name_div(&mut self, other: #field_type) {
            self.#field_name /= other;
        }
    }
}
