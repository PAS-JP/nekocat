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
        None => panic!("#[opt(aes_secret_key = Some(String::from(\"Must be provided\")))]"),
    };
    quote! {
        impl #impl_block {
            fn #aes_256_gcm_siv_key_and_nonce_ga_ident(nonce: &[u8]) -> Result<(
                    aes_gcm_siv::aead::generic_array::GenericArray<u8, aes_gcm_siv::aead::generic_array::typenum::U32>,
                    aes_gcm_siv::aead::generic_array::GenericArray<u8, aes_gcm_siv::aead::generic_array::typenum::U12>
                ),
                String> {
                use aes_gcm_siv::aead::{Aead, KeyInit};
                use std::convert::TryInto;
                use aes_gcm_siv::aead::generic_array::GenericArray;
                use aes_gcm_siv::aead::generic_array::typenum::{U32, U12};

                let key: Vec<u8> = #aes_secret_key
                    .as_ref()
                    .map(|s| hex::decode(s).unwrap_or_default())
                    .unwrap_or_default();
                let key: [u8; 32] = key
                    .as_slice()
                    .try_into()
                    .map_err(|e| format!("invalid key length: {e}"))?;
                 let nonce: [u8; 12] = nonce
                    .try_into()
                    .map_err(|e| format!("invalid nonce length: {e}"))?;

                let key_ga = *GenericArray::from_slice(&key);
                let nonce_ga = *GenericArray::from_slice(&nonce);

                Ok((key_ga, nonce_ga))
            }

              pub fn #aes_256_gcm_siv_encrypt_ident(&self) -> Result<(Vec<u8>, Vec<u8>), String>
            {
                use aes_gcm_siv::aead::{Aead, KeyInit};
                use rand::Rng;
                use rkyv::rancor::Error as RkyvError;

                let value = &self.#field_ident;
                let nonce_rand = rand::rng().random::<[u8; 12]>().to_vec();
                let plaintext: Vec<u8> = rkyv::to_bytes::<RkyvError>(value)
                    .map_err(|e| e.to_string())
                    .map(|v| v.into())?;

                let (key, nonce) = Self::#aes_256_gcm_siv_key_and_nonce_ga_ident(&nonce_rand)?;
                let ciphertext = aes_gcm_siv::Aes256GcmSiv::new(&key)
                    .encrypt(&nonce, plaintext.as_slice())
                    .map_err(|err| err.to_string())?;
                Ok((ciphertext, nonce_rand))
            }

            pub fn #aes_256_gcm_siv_decrypt_ident(ciphertext: Vec<u8>, nonce: Vec<u8>) -> Result<#field_type, String>
            {
                use aes_gcm_siv::aead::{Aead, KeyInit};
                use rkyv::rancor::Error as RkyvError;

                let (key, nonce) = Self::#aes_256_gcm_siv_key_and_nonce_ga_ident(&nonce)?;
                let decrypted = aes_gcm_siv::Aes256GcmSiv::new(&key)
                    .decrypt(&nonce, ciphertext.as_ref())
                    .map_err(|err| err.to_string())?;

                let archived = rkyv::access::<
                    <#field_type as rkyv::Archive>::Archived,
                    RkyvError
                >(&decrypted[..])
                    .map_err(|e| e.to_string())?;

                let decoded: #field_type = rkyv::deserialize::<#field_type, RkyvError>(archived)
                    .map_err(|e| e.to_string())?;

                Ok(decoded)
            }
        }
    }
}
