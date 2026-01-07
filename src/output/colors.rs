use colored::Colorize;

// title
pub fn title(user: &str, host: &str) -> String {
    format!("{}@{}", user.cyan().bold(), host.cyan().bold())
}

// separator
pub fn separator(len: usize) -> String {
    "-".repeat(len)
}

// percent (for colors based on usage level)
pub fn percent(value: f64) -> String {
    let text = format!("{}%", value as u32);
    if value > 80.0 {
        text.red().to_string()
    } else if value > 50.0 {
        text.yellow().to_string()
    } else {
        text.green().to_string()
    }
}

// battery is different because its inverted (low is bad)
pub fn battery_percent(value: u32) -> String {
    let text = format!("{}%", value);
    if value < 20 {
        text.red().to_string()
    } else if value < 50 {
        text.yellow().to_string()
    } else {
        text.green().to_string()
    }
}

// info
pub fn info(key: &str, value: &str) -> String {
    format!("{}: {}", key.yellow().bold(), value)
}

// color blocks
pub fn color_blocks() -> String {
    let normal = "   ".on_black().to_string()
        + &"   ".on_red().to_string()
        + &"   ".on_green().to_string()
        + &"   ".on_yellow().to_string()
        + &"   ".on_blue().to_string()
        + &"   ".on_magenta().to_string()
        + &"   ".on_cyan().to_string()
        + &"   ".on_white().to_string();

    let bright = "   ".on_bright_black().to_string()
        + &"   ".on_bright_red().to_string()
        + &"   ".on_bright_green().to_string()
        + &"   ".on_bright_yellow().to_string()
        + &"   ".on_bright_blue().to_string()
        + &"   ".on_bright_magenta().to_string()
        + &"   ".on_bright_cyan().to_string()
        + &"   ".on_bright_white().to_string();

    format!("{}\n{}", normal, bright)
}
