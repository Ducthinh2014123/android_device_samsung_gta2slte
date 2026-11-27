#![cfg_attr(not(test), no_std)]

use core::ffi::c_void;

// MemAvailable is a better signal than MemFree because Android can reclaim file cache.
// Avoid reclaiming on a healthy system, and cap work on low-core devices.
fn reclaim_budget(available_kib: i64, total_kib: i64, online_cpus: i32) -> i32 {
    if total_kib <= 0 || available_kib < 0 || online_cpus <= 0 {
        return 0;
    }

    let memory_budget = if available_kib < total_kib / 10 {
        4
    } else if available_kib < total_kib / 5 {
        2
    } else {
        0
    };
    let cpu_budget = match online_cpus {
        1..=2 => 1,
        3..=4 => 2,
        _ => 4,
    };
    core::cmp::min(memory_budget, cpu_budget)
}

#[no_mangle]
pub extern "system" fn Java_com_android_server_am_GameModePolicy_nativeReclaimBudget(
    _env: *mut c_void,
    _class: *mut c_void,
    available_kib: i64,
    total_kib: i64,
    online_cpus: i32,
) -> i32 {
    reclaim_budget(available_kib, total_kib, online_cpus)
}

#[cfg(test)]
mod tests {
    use super::reclaim_budget;

    #[test]
    fn bounds_reclaim_under_memory_pressure() {
        assert_eq!(reclaim_budget(545_092, 1_873_032, 4), 0);
        assert_eq!(reclaim_budget(300_000, 1_873_032, 8), 2);
        assert_eq!(reclaim_budget(150_000, 1_873_032, 4), 2);
        assert_eq!(reclaim_budget(150_000, 1_873_032, 2), 1);
        assert_eq!(reclaim_budget(150_000, 1_873_032, 8), 4);
    }

    #[test]
    fn requires_valid_telemetry() {
        assert_eq!(reclaim_budget(-1, 1_873_032, 4), 0);
        assert_eq!(reclaim_budget(0, 0, 4), 0);
        assert_eq!(reclaim_budget(150_000, 1_873_032, 0), 0);
    }
}
