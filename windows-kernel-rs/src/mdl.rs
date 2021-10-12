use crate::error::Error;
use crate::memory::MemoryCaching;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessMode {
    KernelMode = windows_kernel_sys::base::_MODE::KernelMode,
    UserMode   = windows_kernel_sys::base::_MODE::UserMode,
}

pub struct MemoryDescriptorList {
    raw: *mut windows_kernel_sys::base::MDL,
}

impl MemoryDescriptorList {
    pub fn new(
        addr: *mut core::ffi::c_void,
        size: usize,
    ) -> Result<Self, Error> {
        use windows_kernel_sys::ntoskrnl::IoAllocateMdl;

        let raw = unsafe {
            IoAllocateMdl(addr, size as _, false as _, false as _, core::ptr::null_mut())
        };

        if raw.is_null() {
            return Err(Error::INSUFFICIENT_RESOURCES);
        }

        Ok(Self {
            raw,
        })
    }

    pub fn map_locked_pages(
        self,
        access: AccessMode,
        caching: MemoryCaching,
        desired_addr: Option<*mut core::ffi::c_void>,
    ) -> Result<LockedMapping, Error> {
        use windows_kernel_sys::ntoskrnl::MmMapLockedPagesSpecifyCache;

        let ptr = unsafe {
            MmMapLockedPagesSpecifyCache(
                self.raw,
                access as _,
                caching as _,
                desired_addr.unwrap_or(core::ptr::null_mut()),
                false as _,
                0,
            )
        };

        Ok(LockedMapping {
            raw: self.raw,
            ptr,
        })
    }
}

impl Drop for MemoryDescriptorList {
    fn drop(&mut self) {
        use windows_kernel_sys::ntoskrnl::IoFreeMdl;

        unsafe {
            IoFreeMdl(self.raw);
        }
    }
}

pub struct LockedMapping {
    raw: *mut windows_kernel_sys::base::MDL,
    ptr: *mut core::ffi::c_void,
}

impl LockedMapping {
    pub fn ptr(&self) -> *mut core::ffi::c_void {
        self.ptr
    }

    pub fn unlock(self) -> MemoryDescriptorList {
        use windows_kernel_sys::ntoskrnl::MmUnmapLockedPages;

        unsafe {
            MmUnmapLockedPages(self.ptr, self.raw);
        }

        MemoryDescriptorList {
            raw: self.raw,
        }
    }
}

impl Drop for LockedMapping {
    fn drop(&mut self) {
        use windows_kernel_sys::ntoskrnl::{IoFreeMdl, MmUnmapLockedPages};

        unsafe {
            MmUnmapLockedPages(self.ptr, self.raw);
            IoFreeMdl(self.raw);
        }
    }
}
