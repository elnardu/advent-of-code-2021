use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) static ARE_ALLOCATIONS_ALLOWED: AtomicBool = AtomicBool::new(true);

#[macro_export]
macro_rules! without_alloc {
    ($b:block) => {
        use crate::alloc::ARE_ALLOCATIONS_ALLOWED;
        use std::sync::atomic::Ordering;

        ARE_ALLOCATIONS_ALLOWED.store(false, Ordering::Release);
        {
            $b
        }
        ARE_ALLOCATIONS_ALLOWED.store(true, Ordering::Release);
    };
}

pub struct PanicAlloc;

unsafe impl GlobalAlloc for PanicAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if ARE_ALLOCATIONS_ALLOWED.load(Ordering::Acquire) {
            System.alloc(layout)
        } else {
            ARE_ALLOCATIONS_ALLOWED.store(true, Ordering::Release);
            panic!("No allocations allowed!");
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ARE_ALLOCATIONS_ALLOWED.load(Ordering::Acquire) {
            System.dealloc(ptr, layout)
        } else {
            ARE_ALLOCATIONS_ALLOWED.store(true, Ordering::Release);
            panic!("No deallocations allowed!");
        }
    }
}
