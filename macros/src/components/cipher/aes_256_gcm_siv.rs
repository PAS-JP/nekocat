use super::prelude::*;

pub fn cipher_aes_256_gcm_siv(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let field_type = &field.ty;
    let aes_256_gcm_siv_encrypt_ident = format_ident!("aes_256_gcm_siv_encrypt_{field_ident}");
    let aes_256_gcm_siv_decrypt_ident = format_ident!("aes_256_gcm_siv_decrypt_{field_ident}");
    let aes_256_gcm_siv_key_and_nonce_ga_ident =
        format_ident!("aes_256_gcm_siv_key_and_nonce_ga_{field_ident}");
    let impl_block = get_impl(input);
    let Opt { aes_secret_key, .. } = get_opt(&field.attrs);
    let aes_secret_key = match aes_secret_key {
        Some(ts) => quote! { #ts },
        None => quote! { "SECRET" },
    };
    quote! {
        impl #impl_block {
            fn #aes_256_gcm_siv_key_and_nonce_ga_ident(nonce: &[u8]) -> Result<(
                    nekocat::aes_gcm_siv::aead::generic_array::GenericArray<u8, nekocat::aes_gcm_siv::aead::generic_array::typenum::U32>,
                    nekocat::aes_gcm_siv::aead::generic_array::GenericArray<u8, nekocat::aes_gcm_siv::aead::generic_array::typenum::U12>
                ),
                String> {
                use nekocat::aes_gcm_siv::aead::{Aead, KeyInit};
                use std::convert::TryInto;
                use nekocat::aes_gcm_siv::aead::generic_array::GenericArray;
                use nekocat::aes_gcm_siv::aead::generic_array::typenum::{U32, U12};

                let secret_hex = std::env::var(#aes_secret_key)
                    .map_err(|_| format!("Environment variable {} not found", #aes_secret_key))?;
                let key_bytes = nekocat::hex::decode(&secret_hex)
                    .map_err(|e| format!("Invalid hex key: {}", e))?;
                let key_array: [u8; 32] = key_bytes
                    .try_into()
                    .map_err(|_| "AES-256 key must be exactly 32 bytes (64 hex characters)")?;
                 let nonce: [u8; 12] = nonce
                    .try_into()
                    .map_err(|e| format!("invalid nonce length: {e}"))?;

                let key_ga = *GenericArray::from_slice(&key_array);
                let nonce_ga = *GenericArray::from_slice(&nonce);

                Ok((key_ga, nonce_ga))
            }

              pub fn #aes_256_gcm_siv_encrypt_ident(&self) -> Result<(Vec<u8>, Vec<u8>), String>
            {
                use nekocat::aes_gcm_siv::aead::{Aead, KeyInit};
                use nekocat::rand::RngExt;
                use nekocat::rkyv::rancor::Error as RkyvError;

                let value = &self.#field_ident;
                let nonce_rand = nekocat::rand::rng().random::<[u8; 12]>().to_vec();
                let plaintext: Vec<u8> = nekocat::rkyv::to_bytes::<RkyvError>(value)
                    .map_err(|e| e.to_string())
                    .map(|v| v.into())?;

                let (key, nonce) = Self::#aes_256_gcm_siv_key_and_nonce_ga_ident(&nonce_rand)?;
                let ciphertext = nekocat::aes_gcm_siv::Aes256GcmSiv::new(&key)
                    .encrypt(&nonce, plaintext.as_slice())
                    .map_err(|err| err.to_string())?;
                Ok((ciphertext, nonce_rand))
            }

            pub fn #aes_256_gcm_siv_decrypt_ident(ciphertext: Vec<u8>, nonce: Vec<u8>) -> Result<#field_type, String>
            {
                use nekocat::aes_gcm_siv::aead::{Aead, KeyInit};
                use nekocat::rkyv::rancor::Error as RkyvError;

                let (key, nonce) = Self::#aes_256_gcm_siv_key_and_nonce_ga_ident(&nonce)?;
                let decrypted = nekocat::aes_gcm_siv::Aes256GcmSiv::new(&key)
                    .decrypt(&nonce, ciphertext.as_ref())
                    .map_err(|err| err.to_string())?;

                let archived = nekocat::rkyv::access::<
                    <#field_type as nekocat::rkyv::Archive>::Archived,
                    RkyvError
                >(&decrypted[..])
                    .map_err(|e| e.to_string())?;

                let decoded: #field_type = nekocat::rkyv::deserialize::<#field_type, RkyvError>(archived)
                    .map_err(|e| e.to_string())?;

                Ok(decoded)
            }
        }
    }
}
