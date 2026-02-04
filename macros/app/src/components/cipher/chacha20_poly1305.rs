use super::prelude::*;

pub fn cipher_chacha20_poly1305(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let field_type = &field.ty;
    let chacha20_poly1305_encrypt_ident = format_ident!("chacha20_poly1305_encrypt_{field_ident}");
    let chacha20_poly1305_decrypt_ident = format_ident!("chacha20_poly1305_decrypt_{field_ident}");
    let chacha20_poly1305_key_and_nonce_ga_ident =
        format_ident!("chacha20_poly1305_key_and_nonce_ga_{field_ident}");
    let impl_block = get_impl(input);
    let Opt {
        chacha_secret_key, ..
    } = get_opt(&field.attrs);
    let chacha_secret_key = match chacha_secret_key {
        Some(ts) => quote! { #ts },
        None => panic!("#[opt(chacha_secret_key = Some(String::from(\"Must be provided\")))]"),
    };
    quote! {
        impl #impl_block {
            fn #chacha20_poly1305_key_and_nonce_ga_ident(nonce: &[u8]) -> Result<(
                    chacha20poly1305::aead::generic_array::GenericArray<u8, chacha20poly1305::aead::generic_array::typenum::U32>,
                    chacha20poly1305::aead::generic_array::GenericArray<u8, chacha20poly1305::aead::generic_array::typenum::U12>
                ),
                String> {
                use chacha20poly1305::aead::{Aead, KeyInit};
                use std::convert::TryInto;
                use chacha20poly1305::aead::generic_array::GenericArray;
                use chacha20poly1305::aead::generic_array::typenum::{U32, U12};

                let key: Vec<u8> = #chacha_secret_key
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

              pub fn #chacha20_poly1305_encrypt_ident(&self) -> Result<(Vec<u8>, Vec<u8>), String>
            {
                use chacha20poly1305::aead::{Aead, KeyInit};
                use rand::Rng;
                use rkyv::rancor::Error as RkyvError;

                let value = &self.#field_ident;
                let nonce_rand = rand::rng().random::<[u8; 12]>().to_vec();

                let plaintext: Vec<u8> = rkyv::to_bytes::<RkyvError>(value)
                    .map_err(|e| e.to_string())
                    .map(|v| v.into())?;

                let (key, nonce) =
                    Self::#chacha20_poly1305_key_and_nonce_ga_ident(&nonce_rand)?;

                let ciphertext = chacha20poly1305::ChaCha20Poly1305::new(&key)
                    .encrypt(&nonce, plaintext.as_slice())
                    .map_err(|err| err.to_string())?;

                Ok((ciphertext, nonce_rand))
            }

            pub fn #chacha20_poly1305_decrypt_ident(
                ciphertext: Vec<u8>,
                nonce: Vec<u8>
            ) -> Result<#field_type, String>
            {
                use chacha20poly1305::aead::{Aead, KeyInit};
                use rkyv::rancor::Error as RkyvError;

                let (key, nonce) =
                    Self::#chacha20_poly1305_key_and_nonce_ga_ident(&nonce)?;

                let decrypted_bytes =
                    chacha20poly1305::ChaCha20Poly1305::new(&key)
                        .decrypt(&nonce, ciphertext.as_ref())
                        .map_err(|err| err.to_string())?;

                let archived = rkyv::access::<
                    <#field_type as rkyv::Archive>::Archived,
                    RkyvError
                >(&decrypted_bytes[..])
                    .map_err(|e| e.to_string())?;

                let decoded: #field_type =
                    rkyv::deserialize::<#field_type, RkyvError>(archived)
                        .map_err(|e| e.to_string())?;

                Ok(decoded)
            }
        }
    }
}
