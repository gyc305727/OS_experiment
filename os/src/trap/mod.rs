use core::arch::{asm, global_asm};

global_asm!(include_str!("trap.S"));

mod context;

use crate::syscall::syscall;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;

pub use context::TrapContext;

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        asm!("csrw stvec, {}", in(reg) __alltraps as usize);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        asm!("csrs sie, {}", in(reg) 1usize << 5);
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
                println!(
                    "[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, core dumped.",
                    stval,
                    cx.sepc
                );
                exit_current_and_run_next();
            }
            2 => {
                println!("[kernel] IllegalInstruction in application, core dumped.");
                exit_current_and_run_next();
            }
            _ => panic!("Unsupported trap {}, stval = {:#x}!", scause, stval),
        }
    } else {
        match cause {
            5 => {
                set_next_trigger();
                suspend_current_and_run_next();
            }
            _ => panic!("Unsupported interrupt {}, stval = {:#x}!", scause, stval),
        }
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
