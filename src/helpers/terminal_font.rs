// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/terminalfont/terminalfont.c

use std::process::Command;

pub fn get_terminal_font_info() -> String {
    let output = Command::new("ghostty").arg("+show-config").output();

    let mut font_family = String::new();
    let mut font_size = String::new();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);

        for line in stdout.lines() {
            if line.starts_with("font-family =") && font_family.is_empty() {
                font_family = line.replace("font-family =", "").trim().to_string();
            }
            if line.starts_with("font-size =") && font_size.is_empty() {
                font_size = line.replace("font-size =", "").trim().to_string();
            }
        }
    }

    if font_family.is_empty() {
        font_family = "JetBrainsMono Nerd Font".to_string();
    }
    if font_size.is_empty() {
        font_size = "13".to_string();
    }

    format!("{} Regular ({}pt)", font_family, font_size)
}
