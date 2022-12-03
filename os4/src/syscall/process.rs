//! Process management syscalls
use crate::config::{MAX_SYSCALL_NUM, PAGE_SIZE};
use crate::mm::{VirtAddr, MapPermission};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, copy_to_user, mmap, munmap};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let kernel_ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
    };
    let dst = VirtAddr::from(ts as usize);
    unsafe{copy_to_user(dst,&kernel_ts) as isize}
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    if (port & !0x7 != 0) || (port & 0x7 == 0) || (start % PAGE_SIZE != 0){
        return -1;
    }
    let end = VirtAddr::from(start + len);
    let start = VirtAddr::from(start);
    let mut perm = MapPermission::U;
    if port&0x1 != 0{
        perm |= MapPermission::R;
    }
    if port&0x2 != 0{
        perm |= MapPermission::W;
    }
    if port&0x4 != 0{
        perm |= MapPermission::X;
    }
    mmap(start, end, perm)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    let end = VirtAddr::from(start + len);
    let start = VirtAddr::from(start);
    munmap(start, end)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    -1
}


