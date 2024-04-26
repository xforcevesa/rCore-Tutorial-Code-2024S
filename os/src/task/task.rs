//! Types related to task management

use super::TaskContext;

use crate::syscall::TaskInfo;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_info: TaskInfo,
    /// The task context
    pub task_cx: TaskContext,
    /// Last trap
    pub last_time: usize
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
