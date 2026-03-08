use super::import::*;

pub fn math_inflate(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_inflate = format_ident!("inflate_{field_name}");

    quote! {
        pub fn #method_name_inflate(&mut self, percentage: #field_type) {
            self.#field_name = {
                let original = self.#field_name;
                let pct_f64 = percentage as f64;

                if (pct_f64.is_nan()) || (pct_f64.is_infinite()) {
                    original
                } else {
                    let inflated = (original as f64) * (1.0 + pct_f64 / 100.0);

                    if inflated > (#field_type::MAX as f64) {
                        #field_type::MAX
                    } else {
                        inflated as #field_type
                    }
                }
            };
        }
    }
}
