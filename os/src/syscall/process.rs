//! Process management syscalls
use core::slice::from_raw_parts;

use crate::{mm::{get_u8_by_va, get_u8_mutable_by_va, translated_byte_buffer, MapPermission, VirtAddr}, task::{change_program_brk, create_memory_map, current_user_token, delete_memory_map, exit_current_and_run_next, get_syscall_count, suspend_current_and_run_next}, timer::get_time_us};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ref tv = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let source = tv as *const TimeVal;
    let mut start_ptr = source as *const u8;
    let targets = translated_byte_buffer(
        current_user_token(),
    ts as *const u8,
        core::mem::size_of::<TimeVal>()
    );

    for target in targets {
        let length = target.len();
        unsafe {
            target.copy_from_slice(from_raw_parts(start_ptr, length));
            start_ptr = start_ptr.add(length);
        }
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            match get_u8_by_va(current_user_token(), VirtAddr::from(id)) {
                Some(rf) => {
                    *rf as isize
                },
                None => -1
            }
        }
        1 => {
            match get_u8_mutable_by_va(current_user_token(), VirtAddr::from(id)) {
                Some(rf) => {
                    *rf = data as u8;
                    0
                },
                None => -1
             }
        }
        2 => return get_syscall_count(id) as isize,
        _ => panic!("Unsupported trace_request: {trace_request}!"),
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_va = VirtAddr::from(start);
    if !start_va.aligned() || port & !0x7 != 0 || port & 0x7 == 0 {
        return -1;
    }

    let mut pms = MapPermission::from_bits_truncate((port << 1) as u8);
    pms |= MapPermission::U;
    create_memory_map(start_va, (start + len).into(), pms)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start_va = VirtAddr::from(start);
    if !start_va.aligned() {
        return -1;
    }
    delete_memory_map(start_va, (start + len).into())
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
