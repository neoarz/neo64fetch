// neoarz
// neo64fetch - "jarvis, rewrite this project in rust"

use std::env;
use sysinfo::System;

mod helpers;
mod output;

use output::colors;

struct Stats {
    // Neoarz[at]Mac
    username: String,
    hostname: String,
    // --------------
    os: String,
    host: String,
    kernel: String,
    uptime: String,
    packages: String,
    shell: String,
    display: String,
    desktop_env: String,
    window_manager: String,
    window_manager_theme: String,
    font: String,
    cursor: String,
    terminal: String,
    terminal_font: String,
    cpu: String,
    gpu: String,
    memory: String,
    swap: String,
    storage: String,
    ip: String,
    battery: String,
    locale: String,

    // Extra fields
    architecture: String,
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let stats = Stats {
        hostname: System::host_name().unwrap_or_else(|| "<unknown>".to_owned()),
        // This would be the real username of the system but I just want to print out Neoarz for my case
        // Uncoment the line below to use the real username
        /*
        username: env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "<unknown>".to_owned()),
        */
        username: "Neoarz".to_owned(),
        os: System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()),
        host: helpers::host::get_host_info(),
        architecture: System::cpu_arch(),
        kernel: System::kernel_long_version(),
        uptime: helpers::uptime::get_uptime(System::uptime()),
        packages: helpers::packages::get_brew_info(),
        shell: helpers::shell::get_shell_info(),
        display: helpers::display::get_display_info(),
        desktop_env: helpers::desktop_env::get_desktop_env_info(),
        window_manager: helpers::wm::get_window_manager_info().wm_pretty_name,
        window_manager_theme: helpers::wm_theme::get_wm_theme_info(),
        font: helpers::font::get_font_info(),
        cursor: helpers::cursor::get_cursor_info(),
        terminal: helpers::terminal::get_terminal_info(),
        terminal_font: helpers::terminal_font::get_terminal_font_info(),
        cpu: helpers::cpu::get_cpu_info(),
        gpu: helpers::gpu::get_gpu_info(),
        memory: helpers::memory::get_memory_info(),
        swap: helpers::swap::get_swap_info(),
        storage: helpers::storage::get_storage_info(),
        ip: helpers::ip::get_ip_info(),
        battery: helpers::battery::get_battery_info(),
        locale: helpers::locale::get_locale_info(),
    };

    // user@host
    println!("{}", colors::title(&stats.username, &stats.hostname));

    // separator
    println!("{}", colors::separator(stats.username.len() + stats.hostname.len() + 1));

    // info
    println!("{}", colors::info("OS", &format!("{} {}", stats.os, stats.architecture)));
    println!("{}", colors::info("Host", &stats.host));
    println!("{}", colors::info("Kernel", &stats.kernel));
    println!("{}", colors::info("Uptime", &stats.uptime));
    println!("{}", colors::info("Packages", &stats.packages));
    println!("{}", colors::info("Shell", &stats.shell));
    println!("{}", colors::info("Display", &stats.display));
    println!("{}", colors::info("DE", &stats.desktop_env));
    println!("{}", colors::info("WM", &stats.window_manager));
    println!("{}", colors::info("WM Theme", &stats.window_manager_theme));
    println!("{}", colors::info("Font", &stats.font));
    println!("{}", colors::info("Cursor", &stats.cursor));
    println!("{}", colors::info("Terminal", &stats.terminal));
    println!("{}", colors::info("Terminal Font", &stats.terminal_font));
    println!("{}", colors::info("CPU", &stats.cpu));
    println!("{}", colors::info("GPU", &stats.gpu));
    println!("{}", colors::info("Memory", &stats.memory));
    println!("{}", colors::info("Swap", &stats.swap));
    println!("{}", colors::info("Disk (/)", &stats.storage));
    // Don't wanna show print this lolol
    // println!("{}", colors::info("Local IP", &stats.ip));
    println!("{}", colors::info("Battery", &stats.battery));
    println!("{}", colors::info("Locale", &stats.locale));

    // color blocks
    println!();
    println!("{}", colors::color_blocks());
}
