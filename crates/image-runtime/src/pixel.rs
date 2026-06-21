use std::fs;
use std::path::Path;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use tench_pixel_core::Document;

pub fn read_image_file_as_data_url(path: &str) -> Result<String, String> {
    let data = fs::read(path).map_err(|e| e.to_string())?;
    let mime = mime_from_path(path);
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(data)))
}

pub fn save_data_url_to_path(data_url: &str, path: &str) -> Result<(), String> {
    let bytes = decode_data_url_bytes(data_url)?;
    fs::write(path, bytes).map_err(|e| e.to_string())
}

pub fn data_url_dimensions(data_url: &str) -> Result<(u32, u32), String> {
    let bytes = decode_data_url_bytes(data_url)?;
    if bytes.len() >= 24 && &bytes[0..4] == b"\x89PNG" {
        let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
        return Ok((width, height));
    }
    if bytes.len() >= 2 && &bytes[0..2] == b"\xff\xd8" {
        let mut index = 2;
        while index + 4 < bytes.len() {
            if bytes[index] != 0xff {
                break;
            }
            let marker = bytes[index + 1];
            if (marker == 0xc0 || marker == 0xc1 || marker == 0xc2) && index + 9 < bytes.len() {
                let height = u16::from_be_bytes([bytes[index + 5], bytes[index + 6]]) as u32;
                let width = u16::from_be_bytes([bytes[index + 7], bytes[index + 8]]) as u32;
                return Ok((width, height));
            }
            let segment_length = u16::from_be_bytes([bytes[index + 2], bytes[index + 3]]) as usize;
            index += 2 + segment_length;
        }
    }
    Ok((0, 0))
}

pub fn load_document(path: &str) -> Result<Document, String> {
    image::open(Path::new(path))
        .map(|image| Document::from_image(image, Some(path.to_string())))
        .map_err(|e| e.to_string())
}

pub fn save_document_image(document: &Document, path: &str) -> Result<String, String> {
    let flat = document.flatten();
    let image = flat.to_dynamic_image();
    image.save(path).map_err(|e| e.to_string())?;
    Ok(Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("untitled")
        .to_string())
}

fn decode_data_url_bytes(data_url: &str) -> Result<Vec<u8>, String> {
    let encoded = data_url
        .split_once(',')
        .map(|(_, encoded)| encoded)
        .ok_or_else(|| "Invalid data URL".to_string())?;
    STANDARD
        .decode(encoded)
        .map_err(|e| format!("Failed to decode base64: {e}"))
}

fn mime_from_path(path: &str) -> &'static str {
    let ext = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png")
        .to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        _ => "image/png",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_png_dimensions_from_data_url() {
        let mut bytes = vec![0u8; 24];
        bytes[0..4].copy_from_slice(b"\x89PNG");
        bytes[16..20].copy_from_slice(&640u32.to_be_bytes());
        bytes[20..24].copy_from_slice(&480u32.to_be_bytes());
        let data_url = format!("data:image/png;base64,{}", STANDARD.encode(bytes));

        assert_eq!(data_url_dimensions(&data_url).unwrap(), (640, 480));
    }

    #[test]
    fn save_data_url_to_path_decodes_payload() {
        let dir = std::env::temp_dir().join("tench_pixel_runtime");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("out.bin");
        let data_url = format!("data:image/png;base64,{}", STANDARD.encode(b"pixels"));

        save_data_url_to_path(&data_url, &path.to_string_lossy()).unwrap();

        assert_eq!(fs::read(&path).unwrap(), b"pixels");
        let _ = fs::remove_dir_all(&dir);
    }
}
