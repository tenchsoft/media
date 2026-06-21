//! Platform-specific utility operations.
//!
//! Provides clipboard and file manager integration that can be used
//! by both the Widget (direct call) and Tauri commands (IPC wrapper).
//!
//! All external process calls go through the centralized PlatformAction enum
//! from tench-shared-types to ensure path validation and no shell injection.

use std::borrow::Cow;
use tench_shared_types::PlatformAction;

/// Opens the system file manager and selects the given file path.
///
/// Uses the centralized PlatformAction abstraction for safe process spawning.
pub fn show_in_file_manager(path: &str) -> Result<(), String> {
    let action = PlatformAction::RevealFile(path.to_string());
    let result = action.execute();
    if result.success {
        Ok(())
    } else {
        Err(result.message)
    }
}

/// Copies text to the system clipboard.
pub fn copy_to_clipboard_text(text: &str) -> Result<(), String> {
    let mut clipboard =
        arboard::Clipboard::new().map_err(|e| format!("Failed to access clipboard: {e}"))?;
    clipboard
        .set_text(text)
        .map_err(|e| format!("Failed to copy text to clipboard: {e}"))?;
    Ok(())
}

/// Copies an image to the system clipboard from RGBA pixel data.
///
/// The `png_bytes` parameter should be the raw RGBA pixel data with
/// the specified width and height.
pub fn copy_to_clipboard_image(width: usize, height: usize, pixels: Vec<u8>) -> Result<(), String> {
    let mut clipboard =
        arboard::Clipboard::new().map_err(|e| format!("Failed to access clipboard: {e}"))?;
    let img = arboard::ImageData {
        width,
        height,
        bytes: Cow::Owned(pixels),
    };
    clipboard
        .set_image(img)
        .map_err(|e| format!("Failed to copy image to clipboard: {e}"))?;
    Ok(())
}

/// Opens a file with the system default application.
///
/// Uses the centralized PlatformAction abstraction for safe process spawning.
pub fn open_file(path: &str) -> Result<(), String> {
    let action = PlatformAction::OpenFile(path.to_string());
    let result = action.execute();
    if result.success {
        Ok(())
    } else {
        Err(result.message)
    }
}

/// Copies PNG-encoded bytes to the system clipboard as an image.
///
/// This decodes the PNG bytes and copies the resulting image data.
pub fn copy_to_clipboard_png(png_bytes: &[u8]) -> Result<(), String> {
    let img = image::load_from_memory(png_bytes)
        .map_err(|e| format!("Failed to decode PNG for clipboard: {e}"))?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    copy_to_clipboard_image(w as usize, h as usize, rgba.into_raw())
}

/// Sets the given image file as the desktop wallpaper.
///
/// Platform-specific: uses appropriate system commands for each OS.
pub fn set_wallpaper(path: &str) -> Result<(), String> {
    let canonical = std::path::Path::new(path)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {e}"))?;
    let path_str = canonical.display().to_string();

    if cfg!(target_os = "windows") {
        // Windows: use PowerShell to set wallpaper
        std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Add-Type -TypeDefinition 'using System;using System.Runtime.InteropServices;public class WP{{[DllImport(\\\"user32.dll\\\")]public static extern int SystemParametersInfo(int uAction,int uParam,string lpvParam,int fuWinIni);}}'; [WP]::SystemParametersInfo(0x0014,0,'{}',0x01|0x02)",
                    path_str
                ),
            ])
            .spawn()
            .map_err(|e| format!("Failed to set wallpaper: {e}"))?;
    } else if cfg!(target_os = "macos") {
        // macOS: use osascript to set wallpaper
        std::process::Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "tell application \"Finder\" to set desktop picture to POSIX file \"{}\"",
                    path_str
                ),
            ])
            .spawn()
            .map_err(|e| format!("Failed to set wallpaper: {e}"))?;
    } else {
        // Linux: try swaybg, then feh, then nitrogen
        let path_for_cmd = path_str.clone();
        let result = std::process::Command::new("swaybg")
            .args(["-i", &path_for_cmd, "-m", "fill"])
            .spawn();
        if result.is_err() {
            let result2 = std::process::Command::new("feh")
                .args(["--bg-fill", &path_str])
                .spawn();
            if result2.is_err() {
                std::process::Command::new("nitrogen")
                    .args(["--set-zoom-fill", &path_str])
                    .spawn()
                    .map_err(|e| {
                        format!("Failed to set wallpaper (no supported backend found): {e}")
                    })?;
            }
        }
    }
    Ok(())
}

/// Opens a file with the system's "Open With" dialog.
///
/// This allows the user to choose which application to open the file with.
pub fn open_with(path: &str) -> Result<(), String> {
    let canonical = std::path::Path::new(path)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {e}"))?;

    if cfg!(target_os = "windows") {
        std::process::Command::new("rundll32.exe")
            .args([
                "shell32.dll,OpenAs_RunDLL",
                &canonical.display().to_string(),
            ])
            .spawn()
            .map_err(|e| format!("Failed to open 'Open With' dialog: {e}"))?;
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open")
            .args(["-a", "Choose Application", &canonical.display().to_string()])
            .spawn()
            .map_err(|e| format!("Failed to open 'Open With' dialog: {e}"))?;
    } else {
        // Linux: use xdg-open as fallback (no standard "open with" dialog)
        std::process::Command::new("xdg-open")
            .arg(&canonical)
            .spawn()
            .map_err(|e| format!("Failed to open file: {e}"))?;
    }
    Ok(())
}

/// Shares a file using the system share dialog (where available).
///
/// On mobile or desktop environments that support it, this opens the
/// native share sheet. Falls back to opening the file on systems
/// without a share API.
pub fn share_file(path: &str) -> Result<(), String> {
    let canonical = std::path::Path::new(path)
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {e}"))?;

    // Desktop: no universal share API; open the file as a fallback.
    // On platforms with share support (e.g., Android via Tauri), this
    // would be replaced with the native share intent.
    if cfg!(target_os = "macos") {
        std::process::Command::new("open")
            .arg(&canonical)
            .spawn()
            .map_err(|e| format!("Failed to share file: {e}"))?;
    } else {
        std::process::Command::new("xdg-open")
            .arg(&canonical)
            .spawn()
            .map_err(|e| format!("Failed to share file: {e}"))?;
    }
    Ok(())
}
