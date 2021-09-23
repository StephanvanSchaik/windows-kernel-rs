use core::alloc::{GlobalAlloc, Layout};
use windows_kernel_sys::base::_POOL_TYPE as POOL_TYPE;
use windows_kernel_sys::ntoskrnl::{ExAllocatePool, ExFreePool};

#[alloc_error_handler]
fn alloc_error(_: Layout) -> ! {
    loop {}
}

pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = ExAllocatePool(POOL_TYPE::NonPagedPool, layout.size() as u64);

        if ptr.is_null() {
            panic!("[kernel-alloc] failed to allocate pool.");
        }

        ptr as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePool(ptr as _)
    }
}
