use std::env;
use sysinfo::System;

pub fn get_terminal_info() -> String {
    let term_env = env::var("TERM_PROGRAM")
        .or_else(|_| env::var("TERM"))
        .unwrap_or_default();

    let term_ver = env::var("TERM_PROGRAM_VERSION").unwrap_or_default();

    if !term_env.is_empty() && term_env != "xterm-256color" && term_env != "xterm" {
        let clean_name = term_env
            .replace("com.apple.", "") // Apple Terminal
            .replace("com.mitchellh.", "") // Ghostty
            .replace(".app", "");

        return if term_ver.is_empty() {
            clean_name
        } else {
            format!("{} {}", clean_name, term_ver)
        };
    }

    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let my_pid = sysinfo::get_current_pid().unwrap_or(sysinfo::Pid::from(0));

    if let Some(process) = sys.process(my_pid)
        && let Some(parent_pid) = process.parent()
        && let Some(parent_proc) = sys.process(parent_pid)
    {
        return parent_proc.name().to_string_lossy().replace(".app", "");
    }

    "unknown".to_string()
}
