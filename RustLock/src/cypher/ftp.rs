use std::fs::File;
use std::io::Read;
use suppaftp::FtpStream;

pub fn upload_file(
    ftp_server: &str,
    remote_path: &str,
    local_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ftp_stream = FtpStream::connect(ftp_server)?;

    // Iniciar sesión como usuario anónimo
    ftp_stream.login("anonymous", "anonymous@domain.com")?;

    // Extraer la carpeta y el nombre del archivo
    let parts: Vec<&str> = remote_path.rsplitn(2, '/').collect();
    let remote_folder = parts.get(1).unwrap_or(&"/"); // Carpeta remota
    let file_name = parts.get(0).unwrap(); // Nombre del archivo

    // Intentar crear la carpeta (sin error si ya existe)
    ftp_stream.cwd("/")?; // Ir al directorio raíz
    for folder in remote_folder.split('/') {
        if !folder.is_empty() {
            let _ = ftp_stream.mkdir(folder);
            ftp_stream.cwd(folder)?; // Entrar en la carpeta
        }
    }

    // Leer el archivo local
    let mut file = File::open(local_file)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Subir el archivo
    ftp_stream.put_file(file_name, &mut &contents[..])?;

    // Cerrar sesión
    ftp_stream.quit()?;

    Ok(())
}
