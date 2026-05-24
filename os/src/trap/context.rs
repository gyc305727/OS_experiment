
use core::arch::asm;

#[repr(C)]

pub struct TrapContext {

pub x: [usize; 32],

pub sstatus: usize,

pub sepc: usize,

}

impl TrapContext {

pub fn set_sp(&mut self, sp: usize) {

self.x[2] = sp;

}

pub fn app_init_context(entry: usize, sp: usize) -> Self {

let mut sstatus = read_sstatus();

sstatus &= !(1 << 8);

let mut cx = Self {

x: [0; 32],

sstatus,

sepc: entry,

};

cx.set_sp(sp);

cx

}

}

fn read_sstatus() -> usize {

let bits: usize;

unsafe {

asm!("csrr {}, sstatus", out(reg) bits);

}

bits

}

