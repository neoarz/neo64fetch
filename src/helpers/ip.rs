use std::process::Command;

pub fn get_ip_info() -> String {
    let mut interface = "en0".to_string();

    let route_output = Command::new("route").args(["get", "default"]).output();

    if let Ok(output) = route_output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.trim().starts_with("interface:")
                && let Some(iface) = line.split_whitespace().nth(1)
            {
                interface = iface.to_string();
                break;
            }
        }
    }

    let ifconfig_output = Command::new("ifconfig").arg(&interface).output();

    if let Ok(output) = ifconfig_output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("inet ") && !line.contains("127.0.0.1") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let ip = parts[1];
                    let mut ip_with_cidr = ip.to_string();

                    if let Some(netmask_idx) = parts.iter().position(|&x| x == "netmask")
                        && netmask_idx + 1 < parts.len()
                    {
                        let netmask_hex = parts[netmask_idx + 1];
                        if netmask_hex.starts_with("0x")
                            && let Ok(num) = u32::from_str_radix(&netmask_hex[2..], 16)
                        {
                            let cidr = num.count_ones();
                            ip_with_cidr = format!("{}/{}", ip, cidr);
                        }
                    }

                    return format!("({}) {}", interface, ip_with_cidr);
                }
            }
        }
    }

    "<unknown>".to_string()
}
