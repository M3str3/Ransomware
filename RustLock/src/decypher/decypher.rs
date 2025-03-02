use std::ffi::CString;
use std::ptr::null_mut;

#[cfg(debug_assertions)]
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileA, ReadFile, WriteFile, OPEN_ALWAYS, OPEN_EXISTING};
use winapi::um::handleapi::CloseHandle;
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDecrypt, CryptDestroyKey, CryptImportKey, CryptReleaseContext,
    CRYPT_VERIFYCONTEXT, HCRYPTKEY, HCRYPTPROV, PROV_RSA_AES,
};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, FILE_WRITE_DATA, HANDLE,
};

pub fn decrypt(source_file: CString, dest_file: CString, aes_key: Vec<u8>) -> bool {
    let mut h_key: HCRYPTKEY = 0usize; // key
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
                println!("Error during CryptAcquireContext!");
                println!("Error code: {}", GetLastError());
            }
            return false;
        } else {
            #[cfg(debug_assertions)]
            {
                println!("A cryptographic provider has been acquired.");
            }
        }

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
                println!("Import fail {:?}", GetLastError());
            }
            return false;
        } else {
            #[cfg(debug_assertions)]
            {
                println!("Import successful. Key is {}", h_key);
            }
        }

        let src_handle: HANDLE = CreateFileA(
            source_file.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let dest_handle: HANDLE = CreateFileA(
            dest_file.as_ptr(),
            FILE_WRITE_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let block_len: u32 = 960;
        let buffer_len: u32 = 960;

        let mut eof = 0;
        let mut count = 0;

        let mut pb_buffer: Vec<u8> = Vec::new();
        pb_buffer.resize(buffer_len as usize, 0u8);

        while eof == 0 {
            if ReadFile(
                src_handle,
                pb_buffer.as_ptr() as *mut _,
                block_len,
                &mut count,
                null_mut(),
            ) == 0
            {
                #[cfg(debug_assertions)]
                {
                    println!("Error reading 0x{:x}", GetLastError());
                }
                break;
            }

            if count < block_len {
                eof = 1;
            }

            if CryptDecrypt(h_key, 0, eof, 0, pb_buffer.as_mut_ptr(), &mut count) == 0 {
                #[cfg(debug_assertions)]
                {
                    println!("Fail to decrypt 0x{:x}", GetLastError());
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
                    println!("Fail to write");
                }
                break;
            }
        }
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
        CloseHandle(src_handle);
        CloseHandle(dest_handle);
    }
    true
}
