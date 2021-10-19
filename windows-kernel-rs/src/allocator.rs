//! This module provides an allocator to use with the [`alloc`] crate. You can define your own
//! global allocator with the `#[global_allocator]` attribute when not using the `alloc` feature,
//! in case you want to specify your own tag to use with [`ExAllocatePool2`] and
//! [`ExAllocatePoolWithTag`].

use core::alloc::{GlobalAlloc, Layout};
use crate::version::VersionInfo;
use lazy_static::lazy_static;
use windows_kernel_sys::base::_POOL_TYPE as POOL_TYPE;
use windows_kernel_sys::ntoskrnl::{ExAllocatePoolWithTag, ExAllocatePool2, ExFreePool};

/// See issue #52191.
#[alloc_error_handler]
fn alloc_error(_: Layout) -> ! {
    loop {}
}

lazy_static! {
    /// The version of Microsoft Windows that is currently running. This is used by
    /// [`KernelAllocator`] to determine whether to use [`ExAllocatePool2`] or
    /// [`ExAllocatePoolWithTag`].
    static ref VERSION_INFO: VersionInfo = {
        VersionInfo::query().unwrap()
    };
}

/// Represents a kernel allocator that relies on the `ExAllocatePool` family of functions to
/// allocate and free memory for the `alloc` crate.
pub struct KernelAllocator {
    /// The 32-bit tag to use for the pool, this is usually derived from a quadruplet of ASCII
    /// bytes, e.g. by invoking `u32::from_ne_bytes(*b"rust")`.
    tag: u32,
}

impl KernelAllocator {
    /// Sets up a new kernel allocator with the 32-bit tag specified. The tag is usually derived
    /// from a quadruplet of ASCII bytes, e.g. by invoking `u32::from_ne_bytes(*b"rust")`.
    pub const fn new(tag: u32) -> Self {
        Self {
            tag,
        }
    }
}

unsafe impl GlobalAlloc for KernelAllocator {
    /// Uses [`ExAllocatePool2`] on Microsoft Windows 10.0.19041 and later, and
    /// [`ExAllocatePoolWithTag`] on older versions of Microsoft Windows to allocate memory.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let use_ex_allocate_pool2 =
            VERSION_INFO.major() > 10 ||
            (VERSION_INFO.major() == 10 && VERSION_INFO.build_number() == 19041);

        let ptr = if use_ex_allocate_pool2 {
            ExAllocatePool2(
                POOL_TYPE::NonPagedPool as _,
                layout.size() as u64,
                self.tag,
            )
        } else {
            ExAllocatePoolWithTag(
                POOL_TYPE::NonPagedPool,
                layout.size() as u64,
                self.tag,
            )
        };

        if ptr.is_null() {
            panic!("[kernel-alloc] failed to allocate pool.");
        }

        ptr as _
    }

    /// Uses [`ExFreePool`] to free allocated memory.
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePool(ptr as _)
    }
}
