mod antireversing;
mod config;
mod decypher;

// ===================================================================================
// AES Key Recovery Process
// ===================================================================================
// The `build.rs` script restore the key from the base64 string on ransom note
// The AES key is stored as a base64-encoded string in a build-time environment variable
const AES_KEY: &str = env!("AES_KEY");

fn main() {
    // ===================================================================================
    // STEP 1: Anti-Reversing Checks
    // ===================================================================================
    // This module attempts to detect debugging, reverse engineering, or tampering attempts
    // If any suspicious activity is detected, the program may terminate or trigger countermeasures
    antireversing::anti_reversing();

    // ===================================================================================
    // STEP 2: Decoding the AES Key
    // ===================================================================================
    // The AES key is originally stored in base64 format for safe storage and transport
    // Here, we decode it back into its raw binary format for use in decryption
    let aes_key =
        base64::decode(AES_KEY.trim()).expect("Error decoding AES key from base64");

    #[cfg(debug_assertions)]
    {
        println!("AES Key (Base64): {}", AES_KEY);
        println!("Decoded AES Key: {:?}", aes_key);
        println!("AES Key Size: {} bytes", aes_key.len());
    }

    // ===================================================================================
    // STEP 3: Decrypting Files in Target Directories
    // ===================================================================================
    // This step walks through predefined directories and decrypts files that match
    // the specified ransomware extension (`config::RANSOM_EXT`)
    // Any file with the target extension will be restored to its original form
    decypher::walker::walk_decrypt(&config::DIR_NAMES, &aes_key, *config::RANSOM_EXT);
}
