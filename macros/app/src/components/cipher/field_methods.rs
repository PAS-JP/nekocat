use super::prelude::*;

pub fn cipher_field_methods(input: &DeriveInput, field: &Field) -> TokenStream {
    /*let mlkem1024_mldsa87_encrypt_ident = format_ident!("mlkem1024_mldsa87_encrypt_{field_ident}");
    let mlkem1024_mldsa87_decrypt_ident = format_ident!("mlkem1024_mldsa87_decrypt_{field_ident}");*/

    let pqcrypto = cipher_pqcrypto(input, field);
    let aes_256_gcm_siv = cipher_aes_256_gcm_siv(input, field);
    let chacha20_poly1305 = cipher_chacha20_poly1305(input, field);
    let argon2 = cipher_argon2(input, field);

    quote! {
        #aes_256_gcm_siv
        #chacha20_poly1305
        #argon2
        #pqcrypto
    }
}
