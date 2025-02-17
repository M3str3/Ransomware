use std::env;
use std::error::Error;
use std::fs;

use rsa::RsaPrivateKey;
// To parse a private key in PKCS#8 format (header "-----BEGIN PRIVATE KEY-----")
use rsa::pkcs8::DecodePrivateKey;
// Import OAEP padding and SHA256 digest type
use base64;
use rsa::Oaep;
use sha2::Sha256;

/// Decrypts an AES key using an RSA private key.
///
/// # Arguments
/// - `encrypted_aes_key` - The AES key encrypted with an RSA public key.
/// - `private_key_pem` - The RSA private key in PEM format.
///
/// # Returns
/// - A byte vector containing the decrypted AES key.
fn decrypt_key(encrypted_aes_key: Vec<u8>, private_key_pem: &str) -> Vec<u8> {
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(private_key_pem).expect("Error parsing the private key");

    // Configure OAEP padding scheme with SHA256, the same as used in encryption.
    let padding = Oaep::new::<Sha256>();

    // Decrypt the AES key using the private RSA key and the padding scheme.
    let aes_key = private_key
        .decrypt(padding, &encrypted_aes_key)
        .expect("Error decrypting the AES key");

    return aes_key;
}

fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the private key path and the base64-encoded AES key from environment variables.
    let rsa_private_key_path = env::var("RSA_PRIV_KEY").expect(
        "Define the RSA_PRIV_KEY environment variable with the path to the private key file",
    );
    let aes_key_base64 = env::var("AES_KEY_B64")
        .expect("Define the AES_KEY_B64 environment variable with the base64-encoded string");

    // Read the private key content.
    let private_key_pem =
        fs::read_to_string(&rsa_private_key_path).expect("Error reading the private key file");

    // Decode the base64-encoded AES key.
    let encrypted_aes_key = base64::decode(aes_key_base64).unwrap();

    // Decrypt the AES key.
    let decrypted_aes_key = decrypt_key(encrypted_aes_key, &private_key_pem);

    // Encode the decrypted AES key back into base64 format.
    let aes_key_plain_base64 = base64::encode(decrypted_aes_key);

    // Inject the AES key into the binary through a build-time environment variable.
    println!("cargo:rerun-if-env-changed=AES_KEY_B64");
    println!("cargo:rerun-if-env-changed=RSA_PRIV_KEY");
    println!("cargo:rustc-env=AES_KEY={}", aes_key_plain_base64);

    return Ok(());
}
