extern crate winapi;

use crate::decypher::decypher;

use std::ffi::CString;
use std::fs;
use std::io;
use std::path::Path;
use std::ptr::null_mut;
use std::str;

use winapi::um::fileapi::DeleteFileA;
use winapi::um::winbase::GetUserNameA;

/// Traverses the specified directories and calls `traverse_and_decrypt_path`
/// with the AES key and the ransomware extension to perform the decryption
///
/// # Arguments
/// * `directories`   - List of directory names (for example, "Documents", "Downloads")
/// * `aes_key`       - AES key used for decryption
/// * `encrypted_ext` - Extension used by the ransomware-encrypted files
pub fn walk_decrypt(directories: &[&str], aes_key: &[u8], encrypted_ext: &str) {
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();

    unsafe {
        // Retrieve the size of the user name and store it in 'size'
        GetUserNameA(null_mut(), &mut size);

        // Resize the buffer according to the size, then retrieve the user name
        buffer.resize(size as usize, 0i8);
        GetUserNameA(buffer.as_mut_ptr(), &mut size);

        // Convert the buffer to a byte vector and remove the null terminator
        let mut user_name: Vec<u8> = std::mem::transmute(buffer);
        user_name.resize((size - 1) as usize, 0u8); // Adjust size to remove the null terminator

        // Iterate over each directory in the 'directories' slice
        for dir in directories.iter() {
            // Construct the full path: C:\Users\<username>\<directory>
            let mut full_path = String::from("C:\\Users\\");
            full_path.push_str(str::from_utf8(&user_name[..]).unwrap());
            full_path.push_str("\\");
            full_path.push_str(dir);

            let full_path_cs: CString = CString::new(full_path.as_bytes()).unwrap();
            println!("Processing: {}", full_path_cs.to_str().unwrap());

            // Call the function to traverse and decrypt the files, passing in the AES key and the ransomware extension
            let _ = traverse_and_decrypt_path(&full_path, aes_key, encrypted_ext);
        }
    }
}

/// Recursively traverses the files within `directory_path` and decrypts
/// those that have the `encrypted_ext` extension using the provided AES key
///
/// # Arguments
/// * `directory_path` - Path to the directory to be traversed
/// * `aes_key`        - AES key used for decryption
/// * `encrypted_ext`  - Extension used by the ransomware-encrypted files
pub fn traverse_and_decrypt_path(directory_path: &str, aes_key: &[u8], encrypted_ext: &str) -> io::Result<()> {
    let directory = Path::new(directory_path);

    if directory.is_dir() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            // If this is a subdirectory, go deeper
            if path.is_dir() {
                let subdir_str = path.to_str().unwrap_or("");
                traverse_and_decrypt_path(subdir_str, aes_key, encrypted_ext)?;
            } else {
                // Check if the file has the ransomware extension
                if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                    if extension == encrypted_ext {
                        let original_path = path.to_str().unwrap_or("");
                        let mut decrypted_path = path.clone();
                        decrypted_path.set_extension(""); // Remove the ransomware extension
                        let decrypted_path_str = decrypted_path.to_str().unwrap_or("");

                        println!("Decrypting: {} -> {}", original_path, decrypted_path_str);

                        let c_original = CString::new(original_path)
                            .expect("Failed to create CString for source path");
                        let c_decrypted = CString::new(decrypted_path_str)
                            .expect("Failed to create CString for destination path");

                        // Call the decrypt function with the AES key
                        let result = decypher::decrypt(c_original.clone(), c_decrypted, aes_key.to_vec());
                        if !result {
                            println!("Failed to decrypt: {}", original_path);
                        } else {
                            // Remove the encrypted file after successful decryption
                            let _delete_result = unsafe { DeleteFileA(c_original.as_ptr()) };
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
