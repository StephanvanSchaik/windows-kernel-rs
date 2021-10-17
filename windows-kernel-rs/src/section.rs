use bitflags::bitflags;
use crate::error::{Error, IntoResult};
use crate::process::ZwProcess;
use crate::string::create_unicode_string;
use widestring::U16CString;
use windows_kernel_sys::base::{HANDLE, LARGE_INTEGER, OBJECT_ATTRIBUTES};
use windows_kernel_sys::ntoskrnl::{ZwClose, ZwMapViewOfSection, ZwOpenSection, ZwUnmapViewOfSection};

bitflags! {
    pub struct AllocationFlags: u32 {
        const RESERVE     = windows_kernel_sys::base::MEM_RESERVE;
        const LARGE_PAGES = windows_kernel_sys::base::MEM_LARGE_PAGES;
        const TOP_DOWN    = windows_kernel_sys::base::MEM_TOP_DOWN;
    }
}

bitflags! {
    pub struct ProtectFlags: u32 {
        const READ_WRITE = windows_kernel_sys::base::PAGE_READWRITE;
    }
}

bitflags! {
    pub struct SectionAccess: u32 {
        const EXTEND_SIZE = windows_kernel_sys::base::SECTION_EXTEND_SIZE;
        const MAP_EXECUTE = windows_kernel_sys::base::SECTION_MAP_EXECUTE;
        const MAP_READ    = windows_kernel_sys::base::SECTION_MAP_READ;
        const MAP_WRITE   = windows_kernel_sys::base::SECTION_MAP_WRITE;
        const QUERY       = windows_kernel_sys::base::SECTION_QUERY;
        const ALL_ACCESS  = windows_kernel_sys::base::SECTION_ALL_ACCESS;
    }
}

bitflags! {
    pub struct ObjectFlags: u32 {
        const CASE_INSENSITIVE = windows_kernel_sys::base::OBJ_CASE_INSENSITIVE;
        const KERNEL_HANDLE    = windows_kernel_sys::base::OBJ_KERNEL_HANDLE;
    }
}

#[repr(i32)]
pub enum SectionInherit {
    ViewShare = windows_kernel_sys::base::_SECTION_INHERIT::ViewShare,
    ViewUnmap = windows_kernel_sys::base::_SECTION_INHERIT::ViewUnmap,
}

pub enum BaseAddress {
    Desired(*mut core::ffi::c_void),
    ZeroBits(usize),
}

pub struct Section {
    handle: HANDLE,
}

unsafe impl Send for Section {}
unsafe impl Sync for Section {}

impl Section {
    pub fn open(path: &str, obj_flags: ObjectFlags, access: SectionAccess) -> Result<Self, Error> {
        let name = U16CString::from_str(path).unwrap();
        let mut name = create_unicode_string(name.as_slice());

        let mut attrs = OBJECT_ATTRIBUTES {
            Length: core::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
            RootDirectory: core::ptr::null_mut(),
            ObjectName: &mut name,
            Attributes: obj_flags.bits(),
            SecurityDescriptor: core::ptr::null_mut(),
            SecurityQualityOfService: core::ptr::null_mut(),
        };

        let mut handle: HANDLE = core::ptr::null_mut();

        unsafe {
            ZwOpenSection(&mut handle, access.bits(), &mut attrs)
        }.into_result()?;

        Ok(Self {
            handle
        })
    }

    pub fn map_view(
        &mut self,
        process: ZwProcess,
        base_address: BaseAddress,
        commit_size: usize,
        offset: Option<u64>,
        view_size: usize,
        inherit: SectionInherit,
        allocation: AllocationFlags,
        protection: ProtectFlags,
    ) -> Result<SectionView, Error> {
        let (mut base_address, zero_bits) = match base_address {
            BaseAddress::Desired(ptr) => (ptr, 0),
            BaseAddress::ZeroBits(bits) => (core::ptr::null_mut(), bits),
        };

        let mut offset = offset.map(|value| {
            let mut offset: LARGE_INTEGER = unsafe { core::mem::zeroed() };
            offset.QuadPart = value as _;
            offset
        });

        let mut size: u64 = view_size as _;

        unsafe {
            ZwMapViewOfSection(
                self.handle,
                process.handle,
                &mut base_address,
                zero_bits as _,
                commit_size as _,
                match offset {
                    Some(ref mut offset) => offset as _,
                    _ => core::ptr::null_mut(),
                },
                &mut size,
                inherit as _,
                allocation.bits(),
                protection.bits(),
            )
        }.into_result()?;

        Ok(SectionView {
            process,
            address: base_address,
        })
    }
}

impl Drop for Section {
    fn drop(&mut self) {
        unsafe {
            ZwClose(self.handle);
        }
    }
}

pub struct SectionView {
    process: ZwProcess,
    address: *mut core::ffi::c_void,
}

unsafe impl Send for SectionView {}
unsafe impl Sync for SectionView {}

impl SectionView {
    pub fn address(&self) -> *mut core::ffi::c_void {
        self.address
    }
}

impl Drop for SectionView {
    fn drop(&mut self) {
        unsafe {
            ZwUnmapViewOfSection(
                self.process.handle,
                self.address,
            );
        }
    }
}
