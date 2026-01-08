// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/cursor/cursor_apple.m

use plist::Value;
use std::env;
use std::path::PathBuf;

pub fn get_cursor_info() -> String {
    let mut path = PathBuf::from(env::var("HOME").unwrap_or_default());
    path.push("Library/Preferences/com.apple.universalaccess.plist");

    let mut fill = "Black".to_string();
    let mut outline = "White".to_string();
    let mut size = "32".to_string();

    if let Ok(value) = Value::from_file(path)
        && let Some(dict) = value.as_dictionary()
    {
        if let Some(f_dict) = dict.get("cursorFill").and_then(|v| v.as_dictionary()) {
            let r =
                (f_dict.get("red").and_then(|v| v.as_real()).unwrap_or(0.0) * 255.0 + 0.5) as u32;
            let g =
                (f_dict.get("green").and_then(|v| v.as_real()).unwrap_or(0.0) * 255.0 + 0.5) as u32;
            let b =
                (f_dict.get("blue").and_then(|v| v.as_real()).unwrap_or(0.0) * 255.0 + 0.5) as u32;
            let a =
                (f_dict.get("alpha").and_then(|v| v.as_real()).unwrap_or(1.0) * 255.0 + 0.5) as u32;
            let color_hex = (r << 24) | (g << 16) | (b << 8) | a;
            fill = match color_hex {
                0x000000FF => "Black".to_string(),
                0xFFFFFFFF => "White".to_string(),
                0xFF2600FF => "Red".to_string(),
                0x0433FFFF => "Blue".to_string(),
                0x00F900FF => "Green".to_string(),
                0xFFFB00FF => "Yellow".to_string(),
                _ => format!("#{:08X}", color_hex),
            };
        }

        if let Some(o_dict) = dict.get("cursorOutline").and_then(|v| v.as_dictionary()) {
            let r =
                (o_dict.get("red").and_then(|v| v.as_real()).unwrap_or(0.0) * 255.0 + 0.5) as u32;
            let g =
                (o_dict.get("green").and_then(|v| v.as_real()).unwrap_or(0.0) * 255.0 + 0.5) as u32;
            let b =
                (o_dict.get("blue").and_then(|v| v.as_real()).unwrap_or(0.0) * 255.0 + 0.5) as u32;
            let a =
                (o_dict.get("alpha").and_then(|v| v.as_real()).unwrap_or(1.0) * 255.0 + 0.5) as u32;
            let color_hex = (r << 24) | (g << 16) | (b << 8) | a;
            outline = match color_hex {
                0x000000FF => "Black".to_string(),
                0xFFFFFFFF => "White".to_string(),
                0xFF2600FF => "Red".to_string(),
                0x0433FFFF => "Blue".to_string(),
                0x00F900FF => "Green".to_string(),
                0xFFFB00FF => "Yellow".to_string(),
                _ => format!("#{:08X}", color_hex),
            };
        }

        if let Some(s_val) = dict.get("mouseDriverCursorSize").and_then(|v| v.as_real()) {
            size = format!("{:.0}", s_val * 32.0);
        }
    }

    format!("Fill - {}, Outline - {} ({}px)", fill, outline, size)
}
