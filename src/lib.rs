use std::io::{Cursor, Read};
use zip::ZipArchive;

pub fn try_password(password: &str, zip_data: &[u8]) -> bool {
    let cursor = Cursor::new(zip_data);
    let mut archive = match ZipArchive::new(cursor) {
        Ok(archive) => archive,
        Err(_) => return false,
    };

    if archive.len() == 0 {
        return false;
    }

    let mut file = match archive.by_index_decrypt(0, password.as_bytes()) {
        Ok(file) => file,
        Err(_) => return false,
    };

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).is_ok()
}
