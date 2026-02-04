use super::import::*;

pub fn math_mul(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_mul = format_ident!("mul_{field_name}");

    quote! {
        pub fn #method_name_mul(&mut self, other: #field_type) {
            self.#field_name *= other;
        }
    }
}
