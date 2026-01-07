// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/cpu/cpu_apple.c

use std::process::Command;

pub fn get_cpu_info() -> String {
    let brand = Command::new("sysctl")
        .args(["-n", "machdep.cpu.brand_string"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown CPU".to_string());

    let cores = Command::new("sysctl")
        .args(["-n", "hw.physicalcpu"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let ioreg_output = Command::new("ioreg")
        .args(["-p", "IODeviceTree", "-n", "pmgr", "-l"])
        .output();

    let mut max_freq = 0u64;

    if let Ok(output) = ioreg_output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("voltage-states5-sram") {
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

    if max_freq == 0 {
        return format!("{} ({})", brand, cores);
    }

    let ghz = if max_freq > 100_000_000 {
        max_freq as f64 / 1_000_000_000.0
    } else {
        max_freq as f64 / 1_000_000.0
    };

    format!("{} ({}) @ {:.2} GHz", brand, cores, ghz)
}
