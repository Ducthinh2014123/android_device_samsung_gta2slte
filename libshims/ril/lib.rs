#![no_std]

use core::{mem, ptr};

extern "C" {
    fn strlen(src: *const u8) -> usize;
    fn malloc(size: usize) -> *mut u16;
    fn utf8_to_utf16_length(src: *const u8, len: usize, overread_is_fatal: bool) -> isize;
    fn utf8_to_utf16(src: *const u8, src_len: usize, dst: *mut u16, dst_len: usize) -> *mut u16;
}

/// Returns a NUL-terminated, libc-allocated UTF-16 copy for the vendor RIL.
///
/// The caller owns the result and must free it with libc free. A non-null
/// source must point to a readable C string and out_len must be writable.
#[no_mangle]
pub unsafe extern "C" fn strdup8to16(src: *const u8, out_len: *mut usize) -> *mut u16 {
    if out_len.is_null() {
        return ptr::null_mut();
    }
    *out_len = 0;
    if src.is_null() {
        return ptr::null_mut();
    }

    // Keep libutils' handling of malformed UTF-8 and its existing bounds contract.
    let input_len = strlen(src);
    let measured = utf8_to_utf16_length(src, input_len, false);
    if measured < 0 || measured == isize::max_value() {
        return ptr::null_mut();
    }
    let capacity = measured as usize + 1;
    let bytes = match capacity.checked_mul(mem::size_of::<u16>()) {
        Some(bytes) => bytes,
        None => return ptr::null_mut(),
    };
    let result = malloc(bytes);
    if result.is_null() {
        return ptr::null_mut();
    }

    let end = utf8_to_utf16(src, input_len, result, capacity);
    // libutils returns a pointer into the allocation, at the terminator.
    *out_len = (end as usize - result as usize) / mem::size_of::<u16>();
    result
}
