use super::prelude::*;

pub fn cipher_pqcrypto(input: &DeriveInput, field: &Field) -> TokenStream {
    let struct_name = get_struct_name(input);
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let field_type = &field.ty;
    let _pqcrypto_send_chacha_ident = format_ident!("pqcrypto_send_chacha_{field_ident}");
    let _pqcrypto_receive_chacha_ident = format_ident!("pqcrypto_receive_chacha_{field_ident}");
    let pqcrypto_send_aes_gcm_siv_ident = format_ident!("pqcrypto_send_aes_gcm_siv_{field_ident}");
    let pqcrypto_receive_aes_gcm_siv_ident =
        format_ident!("pqcrypto_receive_aes_gcm_siv_{field_ident}");
    let pqcrypto_send_response = format_ident!("PqCryptoSend{struct_name}Response");
    let pqcrypto_session_keys = format_ident!("PqCryptoSession{struct_name}Keys");
    let impl_block = get_impl(input);

    quote! {
        impl #impl_block {
            pub fn #pqcrypto_send_aes_gcm_siv_ident(
                &self,
                hkdf_info: &[u8],
                sender_session_keys: #pqcrypto_session_keys,
                recipient_pkeys: (pqcrypto_mlkem::mlkem1024::PublicKey, pqcrypto_mldsa::mldsa87::PublicKey)
            ) -> Result<#pqcrypto_send_response, Box<dyn std::error::Error>> {
                use aes_gcm_siv::aead::{Aead, KeyInit};
                use std::convert::TryInto;
                use rand::Rng;
                use hkdf::Hkdf;
                use sha2::{Sha256, Digest};
                use rkyv::rancor::Error as RkyvError;
                use aes_gcm_siv::aead::generic_array::GenericArray;
                use aes_gcm_siv::aead::generic_array::typenum::{U32, U12};
                use pqcrypto_mlkem::mlkem1024::{PublicKey as MlkemPublicKey, SecretKey as MlkemSecretKey, encapsulate};
                use pqcrypto_mldsa::mldsa87::{PublicKey as MldsaPublicKey, SecretKey as MldsaSecretKey, detached_sign, open, sign};
                use pqcrypto_traits::kem::{PublicKey as KemPublicKey, SecretKey as KemSecretKey, Ciphertext, SharedSecret};
                use pqcrypto_traits::sign::{PublicKey as SignPublicKey, SecretKey as SignSecretKey, DetachedSignature, SignedMessage};

                let #pqcrypto_session_keys {
                    mlkem_pk: sender_mlkem_pk,
                    mlkem_sk: sender_mlkem_sk,
                    mldsa_pk: sender_mldsa_pk,
                    mldsa_sk: sender_mldsa_sk,
                } = sender_session_keys;

                let (recipient_mlkem_pk, recipient_mldsa_pk) = recipient_pkeys;

                let (shared_secret, kem_ciphertext) = encapsulate(&recipient_mlkem_pk);

                let message: Vec<u8>  = rkyv::to_bytes::<RkyvError>(&self.#field_ident)
                    .map_err(|e| e.to_string())
                    .map(|v| v.into())?;

                let sender_mldsa_signature = detached_sign(&message, &sender_mldsa_sk);
                let nonce_bytes = rand::rng().random::<[u8; 12]>().to_vec();

                let transcript = #impl_block::gen_transcript(
                    sender_mlkem_pk.as_bytes(),
                    sender_mldsa_pk.as_bytes(),
                    recipient_mlkem_pk.as_bytes(),
                    recipient_mldsa_pk.as_bytes(),
                    kem_ciphertext.as_bytes(),
                    sender_mldsa_signature.as_bytes(),
                );

                let hkdf_salt: [u8; 32] = Sha256::digest(&transcript).into();

                let hk = Hkdf::<Sha256>::new(
                    Some(&hkdf_salt),
                    shared_secret.as_bytes(),
                );

                let mut key = [0u8; 32];
                 hk.expand(hkdf_info, &mut key)
                    .map_err(|_| "invalid hk key")?;

                let nonce: [u8; 12] = nonce_bytes
                    .as_slice()
                    .try_into()
                    .map_err(|_| "invalid nonce length mismatch")?;


                let key = *GenericArray::from_slice(&key);
                let nonce = *GenericArray::from_slice(&nonce);

                let aes_cipher_message = aes_gcm_siv::Aes256GcmSiv::new(&key)
                    .encrypt(&nonce, message.as_slice())
                    .map_err(|e| format!("aes-gcm-siv error: {:?}", e))?;


                Ok(#pqcrypto_send_response {
                    nonce: nonce_bytes.to_vec(),
                    kem_ciphertext: kem_ciphertext.as_bytes().to_vec(),
                    aes_cipher_message,
                    sender_mldsa_signature
                })
            }
            pub fn #pqcrypto_receive_aes_gcm_siv_ident(
                &self,
                hkdf_info: &[u8],
                recipient_session_keys: #pqcrypto_session_keys,
                sender_pkeys: (pqcrypto_mlkem::mlkem1024::PublicKey, pqcrypto_mldsa::mldsa87::PublicKey),
                response: #pqcrypto_send_response
            ) -> Result<#field_type, Box<dyn std::error::Error>> {
                use aes_gcm_siv::aead::{Aead, KeyInit};
                use std::convert::TryInto;
                use hkdf::Hkdf;
                use sha2::{Sha256, Digest};
                use rkyv::rancor::Error as RkyvError;
                use aes_gcm_siv::aead::generic_array::GenericArray;
                use aes_gcm_siv::aead::generic_array::typenum::{U32, U12};
                use pqcrypto_mlkem::mlkem1024::{PublicKey as MlkemPublicKey, SecretKey as MlkemSecretKey, decapsulate};
                use pqcrypto_mldsa::mldsa87::{PublicKey as MldsaPublicKey, SecretKey as MldsaSecretKey, detached_sign, open, sign, verify_detached_signature};
                use pqcrypto_traits::kem::{PublicKey as KemPublicKey, SecretKey as KemSecretKey, Ciphertext, SharedSecret};
                use pqcrypto_traits::sign::{PublicKey as SignPublicKey, SecretKey as SignSecretKey, DetachedSignature, SignedMessage};
                use rkyv::from_bytes;

                let #pqcrypto_session_keys {
                    mlkem_pk: recipient_mlkem_pk,
                    mlkem_sk: recipient_mlkem_sk,
                    mldsa_pk: recipient_mldsa_pk,
                    mldsa_sk: _recipient_mldsa_sk,
                } = recipient_session_keys;

                let (sender_mlkem_pk, sender_mldsa_pk) = sender_pkeys;
                let kem_ciphertext = pqcrypto_traits::kem::Ciphertext::from_bytes(&response.kem_ciphertext)
                    .map_err(|_| "invalid kem ciphertext")?;
                let shared_secret = decapsulate(&kem_ciphertext, &recipient_mlkem_sk);

                let transcript = #impl_block::gen_transcript(
                    sender_mlkem_pk.as_bytes(),
                    sender_mldsa_pk.as_bytes(),
                    recipient_mlkem_pk.as_bytes(),
                    recipient_mldsa_pk.as_bytes(),
                    kem_ciphertext.as_bytes(),
                    response.sender_mldsa_signature.as_bytes(),
                );

                let hkdf_salt: [u8; 32] = Sha256::digest(&transcript).into();

                let hk = Hkdf::<Sha256>::new(
                    Some(&hkdf_salt),
                    shared_secret.as_bytes(),
                );

                let mut key = [0u8; 32];
                hk.expand(hkdf_info, &mut key)
                    .map_err(|_| "invalid hk key")?;

                let nonce: [u8; 12] = response.nonce
                    .as_slice()
                    .try_into()
                    .map_err(|_| "invalid nonce length mismatch")?;

                let key = *GenericArray::from_slice(&key);
                let nonce = *GenericArray::from_slice(&nonce);

                let plaintext = aes_gcm_siv::Aes256GcmSiv::new(&key)
                    .decrypt(&nonce, response.aes_cipher_message.as_slice())
                    .map_err(|_| "aes-gcm-siv decrypt failed")?;

                verify_detached_signature(
                    &response.sender_mldsa_signature,
                    &plaintext,
                    &sender_mldsa_pk
                ).map_err(|_| "invalid mldsa signature")?;

                let obj: #field_type = from_bytes::<#field_type, RkyvError>(&plaintext)
                    .map_err(|e| e.to_string())?;

                Ok(obj)
            }
        }
    }
}
