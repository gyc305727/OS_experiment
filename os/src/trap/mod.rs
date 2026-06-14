use core::arch::{asm, global_asm};

global_asm!(include_str!("trap.S"));

mod context;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::syscall::syscall;
use crate::task::{
    current_trap_cx, current_user_token, exit_current_and_run_next, suspend_current_and_run_next,
};
use crate::timer::set_next_trigger;

pub use context::TrapContext;

pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        asm!("csrw stvec, {}", in(reg) trap_from_kernel as usize);
    }
}

fn set_user_trap_entry() {
    unsafe {
        asm!("csrw stvec, {}", in(reg) TRAMPOLINE);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        asm!("csrs sie, {}", in(reg) 1usize << 5);
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cx = current_trap_cx();
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
            5 | 7 | 12 | 13 | 15 => {
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
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") TRAP_CONTEXT,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
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
