use crate::error::{Error, IntoResult};
use windows_kernel_sys::base::{MM_COPY_ADDRESS, MM_COPY_MEMORY_PHYSICAL, MM_COPY_MEMORY_VIRTUAL, PHYSICAL_ADDRESS};
use windows_kernel_sys::ntoskrnl::{MmCopyMemory, MmGetVirtualForPhysical};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CopyAddress {
    Virtual(u64),
    Physical(u64),
}

pub fn get_virtual_for_physical(addr: u64) -> u64 {
    let mut phys_addr: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };

    phys_addr.QuadPart = addr as _;

    let virt_addr = unsafe { MmGetVirtualForPhysical(phys_addr) };

    virt_addr as _
}

pub fn read_memory(
    buffer: &mut [u8],
    source: CopyAddress,
) -> Result<usize, Error> {
    let mut copy_addr: MM_COPY_ADDRESS = unsafe { core::mem::zeroed() };
    let mut bytes = 0;

    let flags = match source {
        CopyAddress::Virtual(addr) => {
            copy_addr.__bindgen_anon_1.VirtualAddress = addr as _;
            MM_COPY_MEMORY_VIRTUAL
        }
        CopyAddress::Physical(addr) => {
            copy_addr.__bindgen_anon_1.PhysicalAddress.QuadPart = addr as _;
            MM_COPY_MEMORY_PHYSICAL
        }
    };

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

pub fn write_memory(
    target: CopyAddress,
    buffer: &[u8],
) -> Result<usize, Error> {
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
