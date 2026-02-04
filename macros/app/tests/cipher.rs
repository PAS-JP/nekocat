#![cfg(feature = "cipher")]
use neko_cat_macros::Cipher;
use rkyv::{Archive, Deserialize, Serialize};

const SECRET: &str = "50e637e1245ee65705409472d7bbf32a27e760ef332d4a19aebca96328a53cfb";

#[derive(Cipher, Archive, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct Secret {
    #[opt(
        aes_secret_key=Some(SECRET.to_string()),
        chacha_secret_key=Some(SECRET.to_string())
    )]
    pwd: String,
}

#[derive(Cipher, Archive, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[rkyv(compare(PartialEq), derive(Debug))]
pub struct User {
    #[opt(
        aes_secret_key=Some(SECRET.to_string()),
        chacha_secret_key=Some(SECRET.to_string())
    )]
    name: String,

    #[opt(
        aes_secret_key=Some(SECRET.to_string()),
        chacha_secret_key=Some(SECRET.to_string())
    )]
    password: String,
}

#[test]
fn aes_256_gcm_siv_user_encrypt_and_decrypt_name_and_password() {
    let user = User {
        name: String::from("aes_name"),
        password: String::from("aes_password"),
    };
    let (ct_name, nonce_name) = user.aes_256_gcm_siv_encrypt_name().unwrap();
    assert!(!ct_name.is_empty(), "ciphertext should not be empty");
    assert_eq!(nonce_name.len(), 12, "nonce must be 12 bytes");
    let dec_name: String =
        User::aes_256_gcm_siv_decrypt_name(ct_name.clone(), nonce_name.clone()).unwrap();
    assert_eq!(dec_name, user.name);

    let (ct_pw, nonce_pw) = user.aes_256_gcm_siv_encrypt_password().unwrap();
    assert!(!ct_pw.is_empty());
    assert_eq!(nonce_pw.len(), 12);
    let dec_pw: String =
        User::aes_256_gcm_siv_decrypt_password(ct_pw.clone(), nonce_pw.clone()).unwrap();
    assert_eq!(dec_pw, user.password);

    assert_ne!(ct_name, ct_pw);
}

#[test]
fn chacha20_poly1305_user_encrypt_and_decrypt_name_and_password() {
    let user = User {
        name: String::from("chacha_name"),
        password: String::from("chacha_password"),
    };

    let (ct_name, nonce_name) = user.chacha20_poly1305_encrypt_name().unwrap();
    assert!(!ct_name.is_empty());
    assert_eq!(
        nonce_name.len(),
        12,
        "nonce must be 12 bytes for ChaCha20-Poly1305 (this expects 12)"
    );
    let dec_name: String =
        User::chacha20_poly1305_decrypt_name(ct_name.clone(), nonce_name.clone()).unwrap();
    assert_eq!(dec_name, user.name);

    let (ct_pw, nonce_pw) = user.chacha20_poly1305_encrypt_password().unwrap();
    assert!(!ct_pw.is_empty());
    assert_eq!(nonce_pw.len(), 12);
    let dec_pw: String =
        User::chacha20_poly1305_decrypt_password(ct_pw.clone(), nonce_pw.clone()).unwrap();
    assert_eq!(dec_pw, user.password);

    assert_ne!(ct_name, ct_pw);
}

#[test]
fn argon2_hash_and_verify_password() {
    let user = User {
        name: "example".to_string(),
        password: "supersecret".to_string(),
    };

    let hash = user
        .argon2_hash_password()
        .expect("hash should be generated");

    let verified = User::argon2_verify_password(hash.clone(), user.password)
        .expect("verification should succeed");
    assert!(verified);

    let wrong_password = "wrongpassword".to_string();
    let verified_wrong = User::argon2_verify_password(hash, wrong_password)
        .expect("verification should return false, not error");
    assert!(!verified_wrong);
}
