use super::prelude::*;

#[derive(Debug, Default)]
pub struct Opt {
    #[cfg(feature = "builder")]
    pub name: Option<TokenStream>,
    #[cfg(feature = "builder")]
    pub pattern: Option<TokenStream>,
    #[cfg(feature = "builder")]
    pub err: Option<TokenStream>,
    #[cfg(feature = "builder")]
    pub default: Option<TokenStream>,
    #[cfg(feature = "builder")]
    pub min: Option<TokenStream>,
    #[cfg(feature = "builder")]
    pub max: Option<TokenStream>,
    #[cfg(feature = "builder")]
    pub err_min: Option<TokenStream>,
    #[cfg(feature = "builder")]
    pub err_max: Option<TokenStream>,
    #[cfg(feature = "aes")]
    pub aes_secret_key: Option<TokenStream>,
    #[cfg(feature = "chacha")]
    pub chacha_secret_key: Option<TokenStream>,
    #[cfg(feature = "argon")]
    pub argon2_params: Option<TokenStream>,
    #[cfg(feature = "sql")]
    pub sql: Option<TokenStream>,
}

pub fn get_opt(attributes: &Vec<Attribute>) -> Opt {
    let mut opt = Opt::default();

    for attr in attributes {
        if attr.path().is_ident("opt") {
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    match ident.to_string().as_str() {
                        #[cfg(feature = "builder")]
                        "name" => {
                            if let Ok(ts) = meta.value()?.parse::<proc_macro2::TokenStream>() {
                                opt.name = Some(ts);
                            }
                        }
                        #[cfg(feature = "builder")]
                        "pattern" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.pattern = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "builder")]
                        "err" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.err = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "builder")]
                        "default" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.default = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "builder")]
                        "min" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.min = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "builder")]
                        "max" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.max = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "builder")]
                        "err_min" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.err_min = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "builder")]
                        "err_max" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.err_max = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "cipher")]
                        "aes_secret_key" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.aes_secret_key = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "cipher")]
                        "chacha_secret_key" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.chacha_secret_key = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "cipher")]
                        "argon2_params" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.argon2_params = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "sql")]
                        "sql" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.sql = Some(quote! { #expr });
                            }
                        }
                        /*#[cfg(feature = "path2enum")]
                        "path" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.path = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "path2enum")]
                        "ext" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.ext = Some(quote! { #expr });
                            }
                        }
                        #[cfg(feature = "path2enum")]
                        "prefix" => {
                            if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                                opt.prefix = Some(quote! { #expr });
                            }
                        }*/
                        _ => {}
                    }
                }

                Ok(())
            })
            .unwrap();
        }
    }

    opt
}
