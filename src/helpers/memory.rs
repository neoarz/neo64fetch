// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/memory/memory_apple.c

use libc::{
    HOST_VM_INFO64, HOST_VM_INFO64_COUNT, host_statistics64, mach_host_self, vm_statistics64_data_t,
};
use std::mem;
use std::process::Command;

pub fn get_memory_info() -> String {
    let total_bytes = Command::new("sysctl")
        .args(["-n", "hw.memsize"])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .unwrap_or(0);

    let usable_bytes = Command::new("sysctl")
        .args(["-n", "hw.memsize_usable"])
        .output()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .parse::<u64>()
                .unwrap_or(total_bytes)
        })
        .unwrap_or(total_bytes);

    let mut used_bytes: u64 = 0;

    // mach_host_self, and HOST_VM_INFO64 are macos c functions (ffi), so we HAVE to use unsafe
    // i needa learn a better way to do this tbh
    // learned smth new lol
    unsafe {
        let mut count = HOST_VM_INFO64_COUNT;
        let mut vmstat: vm_statistics64_data_t = mem::zeroed(); // this this is unsafe we have to manually zero it

        if host_statistics64(
            mach_host_self(),
            HOST_VM_INFO64,
            &mut vmstat as *mut _ as *mut _,
            &mut count,
        ) == 0
        {
            let page_size = Command::new("pagesize")
                .output()
                .map(|o| {
                    String::from_utf8_lossy(&o.stdout)
                        .trim()
                        .parse::<u64>()
                        .unwrap_or(4096)
                })
                .unwrap_or(4096);

            let app_memory = (vmstat.internal_page_count as u64) * page_size;
            let wired_memory = (vmstat.wire_count as u64) * page_size;
            let compressed_memory = (vmstat.compressor_page_count as u64) * page_size;
            let reserved_memory = total_bytes.saturating_sub(usable_bytes);

            used_bytes = app_memory + wired_memory + compressed_memory + reserved_memory;
        }
    }

    let total_gib = total_bytes as f64 / 1073741824.0;
    let used_gib = used_bytes as f64 / 1073741824.0;
    let percentage = if total_bytes > 0 {
        (used_bytes as f64 / total_bytes as f64) * 100.0
    } else {
        0.0
    };

    format!(
        "{:.2} GiB / {:.2} GiB ({})",
        used_gib, total_gib, crate::output::colors::percent(percentage)
    )
}
