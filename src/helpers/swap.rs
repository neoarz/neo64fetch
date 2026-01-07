// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/swap/swap_apple.c

use std::process::Command;

pub fn get_swap_info() -> String {
    let output = Command::new("sysctl").args(["-n", "vm.swapusage"]).output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);

        let mut total_mb = 0.0;
        let mut used_mb = 0.0;

        for part in stdout.split_whitespace() {
            if let Some(val) = part.strip_suffix('M') {
                if let Ok(num) = val.parse::<f64>() {
                    if stdout.contains(&format!("total = {}", part)) {
                        total_mb = num;
                    } else if stdout.contains(&format!("used = {}", part)) {
                        used_mb = num;
                    }
                }
            }
        }

        let total_gib = total_mb / 1024.0;
        let used_gib = used_mb / 1024.0;
        let percentage = if total_mb > 0.0 {
            (used_mb / total_mb) * 100.0
        } else {
            0.0
        };

        return format!(
            "{:.2} GiB / {:.2} GiB ({})",
            used_gib, total_gib, crate::output::colors::percent(percentage)
        );
    }

    "0.00 GiB / 0.00 GiB (0%)".to_string()
}
