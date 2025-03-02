extern crate winapi;

#[cfg(debug_assertions)]
use std::ffi::CStr;
#[cfg(debug_assertions)]
use winapi::um::errhandlingapi::GetLastError;

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::um::fileapi::{CreateFileA, ReadFile, WriteFile, OPEN_ALWAYS, OPEN_EXISTING};
use winapi::um::handleapi::CloseHandle;
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDestroyKey, CryptEncrypt, CryptImportKey, CryptReleaseContext,
    CRYPT_VERIFYCONTEXT, HCRYPTKEY, HCRYPTPROV, PROV_RSA_AES,
};
use winapi::um::winnt::{
    DELETE, FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, FILE_WRITE_DATA, HANDLE,
};

/// Function to encrypt file
pub fn encrypt(source_file: CString, dest_file: CString, aes_key: Vec<u8>) -> bool {
    let mut h_key: HCRYPTKEY = 0usize;
    let mut h_crypt_prov: HCRYPTPROV = 0usize;

    unsafe {
        if CryptAcquireContextA(
            &mut h_crypt_prov,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            #[cfg(debug_assertions)]
            {
                println!(
                    "Error during CryptAcquireContext! Error code: {}",
                    GetLastError()
                );
            }
            return false;
        } else {
            #[cfg(debug_assertions)]
            {
                println!("A cryptographic provider has been acquired.");
            }
        }

        // Import the AES key
        if CryptImportKey(
            h_crypt_prov,
            aes_key.as_ptr(),
            aes_key.len() as u32,
            0,
            0,
            &mut h_key,
        ) == 0
        {
            #[cfg(debug_assertions)]
            {
                println!("Failed to import key, error: {:?}", GetLastError());
            }
            return false;
        } else {
            #[cfg(debug_assertions)]
            {
                println!("Import successful. The key is 0x{:x}", h_key);
            }
        }

        let block_len: u32 = 960;
        let buffer_len: u32 = 960;
        let mut pb_buffer: Vec<u8> = vec![0u8; buffer_len as usize];
        #[cfg(debug_assertions)]
        {
            println!("Memory has been allocated for the buffer.");
        }

        // Open the source file
        let source_handle: HANDLE = CreateFileA(
            source_file.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        // Open the destination file
        let dest_handle: HANDLE = CreateFileA(
            dest_file.as_ptr(),
            FILE_WRITE_DATA | DELETE,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut eof = 0;
        let mut count = 0;

        while eof == 0 {
            if ReadFile(
                source_handle,
                pb_buffer.as_mut_ptr() as *mut _,
                block_len,
                &mut count,
                null_mut(),
            ) == 0
            {
                #[cfg(debug_assertions)]
                {
                    let c_str: &CStr = &source_file;
                    match c_str.to_str() {
                        Ok(str_slice) => println!("Converted CString to str: {}", str_slice),
                        Err(e) => eprintln!("Failed to convert CString to str: {:?}", e),
                    }
                    println!("Error reading");
                }
                break;
            }
            if count < block_len {
                eof = 1;
            }
            if CryptEncrypt(
                h_key,
                0,
                eof,
                0,
                pb_buffer.as_mut_ptr(),
                &mut count,
                buffer_len,
            ) == 0
            {
                #[cfg(debug_assertions)]
                {
                    println!("Failed to encrypt, error: 0x{:x}", GetLastError());
                }
                break;
            }
            if WriteFile(
                dest_handle,
                pb_buffer.as_ptr() as *const _,
                count,
                &mut count,
                null_mut(),
            ) == 0
            {
                #[cfg(debug_assertions)]
                {
                    println!("Failed to write");
                }
                break;
            }
        }
        CloseHandle(source_handle);
        CloseHandle(dest_handle);
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
    }
    true
}
