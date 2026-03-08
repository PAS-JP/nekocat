use super::prelude::*;

pub fn cipher_argon2(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let field_type = &field.ty;
    let argon2_hash_ident = format_ident!("argon2_hash_{field_ident}");
    let argon2_verify_ident = format_ident!("argon2_verify_{field_ident}");
    let argon2_params_ident = format_ident!("argon2_params_{field_ident}");
    let impl_block = get_impl(input);
    let Opt { argon2_params, .. } = get_opt(&field.attrs);
    let argon2_params = match argon2_params {
        Some(ts) => quote! { #ts },
        None => {
            quote! {
               nekocat::argon2::Params::new(32 * 1024, 3, 1, Some(32))
                    .map_err(|e: nekocat::argon2::Error| e.to_string())
            }
        }
    };
    let Opt {
        argon2_secret_pepper,
        ..
    } = get_opt(&field.attrs);
    let argon2_secret_pepper = match argon2_secret_pepper {
        Some(ts) => quote! { #ts },
        None => quote! { "SECRET" },
    };
    quote! {
        impl #impl_block {
            fn #argon2_params_ident() -> Result<nekocat::argon2::Params, String> {
               #argon2_params
            }

            pub fn #argon2_hash_ident(&self) -> Result<String, String>
            where
                #field_type: AsRef<[u8]>,
            {
                use nekocat::argon2::{Argon2, Algorithm, Version};
                use nekocat::argon2::password_hash::SaltString;
                use nekocat::argon2::PasswordHash;
                use nekocat::argon2::PasswordHasher;
                use nekocat::argon2::password_hash::rand_core::OsRng;

                let params = Self::#argon2_params_ident()?;
                let pepper = std::env::var(#argon2_secret_pepper).expect("Argon2 env pepper must be provided");
                let secret = pepper.as_bytes();

               let argon2 = Argon2::new_with_secret(
                    secret,
                    Algorithm::Argon2id,
                    Version::V0x13,
                    params
                ).map_err(|e| e.to_string())?;

                let salt = SaltString::generate(&mut OsRng);
                let password_bytes: &[u8] = self.#field_ident.as_ref();

                let password_hash = argon2
                    .hash_password(password_bytes, &salt)
                    .map_err(|e| e.to_string())?;

                Ok(password_hash.to_string())
            }

           pub fn #argon2_verify_ident(hash: String, password: impl Into<Vec<u8>>) -> Result<bool, String> {
                use nekocat::argon2::{Argon2, Algorithm, Version};
                use nekocat::argon2::PasswordVerifier;
                use nekocat::argon2::PasswordHash;

                let params = Self::#argon2_params_ident()?;

                let pepper = std::env::var(#argon2_secret_pepper)
                    .map_err(|_| format!("Argon2 env pepper {} must be provided", #argon2_secret_pepper))?;
                let secret = pepper.as_bytes();

                let argon2 = Argon2::new_with_secret(
                    secret,
                    Algorithm::Argon2id,
                    Version::V0x13,
                    params
                ).map_err(|e| e.to_string())?;

                let parsed = PasswordHash::new(&hash).map_err(|e| e.to_string())?;

                match argon2.verify_password(password.into().as_ref(), &parsed) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
        }
    }
}
