// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/wmtheme/wmtheme_apple.m

use plist::Value;
use std::env;
use std::path::PathBuf;

pub fn get_wm_theme_info() -> String {
    let mut path = PathBuf::from(env::var("HOME").unwrap_or_else(|_| "".to_string()));
    path.push("Library/Preferences/.GlobalPreferences.plist");

    let mut accent_name = "Multicolor".to_string();
    let mut appearance = "Light".to_string();

    if let Ok(value) = Value::from_file(path)
        && let Some(dict) = value.as_dictionary()
    {
        if let Some(accent_val) = dict
            .get("AppleAccentColor")
            .and_then(|v| v.as_signed_integer())
        {
            accent_name = match accent_val {
                -1 => "Graphite".to_string(),
                0 => "Red".to_string(),
                1 => "Orange".to_string(),
                2 => "Yellow".to_string(),
                3 => "Green".to_string(),
                4 => "Blue".to_string(),
                5 => "Purple".to_string(),
                6 => "Pink".to_string(),
                _ => "Multicolor".to_string(),
            };
        }

        if let Some(style) = dict.get("AppleInterfaceStyle").and_then(|v| v.as_string()) {
            appearance = style.to_string(); // Usually "Dark"
        }
    }

    format!("{} ({})", accent_name, appearance)
}
