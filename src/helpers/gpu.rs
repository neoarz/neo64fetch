// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/gpu/gpu_apple.c

use std::process::Command;

pub fn get_gpu_info() -> String {
    let ioreg_accel = Command::new("ioreg")
        .args(["-rc", "IOAccelerator", "-d", "1"])
        .output();

    let mut model = String::new();
    let mut cores = String::new();
    let mut vendor_id = String::new();

    if let Ok(output) = ioreg_accel {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("\"model\"") {
                model = line
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .replace('"', "")
                    .replace('<', "")
                    .replace('>', "");
            }
            if line.contains("\"gpu-core-count\"") {
                cores = line.split('=').nth(1).unwrap_or("").trim().to_string();
            }
            if line.contains("\"vendor-id\"") {
                vendor_id = line.split('=').nth(1).unwrap_or("").trim().to_string();
            }
        }
    }

    // Apple (0x106b) or Intel (0x8086) = Integrated
    let type_str = match vendor_id.to_lowercase().as_str() {
        "0x106b" | "4203" => "[Integrated]",
        "0x8086" | "32902" => "[Integrated]",
        "0x1002" | "4098" => "[Discrete]",
        "0x10de" | "4318" => "[Discrete]",
        _ => "[Integrated]", // Default for Apple Silicon if vendor-id is weird
    };

    let ioreg_pmgr = Command::new("ioreg")
        .args(["-p", "IODeviceTree", "-n", "pmgr", "-l"])
        .output();

    let mut max_freq = 0u64;
    if let Ok(output) = ioreg_pmgr {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("voltage-states9-sram") {
                let hex_data = line
                    .split('<')
                    .nth(1)
                    .and_then(|s| s.split('>').next())
                    .unwrap_or("");
                for i in (0..hex_data.len()).step_by(16) {
                    if i + 8 <= hex_data.len() {
                        let chunk = &hex_data[i..i + 8];
                        let mut bytes = [0u8; 4];
                        for j in 0..4 {
                            if let Ok(byte) = u8::from_str_radix(&chunk[j * 2..j * 2 + 2], 16) {
                                bytes[j] = byte;
                            }
                        }
                        let freq = u32::from_le_bytes(bytes) as u64;
                        if freq > max_freq {
                            max_freq = freq;
                        }
                    }
                }
            }
        }
    }

    let freq_str = if max_freq > 0 {
        let ghz = if max_freq > 100_000_000 {
            max_freq as f64 / 1_000_000_000.0
        } else {
            max_freq as f64 / 1_000_000.0
        };
        format!(" @ {:.2} GHz", ghz)
    } else {
        "".to_string()
    };

    let core_str = if !cores.is_empty() {
        format!(" ({})", cores)
    } else {
        "".to_string()
    };

    if model.is_empty() {
        "Unknown GPU".to_string()
    } else {
        format!("{}{} {} {}", model, core_str, freq_str, type_str).replace("  ", " ")
    }
}
