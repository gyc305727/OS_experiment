use crate::trap::trap_return;

#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_trap_return(kernel_stack_top: usize) -> Self {
        Self {
            ra: trap_return as usize,
            sp: kernel_stack_top,
            s: [0; 12],
        }
    }
}
