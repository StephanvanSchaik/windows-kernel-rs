use crate::error::{Error, IntoResult};
use windows_kernel_sys::base::{MM_COPY_ADDRESS, MM_COPY_MEMORY_PHYSICAL, MM_COPY_MEMORY_VIRTUAL, PHYSICAL_ADDRESS};
use windows_kernel_sys::base::_MEMORY_CACHING_TYPE as MEMORY_CACHING_TYPE;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryCaching {
    NonCached              = MEMORY_CACHING_TYPE::MmNonCached,
    Cached                 = MEMORY_CACHING_TYPE::MmCached,
    WriteCombined          = MEMORY_CACHING_TYPE::MmWriteCombined,
    #[cfg(feature = "system")]
    HardwareCoherentCached = MEMORY_CACHING_TYPE::MmHardwareCoherentCached,
    #[cfg(feature = "system")]
    NonCachedUnordered     = MEMORY_CACHING_TYPE::MmNonCachedUnordered,
    #[cfg(feature = "system")]
    USWCCached             = MEMORY_CACHING_TYPE::MmUSWCCached,
    NotMapped              = MEMORY_CACHING_TYPE::MmNotMapped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddress(u64);

impl From<u64> for PhysicalAddress {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Into<u64> for PhysicalAddress {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<PHYSICAL_ADDRESS> for PhysicalAddress {
    fn from(value: PHYSICAL_ADDRESS) -> Self {
        Self(unsafe { value.QuadPart } as _)
    }
}

impl Into<PHYSICAL_ADDRESS> for PhysicalAddress {
    fn into(self) -> PHYSICAL_ADDRESS {
        let mut addr: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };

        addr.QuadPart = self.0 as _;

        addr
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CopyAddress {
    Virtual(*mut core::ffi::c_void),
    Physical(PhysicalAddress),
}

impl Into<(u32, MM_COPY_ADDRESS)> for CopyAddress {
    fn into(self) -> (u32, MM_COPY_ADDRESS) {
        let mut copy_addr: MM_COPY_ADDRESS = unsafe {
            core::mem::zeroed()
        };

        let flags = match self {
            CopyAddress::Virtual(addr) => {
                copy_addr.__bindgen_anon_1.VirtualAddress = addr as _;
                MM_COPY_MEMORY_VIRTUAL
            }
            CopyAddress::Physical(addr) => {
                copy_addr.__bindgen_anon_1.PhysicalAddress = addr.into();
                MM_COPY_MEMORY_PHYSICAL
            }
        };

        (flags, copy_addr)
    }
}

pub struct IoMapping {
    ptr: *mut core::ffi::c_void,
    size: usize,
}

impl IoMapping {
    pub fn new(addr: PhysicalAddress, size: usize, caching: MemoryCaching) -> Result<Self, Error> {
        use windows_kernel_sys::ntoskrnl::MmMapIoSpace;

        let ptr = unsafe {
            MmMapIoSpace(addr.into(), size as _, caching as _)
        };

        if ptr.is_null() {
            return Err(Error::INVALID_PARAMETER);
        }

        Ok(Self {
            ptr,
            size,
        })
    }

    pub fn ptr(&self) -> &mut core::ffi::c_void {
        self.ptr
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for IoMapping {
    fn drop(&mut self) {
        use windows_kernel_sys::ntoskrnl::MmUnmapIoSpace;

        unsafe {
            MmUnmapIoSpace(self.ptr, self.size as _);
        }
    }
}

#[cfg(feature = "system")]
pub fn get_virtual_for_physical(addr: PhysicalAddress) -> *mut core::ffi::c_void {
    use windows_kernel_sys::ntoskrnl::MmGetVirtualForPhysical;

    let virt_addr = unsafe { MmGetVirtualForPhysical(addr.into()) };

    virt_addr as _
}

pub fn read_memory(
    buffer: &mut [u8],
    source: CopyAddress,
) -> Result<usize, Error> {
    use windows_kernel_sys::ntoskrnl::MmCopyMemory;

    let (flags, copy_addr) = source.into();
    let mut bytes = 0;

    unsafe {
        MmCopyMemory(
            buffer.as_mut_ptr() as _,
            copy_addr,
            buffer.len() as _,
            flags,
            &mut bytes,
        )
    }.into_result()?;

    Ok(bytes as _)
}

#[cfg(feature = "system")]
pub fn write_memory(
    target: CopyAddress,
    buffer: &[u8],
) -> Result<usize, Error> {
    use windows_kernel_sys::ntoskrnl::MmCopyMemory;

    let mut copy_addr: MM_COPY_ADDRESS = unsafe { core::mem::zeroed() };
    let mut bytes = 0;

    let target = match target {
        CopyAddress::Virtual(addr) => addr,
        CopyAddress::Physical(addr) => get_virtual_for_physical(addr),
    };

    copy_addr.__bindgen_anon_1.VirtualAddress = buffer.as_ptr() as _;

    unsafe {
        MmCopyMemory(
            target as _,
            copy_addr,
            buffer.len() as _,
            MM_COPY_MEMORY_VIRTUAL,
            &mut bytes,
        )
    }.into_result()?;

    Ok(bytes as _)
}
