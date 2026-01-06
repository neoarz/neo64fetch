// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/theme/theme_apple.c

use sysinfo::System;

pub fn get_desktop_env_info() -> String {
    let os_version = System::os_version().unwrap_or_else(|| "0.0.0".to_string());
    let major_version: u32 = os_version
        .split('.')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    if major_version > 15 {
        "Liquid Glass".to_string()
    } else if major_version < 10 && major_version != 0 {
        "Platinum".to_string()
    } else if major_version >= 10 {
        "Aqua".to_string()
    } else {
        "Unknown".to_string()
    }
}
