use crate::lib::allocator::Allocator;
use core::alloc::{GlobalAlloc, Layout};

// HEAP region, they symbols must be define in the linker script
extern "C" {
    static __heap_start: u8;
    static __heap_end: u8;
}

static HEAP: Allocator = Allocator::new(unsafe { &__heap_start as *const u8 }, unsafe {
    &__heap_end as *const u8
});

/// allocate memory from heap
pub fn alloc(size: usize, align: usize) -> *mut u8 {
    HEAP.alloc(size as u32, align as u32)
}

/// free heap memory
pub fn free(p: *mut u8) {
    HEAP.free(p)
}

struct HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, p: *mut u8, _: Layout) {
        free(p)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: HeapAllocator = HeapAllocator;

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("out of memory");
}
