// neoarz
// neo64fetch - "jarvis, rewrite this project in rust"

use std::env;
use sysinfo::System;

mod helpers;
mod output;

use output::{colors, image};

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
    battery: (String, String), // (device_name, info)
    locale: String,

    // Extra fields
    architecture: String, // appended to os
}


fn get_system_stats() -> Stats {
    let mut sys = System::new_all();
    sys.refresh_all();

    Stats {
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
    }
}


fn print_stats(stats: &Stats, offset: usize) {
    let mut lines = Vec::new();

    // user@host
    lines.push(colors::title(&stats.username, &stats.hostname));

    // separator
    lines.push(colors::separator(stats.username.len() + stats.hostname.len() + 1));

    // info
    lines.push(colors::info("OS", &format!("{} {}", stats.os, stats.architecture)));
    lines.push(colors::info("Host", &stats.host));
    lines.push(colors::info("Kernel", &stats.kernel));
    lines.push(colors::info("Uptime", &stats.uptime));
    lines.push(colors::info("Packages", &stats.packages));
    lines.push(colors::info("Shell", &stats.shell));
    lines.push(colors::info("Display", &stats.display));
    // lines.push(colors::info("DE", &stats.desktop_env));
    // lines.push(colors::info("WM", &stats.window_manager));
    // lines.push(colors::info("WM Theme", &stats.window_manager_theme));
    lines.push(colors::info("Font", &stats.font));
    lines.push(colors::info("Cursor", &stats.cursor));
    lines.push(colors::info("Terminal", &stats.terminal));
    lines.push(colors::info("Terminal Font", &stats.terminal_font));
    lines.push(colors::info("CPU", &stats.cpu));
    lines.push(colors::info("GPU", &stats.gpu));
    lines.push(colors::info("Memory", &stats.memory));
    lines.push(colors::info("Swap", &stats.swap));
    lines.push(colors::info("Disk (/)", &stats.storage));
    // lines.push(colors::info("Local IP", &stats.ip));
    lines.push(colors::info(&format!("Battery {}", stats.battery.0), &stats.battery.1));
    // lines.push(colors::info("Locale", &stats.locale));

    // color blocks
    lines.push(String::new());
    for line in colors::color_blocks().lines() {
        lines.push(line.to_string());
    }

    for line in lines {
        image::offset_println!(offset, "{}", line);
    }
}



fn main() {
    let stats = get_system_stats();                                                         
    let (offset, img_rows) = image::print_image_and_setup("assets/logo.png", 700);
    //                                                                       ^^^ size of the image change it here                  
    print_stats(&stats, offset);
    image::finish_printing(offset, 24, img_rows);
}