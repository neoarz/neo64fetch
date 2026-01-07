// neoarz
// neo64fetch - "jarvis, rewrite this project in rust"

// use colored::*;
use std::env;
use sysinfo::System;
mod helpers;

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
        username: env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "<unknown>".to_owned()),
        os: System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()),
        host: System::name().unwrap_or_else(|| "<unknown>".to_owned()),
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

    // TODO: Add ascii art support later
    // Testing each component separately; going to comment out at the end
    {
        println!("{}", stats.username);
        println!("{}", stats.hostname);
        println!("{}", stats.os);
        println!("{}", stats.host);
        println!("{}", stats.architecture);
        println!("{}", stats.kernel);
        println!("{}", stats.uptime);
        println!("{}", stats.packages);
        println!("{}", stats.shell);
        println!("{}", stats.display);
        println!("{}", stats.desktop_env);
        println!("{}", stats.window_manager);
        println!("{}", stats.window_manager_theme);
        println!("{}", stats.font);
        println!("{}", stats.cursor);
        println!("{}", stats.terminal);
        println!("{}", stats.terminal_font);
        println!("{}", stats.cpu);
        println!("{}", stats.gpu);
        println!("{}", stats.memory);
        println!("{}", stats.swap);
        println!("{}", stats.storage);
        println!("{}", stats.ip);
        println!("{}", stats.battery);
        println!("{}", stats.locale);
    }
}
