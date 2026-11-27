#![no_std]

mod profile;

use core::ffi::c_void;

const READ_ONLY: i32 = 0;
const WRITE_ONLY: i32 = 1;
const INFO: i32 = 4;
const TAG: &[u8] = b"gta2slte-rust-post-boot\0";

extern "C" {
    fn open(path: *const u8, flags: i32, ...) -> i32;
    fn read(fd: i32, buffer: *mut c_void, len: usize) -> isize;
    fn write(fd: i32, buffer: *const c_void, len: usize) -> isize;
    fn close(fd: i32) -> i32;
    fn memcmp(a: *const c_void, b: *const c_void, len: usize) -> i32;
    fn strcmp(a: *const u8, b: *const u8) -> i32;
    fn __android_log_print(prio: i32, tag: *const u8, format: *const u8, ...) -> i32;
    fn printf(format: *const u8, ...) -> i32;
}

fn matches(path: &[u8], value: &[u8]) -> bool {
    let mut buffer = [0u8; 128];
    unsafe {
        let fd = open(path.as_ptr(), READ_ONLY);
        if fd < 0 {
            return false;
        }
        let size = read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len());
        close(fd);
        size == value.len() as isize
            && memcmp(buffer.as_ptr() as *const c_void, value.as_ptr() as *const c_void,
                      value.len()) == 0
    }
}

fn apply(path: &[u8], value: &[u8]) -> bool {
    unsafe {
        let fd = open(path.as_ptr(), WRITE_ONLY);
        if fd < 0 {
            return false;
        }
        let size = write(fd, value.as_ptr() as *const c_void, value.len());
        close(fd);
        size == value.len() as isize
    }
}

// Android's C startup code calls this entry point. No Rust runtime is required.
#[no_mangle]
pub unsafe extern "C" fn main(argc: i32, argv: *const *const u8) -> i32 {
    if argc != 2 {
        return 2;
    }
    let arg = *argv.add(1);
    let apply_changes = strcmp(arg, b"--apply\0".as_ptr()) == 0;
    if !apply_changes && strcmp(arg, b"--check\0".as_ptr()) != 0 {
        return 2;
    }

    let mut unchanged = 0;
    let mut written = 0;
    let mut unresolved = 0;
    let mut governor_ok = true;
    for &(path, value) in profile::SETTINGS {
        if matches(path, value) {
            unchanged += 1;
        } else if apply_changes && apply(path, value) {
            written += 1;
        } else {
            unresolved += 1;
            if path.ends_with(b"/scaling_governor\0") {
                governor_ok = false;
            }
        }
    }

    if apply_changes {
        __android_log_print(INFO, TAG.as_ptr(),
            b"CPU profile: %d unchanged, %d written, %d unavailable\0".as_ptr(),
            unchanged, written, unresolved);
        // If the required governor could not be configured, the caller uses the old shell path.
        if !governor_ok {
            return 1;
        }
    } else {
        printf(b"CPU profile: %d already match, %d different or unreadable\n\0".as_ptr(),
            unchanged, unresolved);
    }
    0
}
