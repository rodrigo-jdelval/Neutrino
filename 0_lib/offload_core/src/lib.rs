#![no_std]
pub fn sys_offload(op: usize, ptr: usize, len: usize) -> usize { unsafe { syscall(250, op, ptr, len, 0) } }