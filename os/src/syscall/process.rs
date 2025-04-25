//! Process management syscalls
use core::slice::from_raw_parts;

use crate::{mm::{get_u8_mut_by_va, translated_byte_buffer, VirtAddr}, task::{change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_count, suspend_current_and_run_next}, timer::get_time_us};

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
            match get_u8_mut_by_va(current_user_token(), VirtAddr::from(id)) {
                Some(rf) => {
                    *rf as isize
                },
                None => -1
            }
        }
        1 => {
            match get_u8_mut_by_va(current_user_token(), VirtAddr::from(id)) {
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
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
