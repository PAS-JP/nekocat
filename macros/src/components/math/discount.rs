use super::import::*;

pub fn math_discount(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_discount = format_ident!("discount_{field_name}");

    quote! {
        pub fn #method_name_discount(&mut self, percentage: #field_type) {
            self.#field_name = {
                let value = self.#field_name as f64;
                let discounted = value * (1.0 - (percentage as f64 / 100.0));
                discounted.max(0.0) as #field_type
            };
        }
    }
}
