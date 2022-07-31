//! Process management syscalls

use riscv::addr::page;
use riscv::paging::PageTable;

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next,current_user_token,get_current_task, TaskStatus,task_map_an_area,task_unmap_an_area};
use crate::timer::get_time_us;
use crate::mm::{frame_alloc, VirtAddr,get_ptr_physical_addr};
use crate::config::{PAGE_SIZE,PAGE_SIZE_BITS};

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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let _us = get_time_us();
    let _ts=get_ptr_physical_addr(_ts as usize) as *mut TimeVal;
    unsafe {
        *_ts = TimeVal {
            sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    if _start%PAGE_SIZE!=0{
        return -1;
    }
    if _port & !0x7 != 0{
        return -1;
    }
    if _port & 0x7 == 0{
        return -1;
    }
    task_map_an_area(_start, _len, _port)
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    task_unmap_an_area(_start, _len)
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let curr_time=get_time_us();
    let task_ctr=get_current_task();
    let ti=get_ptr_physical_addr(ti as usize) as *mut TaskInfo;
    unsafe {
        (*ti).status=task_ctr.task_status;
        (*ti).syscall_times=task_ctr.task_syscall_times;
        (*ti).time=curr_time/1000-task_ctr.task_start_time/1000;
    }
    0
}
