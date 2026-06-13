//! Icon loading utilities for tray and menu icons

use anyhow::Result;
use tray_icon::Icon as TrayIcon;

/// Embedded icon bytes
pub const VIETNAMESE_ICON_BYTES: &[u8] = include_bytes!("../../../icons/vietnamese.png");
pub const ENGLISH_ICON_BYTES: &[u8] = include_bytes!("../../../icons/english.png");
pub const CHECK_ICON_BYTES: &[u8] = include_bytes!("../../../icons/check.png");
pub const CUSTOM_ICON_BYTES: &[u8] = include_bytes!("../../../icons/custom.png");

// Input method specific icons
pub const TELEX_ICON_BYTES: &[u8] = include_bytes!("../../../icons/telex.png");
pub const VNI_ICON_BYTES: &[u8] = include_bytes!("../../../icons/vni.png");
pub const NOM_ICON_BYTES: &[u8] = include_bytes!("../../../icons/nom.png");

/// Load a tray icon from embedded bytes
pub fn load_icon_from_bytes(bytes: &[u8]) -> Result<TrayIcon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(bytes)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(TrayIcon::from_rgba(icon_rgba, icon_width, icon_height)?)
}

/// Load a menu icon from embedded bytes
pub fn load_menu_icon(bytes: &[u8]) -> Option<muda::Icon> {
    let image = image::load_from_memory(bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    muda::Icon::from_rgba(rgba, width, height).ok()
}
