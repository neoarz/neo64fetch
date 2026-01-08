use std::env;
use std::process::Command;

pub fn get_shell_info() -> String {
    let shell_path = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let shell_name = shell_path.split('/').next_back().unwrap_or("unknown");

    Command::new(&shell_path)
        .arg("--version")
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Most shells return "zsh 5.9 (x86_64-apple-darwin24.0)" or similar; we just want the first two words
            stdout
                .split_whitespace()
                .take(2)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_else(|_| shell_name.to_string())
}
