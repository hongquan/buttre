//! UI update helper functions
//!
//! This module contains helper functions for updating menu and tray icons
//! to avoid code duplication in the main event loop.

use crate::shared::ui::{load_icon_from_bytes, load_menu_icon, CHECK_ICON_BYTES};
use buttre_core::vietnamese::config_loader::MethodMetadata;
use buttre_core::vietnamese::get_custom_dir;
use muda::{IconMenuItem, Submenu};
use std::fs;
use tray_icon::{Icon as TrayIcon, TrayIcon as TrayIconType};

/// Update menu checkmarks for the given method
///
/// # Algorithm
/// 1. Clear all existing checkmarks
/// 2. Set checkmark on the active method
pub fn update_menu_checkmarks(
    method: &str,
    english_item: &IconMenuItem,
    _chu_viet_menu: &Submenu,
    telex_item: &IconMenuItem,
    vni_item: &IconMenuItem,
    nom_item: &IconMenuItem,
    custom_items: &[(MethodMetadata, IconMenuItem)],
) {
    // Clear all checkmarks
    english_item.set_icon(None);
    telex_item.set_icon(None);
    vni_item.set_icon(None);
    nom_item.set_icon(None);
    for (_, item) in custom_items {
        item.set_icon(None);
    }

    // NOTE: Submenu provided by muda crate does not support set_icon.
    // User requested not to use text prefix hack ("✓"), so we leave parent menu unchecked for now.

    // Set checkmark on active method
    if let Some(check_icon) = load_menu_icon(CHECK_ICON_BYTES) {
        match method {
            "english" => {
                english_item.set_icon(Some(check_icon));
            }
            "telex" => {
                telex_item.set_icon(Some(check_icon));
            }
            "vni" => {
                vni_item.set_icon(Some(check_icon));
            }
            "nom" => {
                nom_item.set_icon(Some(check_icon));
            }
            _ => {
                // Search in custom methods
                for (data, item) in custom_items {
                    if data.id == method {
                        item.set_icon(Some(check_icon.clone()));
                        break;
                    }
                }
            }
        }
    }
}

/// Update tray icon and tooltip for the given method
///
/// # Algorithm
/// 1. If disabled, show English icon
/// 2. Otherwise, show icon for the active method
/// 3. For custom methods, try to load custom icon from file
///
/// One parameter per method icon — grouping into a struct is possible but
/// out of scope for a lint cleanup (would ripple through every call site in
/// UI init code that isn't covered by an automated test).
#[allow(clippy::too_many_arguments)]
pub fn update_tray_icon(
    method: &str,
    enabled: bool,
    tray_icon: &mut TrayIconType,
    telex_icon: &TrayIcon,
    vni_icon: &TrayIcon,
    english_icon: &TrayIcon,
    nom_icon: &TrayIcon,
    custom_icon: &TrayIcon,
    custom_items: &[(MethodMetadata, IconMenuItem)],
) {
    if !enabled {
        let _ = tray_icon.set_icon(Some(english_icon.clone()));
        let _ = tray_icon.set_tooltip(Some("buttre\nOFF".to_string()));
        return;
    }

    match method {
        "telex" => {
            let _ = tray_icon.set_icon(Some(telex_icon.clone()));
            let _ = tray_icon.set_tooltip(Some("buttre\nChữ Việt\nTELEX".to_string()));
        }
        "vni" => {
            let _ = tray_icon.set_icon(Some(vni_icon.clone()));
            let _ = tray_icon.set_tooltip(Some("buttre\nChữ Việt\nVNI".to_string()));
        }
        "nom" => {
            let _ = tray_icon.set_icon(Some(nom_icon.clone()));
            let _ = tray_icon.set_tooltip(Some("buttre\nChữ Nôm".to_string()));
        }
        _ => {
            // Handle custom methods
            let mut custom_icon_loaded = false;
            let mut name = method.to_string();

            if let Some((data, _)) = custom_items.iter().find(|(d, _)| d.id == method) {
                name = data.name.clone();
                if let Some(icon_path_str) = &data.icon {
                    let icon_path = get_custom_dir().join(icon_path_str);
                    if let Ok(bytes) = fs::read(&icon_path) {
                        if let Ok(icon) = load_icon_from_bytes(&bytes) {
                            let _ = tray_icon.set_icon(Some(icon));
                            custom_icon_loaded = true;
                        }
                    }
                }
            }

            if !custom_icon_loaded {
                let _ = tray_icon.set_icon(Some(custom_icon.clone()));
            }

            let _ = tray_icon.set_tooltip(Some(format!("buttre\nCustom\n{}", name)));
        }
    }
}
