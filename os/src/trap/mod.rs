use core::arch::{asm, global_asm};

global_asm!(include_str!("trap.S"));

mod context;

use crate::syscall::syscall;

pub use context::TrapContext;

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        asm!("csrw stvec, {}", in(reg) __alltraps as usize);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = read_scause();
    let stval = read_stval();
    let is_interrupt = (scause >> 63) != 0;
    let cause = scause & 0xfff;

    if !is_interrupt {
        match cause {
            8 => {
                cx.sepc += 4;
                cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            }
            7 | 15 => {
                panic!("[kernel] PageFault in application, stval = {:#x}", stval);
            }
            2 => {
                panic!("[kernel] IllegalInstruction in application, stval = {:#x}", stval);
            }
            _ => {
                panic!("Unsupported trap {}, stval = {:#x}!", scause, stval);
            }
        }
    } else {
        panic!("Unsupported interrupt {}, stval = {:#x}!", scause, stval);
    }
    cx
}

fn read_scause() -> usize {
    let bits: usize;
    unsafe {
        asm!("csrr {}, scause", out(reg) bits);
    }
    bits
}

fn read_stval() -> usize {
    let bits: usize;
    unsafe {
        asm!("csrr {}, stval", out(reg) bits);
    }
    bits
}
