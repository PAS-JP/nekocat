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
                argon2::Params::new(32 * 1024, 3, 1, 32.into()).map_err(|e| e.to_string())
            }
        }
    };
    quote! {
        impl #impl_block {
            fn #argon2_params_ident() -> Result<argon2::Params, String> {
               #argon2_params
            }

            pub fn #argon2_hash_ident(&self) -> Result<String, String>
            where
                #field_type: AsRef<[u8]>,
            {
                use argon2::{Argon2, Algorithm, Version};
                use argon2::password_hash::SaltString;
                use argon2::PasswordHash;
                use argon2::PasswordHasher;
                use argon2::password_hash::rand_core::OsRng;

                let params = Self::#argon2_params_ident()?;
                let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
                let salt = SaltString::generate(&mut OsRng);
                let password_bytes: &[u8] = self.#field_ident.as_ref();

                let password_hash = argon2
                    .hash_password(password_bytes, &salt)
                    .map_err(|e| e.to_string())?;

                Ok(password_hash.to_string())
            }

            pub fn #argon2_verify_ident(hash: String, password: impl Into<Vec<u8>>) -> Result<bool, String> {
                use argon2::{Argon2, Algorithm, Version};
                use argon2::password_hash::SaltString;
                use argon2::PasswordVerifier;
                use argon2::PasswordHash;
                use argon2::PasswordHasher;

                let params = Self::#argon2_params_ident()?;
                let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

                let parsed = PasswordHash::new(&hash).map_err(|e| e.to_string())?;

                match argon2.verify_password(password.into().as_ref(), &parsed) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
        }
    }
}
