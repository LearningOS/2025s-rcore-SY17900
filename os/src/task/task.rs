//! Types related to task management

use crate::syscall::MAX_SYSCALL_ID;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// Syscall count records
    pub syscall_count: [u32; MAX_SYSCALL_ID + 1],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

impl TaskControlBlock {
    /// Increase syscall count by syscall_id
    pub fn increase_syscall_count(&mut self, syscall_id: usize) {
        if syscall_id > MAX_SYSCALL_ID {
            panic!("Unsupported syscall_id: {syscall_id}");
        }
        self.syscall_count[syscall_id] += 1;
    }
    /// Get syscall count by syscall_id
    pub fn get_syscall_count(&self, syscall_id: usize) -> u32 {
        if syscall_id > MAX_SYSCALL_ID {
            panic!("Unsupported syscall_id: {syscall_id}");
        }
        self.syscall_count[syscall_id]
    }
}