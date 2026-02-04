use super::prelude::*;

pub fn build_field_methods(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let mut_field_ident = format_ident!("mut_{field_ident}");
    let get_field_ident = format_ident!("get_{field_ident}");
    let get_mut_field_ident = format_ident!("get_mut_{field_ident}");
    let get_ref_field_ident = format_ident!("get_ref_{field_ident}");

    let Opt { name, .. } = get_opt(&field.attrs);
    let ty = &field.ty;
    let struct_name = get_struct_name(input);
    let impl_block = get_impl(input);
    let type_name_ts_special: TokenStream = build_get_type_name_ts(&struct_name, field_ident, name);

    let method: TokenStream = build_method(input, field);

    quote! {
        #[derive(Debug)]
        pub struct #type_name_ts_special(pub #ty);
        #method
        impl #impl_block {
            pub fn #field_ident(mut self, new_value: #type_name_ts_special) -> Self {
                self.#field_ident = new_value.into();
                self
            }
            pub fn #mut_field_ident(&mut self, new_value: #type_name_ts_special) -> &mut Self {
                self.#field_ident = new_value.into();
                self
            }
            pub fn #get_field_ident(self) -> #ty {
                self.#field_ident
            }
            pub fn #get_ref_field_ident(&self) -> &#ty {
                &self.#field_ident
            }
            pub fn #get_mut_field_ident(&mut self) -> &mut #ty {
                &mut self.#field_ident
            }
        }

        impl #type_name_ts_special {
            fn get(self) -> #ty {
                self.0
            }
        }

        impl From<#type_name_ts_special> for #ty {
            fn from(value: #type_name_ts_special) -> Self {
                value.0
            }
        }

        impl AsRef<#ty> for #type_name_ts_special {
            fn as_ref(&self) -> &#ty {
                &self.0
            }
        }

        impl std::ops::Deref for #type_name_ts_special {
            type Target = #ty;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}
