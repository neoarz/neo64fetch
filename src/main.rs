// neoarz
// neo64fetch - "jarvis, rewrite this project in rust"

// use colored::*;
use display_info::DisplayInfo;
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

    // Extra fields which are usually appended
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
    };

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
    }
}
