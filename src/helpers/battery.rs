use std::process::Command;

pub fn get_battery_info() -> String {
    let output = Command::new("ioreg")
        .args(["-l", "-w0", "-r", "-c", "AppleSmartBattery"])
        .output();

    let stdout = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => return "<unknown>".to_string(),
    };

    let mut device_name = "Built-in".to_string();
    let mut current_capacity: Option<i32> = None;
    let mut external_connected = false;
    let mut is_charging = false;
    let mut avg_time_to_empty: Option<i32> = None;

    for line in stdout.lines() {
        if line.contains("\"DeviceName\"") {
            if let Some(equals_pos) = line.find('=') {
                let value_part = &line[equals_pos + 1..].trim();
                let value = value_part.trim_matches('"').trim_matches(';').trim();
                if !value.is_empty() {
                    device_name = value.to_string();
                }
            }
        } else if line.contains("\"CurrentCapacity\"") {
            if let Some(equals_pos) = line.find('=') {
                let value_part = &line[equals_pos + 1..].trim();
                let value = value_part.trim_matches(';').trim();
                if let Ok(capacity) = value.parse::<i32>() {
                    current_capacity = Some(capacity);
                }
            }
        } else if line.contains("\"ExternalConnected\"") {
            if let Some(equals_pos) = line.find('=') {
                let value_part = &line[equals_pos + 1..].trim();
                let value = value_part.trim_matches(';').trim();
                external_connected = value == "Yes";
            }
        } else if line.contains("\"IsCharging\"") {
            if let Some(equals_pos) = line.find('=') {
                let value_part = &line[equals_pos + 1..].trim();
                let value = value_part.trim_matches(';').trim();
                is_charging = value == "Yes";
            }
        } else if line.contains("\"AvgTimeToEmpty\"") {
            if let Some(equals_pos) = line.find('=') {
                let value_part = &line[equals_pos + 1..].trim();
                let value = value_part.trim_matches(';').trim();
                if let Ok(time) = value.parse::<i32>() {
                    avg_time_to_empty = Some(time);
                }
            }
        }
    }

    let percentage = if let Some(capacity) = current_capacity {
        if capacity >= 0 && capacity <= 100 {
            capacity as u32
        } else {
            return "<unknown>".to_string();
        }
    } else {
        return "<unknown>".to_string();
    };

    let mut status = String::new();
    if external_connected {
        status.push_str("AC connected");
    } else if is_charging {
        status.push_str("Charging");
    } else {
        status.push_str("Discharging");
    }

    let mut result = format!("({}) {}", device_name, crate::output::colors::battery_percent(percentage));

    if !external_connected && !is_charging {
        if let Some(time_mins) = avg_time_to_empty {
            if time_mins > 0 && time_mins < 0xFFFF {
                let hours = time_mins / 60;
                let mins = time_mins % 60;

                if hours > 0 && mins > 0 {
                    result.push_str(&format!(" ({} hours, {} mins remaining)", hours, mins));
                } else if hours > 0 {
                    result.push_str(&format!(" ({} hours remaining)", hours));
                } else if mins > 0 {
                    result.push_str(&format!(" ({} mins remaining)", mins));
                }
            }
        }
    }

    result.push_str(&format!(" [{}]", status));

    result
}
