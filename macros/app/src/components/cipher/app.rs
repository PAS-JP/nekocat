use super::prelude::*;

pub fn cipher_app(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_clone = input.clone();
    let struct_name = get_struct_name(input);
    let impl_block = get_impl(input);
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;
    let field_methods: Vec<_> = fields
        .iter()
        .map(|f| cipher_field_methods(input, f))
        .collect();
    let pqcrypto_send_response = format_ident!("PqCryptoSend{struct_name}Response");
    let pqcrypto_session_keys = format_ident!("PqCryptoSession{struct_name}Keys");

    quote! {
        pub struct #pqcrypto_send_response {
            pub nonce: Vec<u8>,
            pub kem_ciphertext: Vec<u8>,
            pub aes_cipher_message: Vec<u8>,
            pub sender_mldsa_signature: pqcrypto_mldsa::mldsa87::DetachedSignature
        }

        pub struct #pqcrypto_session_keys {
            pub mlkem_pk: pqcrypto_mlkem::mlkem1024::PublicKey,
            pub mlkem_sk: pqcrypto_mlkem::mlkem1024::SecretKey,
            pub mldsa_pk: pqcrypto_mldsa::mldsa87::PublicKey,
            pub mldsa_sk: pqcrypto_mldsa::mldsa87::SecretKey,
        }

        impl #impl_block {
            pub fn gen_transcript(
                sender_mlkem_pk: &[u8],
                sender_mldsa_pk: &[u8],
                recipient_mlkem_pk: &[u8],
                recipient_mldsa_pk: &[u8],
                kem_ciphertext: &[u8],
                mldsa_signature: &[u8],
            ) -> Vec<u8> {
                let mut transcript = Vec::new();

                transcript.extend_from_slice(sender_mlkem_pk);
                transcript.extend_from_slice(sender_mldsa_pk);
                transcript.extend_from_slice(recipient_mlkem_pk);
                transcript.extend_from_slice(recipient_mldsa_pk);
                transcript.extend_from_slice(kem_ciphertext);
                transcript.extend_from_slice(mldsa_signature);

                transcript
            }

            pub fn gen_pqcrypto_keypairs() -> #pqcrypto_session_keys {
                use pqcrypto_mlkem::mlkem1024::keypair as mlkem_key_pair;
                use pqcrypto_mldsa::mldsa87::keypair as mldsa_key_pair;

                let (mlkem_pk, mlkem_sk) = mlkem_key_pair();
                let (mldsa_pk, mldsa_sk) = mldsa_key_pair();

                #pqcrypto_session_keys {
                    mlkem_pk,
                    mlkem_sk,
                    mldsa_pk,
                    mldsa_sk,
                }
            }
        }
        #(#field_methods)*
    }
}
