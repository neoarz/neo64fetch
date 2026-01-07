// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/disk/disk.c

use libc::{c_int, c_char};
use std::ffi::CStr;

#[repr(C)]
#[derive(Clone, Copy)]
struct Statfs {
    f_bsize: u32,
    f_iosize: c_int,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_fsid: [u32; 2],
    f_owner: u32,
    f_type: u32,
    f_flags: u32,
    f_fssubtype: u32,
    f_fstypename: [c_char; 16],
    f_mntonname: [c_char; 1024],
    f_mntfromname: [c_char; 1024],
    f_reserved: [u32; 8],
}

unsafe extern "C" {
    fn getfsstat(buf: *mut Statfs, bufsize: c_int, flags: c_int) -> c_int;
}

const MNT_WAIT: c_int = 1;
const MNT_NOWAIT: c_int = 2;
const MNT_RDONLY: u32 = 0x00000001;

pub fn get_storage_info() -> String {
    unsafe {
        let size = getfsstat(std::ptr::null_mut(), 0, MNT_WAIT);
        if size <= 0 {
            return "<unknown>".to_string();
        }

        let statfs_size = std::mem::size_of::<Statfs>();
        if statfs_size != 2168 {
            return "<unknown>".to_string();
        }

        let mut buf = vec![std::mem::zeroed::<Statfs>(); size as usize];
        let bufsize = (statfs_size * size as usize) as c_int;

        let result = getfsstat(buf.as_mut_ptr(), bufsize, MNT_NOWAIT);
        if result <= 0 {
            return "<unknown>".to_string();
        }

        for fs in &buf {
            let mountpoint = CStr::from_ptr(fs.f_mntonname.as_ptr() as *const c_char)
                .to_string_lossy()
                .to_string();

            if mountpoint == "/" {
                let filesystem = CStr::from_ptr(fs.f_fstypename.as_ptr() as *const c_char)
                    .to_string_lossy()
                    .to_string();

                let block_size = fs.f_bsize as u64;
                let total_bytes = fs.f_blocks * block_size;
                let available_bytes = fs.f_bavail * block_size;
                let used_bytes = total_bytes.saturating_sub(available_bytes);

                let total_gib = total_bytes as f64 / 1073741824.0;
                let used_gib = used_bytes as f64 / 1073741824.0;
                let percentage = if total_bytes > 0 {
                    (used_bytes as f64 / total_bytes as f64) * 100.0
                } else {
                    0.0
                };

                let read_only = (fs.f_flags & MNT_RDONLY) != 0;

                let mut result = format!(
                    "{:.2} GiB / {:.2} GiB ({:.0}%)",
                    used_gib, total_gib, percentage
                );

                if !filesystem.is_empty() {
                    result.push_str(&format!(" - {}", filesystem));
                }

                if read_only {
                    result.push_str(" [Read-only]");
                }

                return result;
            }
        }
    }

    "<unknown>".to_string()
}

