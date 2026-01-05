pub fn get_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if days > 0 {
        format!("{} days, {} hours, {} minutes", days, hours, minutes)
    } else {
        format!("{} hours, {} minutes", hours, minutes)
    }
}
