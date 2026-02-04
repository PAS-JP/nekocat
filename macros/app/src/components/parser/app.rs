use super::prelude::*;

pub fn parser_app(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_clone = input.clone();
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;
    let impl_block = get_impl(input);
    let struct_name = get_struct_name(input);
    let key_enum_ident = format_ident!("{struct_name}ParserKey");
    let value_enum_ident = format_ident!("{struct_name}ParserValue");
    let field_enum_idents: Vec<Ident> = fields
        .iter()
        .map(|f| {
            let field_captalize: String = f
                .ident
                .as_ref()
                .expect("field name must be set")
                .to_string()
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect();
            format_ident!("{struct_name}{field_captalize}")
        })
        .collect();
    let field_types: Vec<Type> = fields.iter().map(|f| f.ty.clone()).collect();
    let field_idents: Vec<Ident> = fields
        .iter()
        .map(|f| f.ident.as_ref().expect("field name must be set").clone())
        .collect();
    let type_hash_map_ident = format_ident!("{struct_name}HashMap");
    let type_hash_set_ident = format_ident!("{struct_name}HashSet");

    let retain_arms: Vec<TokenStream> = fields
        .iter()
        .filter_map(|f| {
            let ident = f.ident.as_ref().unwrap();
            let field_capitalize: String = ident
                .to_string()
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect();

            let variant = format_ident!("{struct_name}{field_capitalize}");
            let ty = &f.ty;

            if is_string(ty) || is_str_ref(ty) || is_vec(ty) {
                Some(quote! {
                    #value_enum_ident::#variant(v) if v.is_empty() => false
                })
            } else if is_bool(ty) {
                Some(quote! {
                    #value_enum_ident::#variant(false) => false
                })
            } else if is_int(ty) {
                Some(quote! {
                    #value_enum_ident::#variant(v) if *v == 0 => false
                })
            } else if is_float(ty) {
                if let Type::Path(type_path) = ty {
                    if let Some(seg) = type_path.path.segments.last() {
                        let zero = if seg.ident == "f32" {
                            quote! { 0.0f32 }
                        } else {
                            quote! { 0.0f64 }
                        };
                        Some(quote! {
                            #value_enum_ident::#variant(v) if v == #zero => false
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else if is_option(ty) {
                Some(quote! {
                    #value_enum_ident::#variant(None) => false
                })
            } else {
                None
            }
        })
        .collect();

    quote! {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum #key_enum_ident {
            #(#field_enum_idents),*
        }
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum #value_enum_ident {
            #(#field_enum_idents(#field_types)),*
        }
        pub type #type_hash_map_ident = std::collections::HashMap<#key_enum_ident, #value_enum_ident>;
        pub type #type_hash_set_ident = std::collections::HashSet<#value_enum_ident>;

        impl #impl_block {
            pub fn to_hash_map(self) -> #type_hash_map_ident {
                self.into()
            }

            pub fn to_clean_hash_map(self) -> #type_hash_map_ident {
                let mut map: #type_hash_map_ident = self.into();
                map.retain(|_, v| match v {
                    #(#retain_arms),*
                    , _ => true,
                });
                map
            }

            pub fn to_hash_set(self) -> #type_hash_set_ident {
                self.into()
            }

            pub fn to_bytes(self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
                let aligned = rkyv::to_bytes::<rkyv::rancor::Error>(&self)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(aligned.into())
            }

            pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let value = rkyv::from_bytes::<#impl_block, rkyv::rancor::Error>(&bytes)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(value)
            }

            pub fn from_hash_map(
                map: #type_hash_map_ident,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                #(
                    let mut #field_idents: Option<#field_types> = None;
                )*

                for (k, v) in map {
                    match (k, v) {
                        #(
                            (
                                #key_enum_ident::#field_enum_idents,
                                #value_enum_ident::#field_enum_idents(val),
                            ) => {
                                #field_idents = Some(val);
                            }
                        )*
                        _ => {}
                    }
                }

                Ok(Self {
                    #(
                        #field_idents: #field_idents.ok_or_else(|| {
                            Box::<dyn std::error::Error + Send + Sync>::from(
                                format!("missing field {}", stringify!(#field_idents)),
                            )
                        })?,
                    )*
                })
            }

            pub fn from_hash_set(
                set: #type_hash_set_ident,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                #(
                    let mut #field_idents: Option<#field_types> = None;
                )*

                for v in set {
                    match v {
                        #(
                            #value_enum_ident::#field_enum_idents(val) => {
                                #field_idents = Some(val);
                            }
                        )*
                    }
                }

                Ok(Self {
                    #(
                        #field_idents: #field_idents.ok_or_else(|| {
                            Box::<dyn std::error::Error + Send + Sync>::from(
                                format!("missing field {}", stringify!(#field_idents)),
                            )
                        })?,
                    )*
                })
            }
        }

        impl TryFrom<Vec<u8>> for #impl_block {
            type Error = Box<dyn std::error::Error + Send + Sync>;

            fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                let value = rkyv::from_bytes::<#impl_block, rkyv::rancor::Error>(&bytes)
                    .map_err(|e| Box::new(e) as Self::Error)?;
                Ok(value)
            }
        }

        impl TryInto<Vec<u8>> for #impl_block {
            type Error = Box<dyn std::error::Error + Send + Sync>;

            fn try_into(self) -> Result<Vec<u8>, Self::Error> {
                let aligned = rkyv::to_bytes::<rkyv::rancor::Error>(&self)
                    .map_err(|e| Box::new(e) as Self::Error)?;
                Ok(aligned.into())
            }
        }

        impl Into<#type_hash_set_ident> for #impl_block {
            fn into(self) -> #type_hash_set_ident {
                let #struct_name { #(#field_idents),* } = self;
                let mut set = std::collections::HashSet::new();
                #(
                    set.insert(#value_enum_ident::#field_enum_idents(#field_idents));
                )*
                set
            }
        }

        impl Into<#type_hash_map_ident> for #impl_block {
            fn into(self) -> #type_hash_map_ident {
                let #struct_name { #(#field_idents),* } = self;
                let mut map = std::collections::HashMap::new();
                #(
                    map.insert(
                        #key_enum_ident::#field_enum_idents,
                        #value_enum_ident::#field_enum_idents(#field_idents)
                    );
                )*

                map
            }
        }
    }
}
