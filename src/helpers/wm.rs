// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/displayserver/displayserver_apple.c

use plist::Value;
use std::path::Path;

pub struct DisplayServerResult {
    pub wm_pretty_name: String,
}

pub fn get_window_manager_info() -> DisplayServerResult {
    let mut result = DisplayServerResult {
        wm_pretty_name: "Quartz Compositor".to_string(),
    };

    if cfg!(target_os = "macos") {
        let plist_path = "/System/Library/CoreServices/WindowManager.app/Contents/version.plist";

        if Path::new(plist_path).exists()
            && let Ok(value) = Value::from_file(plist_path)
            && let Some(dict) = value.as_dictionary()
            && let Some(raw_version) = dict.get("SourceVersion").and_then(|v| v.as_string())
        {
            // Apple format: AAAABBBCCDDDDDD (Major, Minor, Patch, Build)
            if raw_version.len() >= 8 && raw_version.chars().all(|c| c.is_numeric()) {
                let major = raw_version[..raw_version.len() - 12].trim_start_matches('0');
                let minor = raw_version[raw_version.len() - 12..raw_version.len() - 9]
                    .trim_start_matches('0');
                let patch = raw_version[raw_version.len() - 9..raw_version.len() - 7]
                    .trim_start_matches('0');

                let m = if minor.is_empty() { "0" } else { minor };
                let p = if patch.is_empty() { "0" } else { patch };

                result.wm_pretty_name = format!("Quartz Compositor {}.{}.{}", major, m, p);
            } else {
                result.wm_pretty_name = format!("Quartz Compositor {}", raw_version);
            }
        }
    }

    result
}
