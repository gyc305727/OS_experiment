pub use memory_set::*;
mod memory_set;
pub use frame_allocator::*;
mod frame_allocator;
pub use page_table::*;
pub use address::*;
mod page_table;
mod address;
mod heap_allocator;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
    memory_set::activate_kernel_space();
    memory_set::remap_test();
}
