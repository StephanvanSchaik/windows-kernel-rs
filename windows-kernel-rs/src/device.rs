use alloc::boxed::Box;
use bitflags::bitflags;
use crate::error::Error;
use crate::request::IoRequest;
use windows_kernel_sys::base::{DEVICE_OBJECT, IRP, NTSTATUS};
use windows_kernel_sys::base::STATUS_SUCCESS;
use windows_kernel_sys::base::{IRP_MJ_CREATE, IRP_MJ_CLOSE, IRP_MJ_CLEANUP, IRP_MJ_READ, IRP_MJ_WRITE, IRP_MJ_DEVICE_CONTROL};
use windows_kernel_sys::ntoskrnl::{IoDeleteDevice, IoGetCurrentIrpStackLocation};

#[derive(Copy, Clone, Debug)]
pub enum Access {
    NonExclusive,
    Exclusive,
}

impl Access {
    pub fn is_exclusive(&self) -> bool {
        match *self {
            Access::Exclusive => true,
            _ => false,
        }
    }
}

bitflags! {
    pub struct DeviceFlags: u32 {
        const SECURE_OPEN = windows_kernel_sys::base::FILE_DEVICE_SECURE_OPEN;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DeviceType {
    Port8042,
    Acpi,
    Battery,
    Beep,
    BusExtender,
    Cdrom,
    CdromFileSystem,
    Changer,
    Controller,
    DataLink,
    Dfs,
    DfsFileSystem,
    DfsVolume,
    Disk,
    DiskFileSystem,
    Dvd,
    FileSystem,
    Unknown,
    Video,
    VirtualDisk,
    WaveIn,
    WaveOut,
}

impl Into<u32> for DeviceType {
    fn into(self) -> u32 {
        match self {
            DeviceType::Port8042 => windows_kernel_sys::base::FILE_DEVICE_8042_PORT,
            DeviceType::Acpi => windows_kernel_sys::base::FILE_DEVICE_ACPI,
            DeviceType::Battery => windows_kernel_sys::base::FILE_DEVICE_BATTERY,
            DeviceType::Beep => windows_kernel_sys::base::FILE_DEVICE_BEEP,
            DeviceType::BusExtender => windows_kernel_sys::base::FILE_DEVICE_BUS_EXTENDER,
            DeviceType::Cdrom => windows_kernel_sys::base::FILE_DEVICE_CD_ROM,
            DeviceType::CdromFileSystem => windows_kernel_sys::base::FILE_DEVICE_CD_ROM_FILE_SYSTEM,
            DeviceType::Changer => windows_kernel_sys::base::FILE_DEVICE_CHANGER,
            DeviceType::Controller => windows_kernel_sys::base::FILE_DEVICE_CONTROLLER,
            DeviceType::DataLink => windows_kernel_sys::base::FILE_DEVICE_DATALINK,
            DeviceType::Dfs => windows_kernel_sys::base::FILE_DEVICE_DFS,
            DeviceType::DfsFileSystem => windows_kernel_sys::base::FILE_DEVICE_DFS_FILE_SYSTEM,
            DeviceType::DfsVolume => windows_kernel_sys::base::FILE_DEVICE_DFS_VOLUME,
            DeviceType::Disk => windows_kernel_sys::base::FILE_DEVICE_DISK,
            DeviceType::DiskFileSystem => windows_kernel_sys::base::FILE_DEVICE_DISK_FILE_SYSTEM,
            DeviceType::Dvd => windows_kernel_sys::base::FILE_DEVICE_DVD,
            DeviceType::FileSystem => windows_kernel_sys::base::FILE_DEVICE_FILE_SYSTEM,
            DeviceType::Unknown => windows_kernel_sys::base::FILE_DEVICE_UNKNOWN,
            DeviceType::Video => windows_kernel_sys::base::FILE_DEVICE_VIDEO,
            DeviceType::VirtualDisk => windows_kernel_sys::base::FILE_DEVICE_VIRTUAL_DISK,
            DeviceType::WaveIn => windows_kernel_sys::base::FILE_DEVICE_WAVE_IN,
            DeviceType::WaveOut => windows_kernel_sys::base::FILE_DEVICE_WAVE_OUT,
        }
    }
}

#[repr(C)]
pub struct device_operations {
    dispatch: Option<extern "C" fn (*mut DEVICE_OBJECT, *mut IRP, u8) -> NTSTATUS>,
    release: Option<extern "C" fn (*mut DEVICE_OBJECT)>,
}

pub struct Device {
    raw: *mut DEVICE_OBJECT,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    pub unsafe fn from_raw(raw: *mut DEVICE_OBJECT) -> Self {
        Self {
            raw,
        }
    }

    pub unsafe fn as_raw(&self) -> *const DEVICE_OBJECT {
        self.raw as *const _
    }

    pub unsafe fn as_raw_mut(&self) -> *mut DEVICE_OBJECT {
        self.raw
    }

    pub fn into_raw(mut self) -> *mut DEVICE_OBJECT {
        core::mem::replace(&mut self.raw, core::ptr::null_mut())
    }

    pub(crate) fn extension(&self) -> &DeviceExtension {
        unsafe {
            &*((*self.raw).DeviceExtension as *const DeviceExtension)
        }
    }

    pub(crate) fn extension_mut(&self) -> &mut DeviceExtension {
        unsafe {
            &mut *((*self.raw).DeviceExtension as *mut DeviceExtension)
        }
    }

    pub(crate) fn vtable(&self) -> &device_operations {
        unsafe {
            &*(self.extension().vtable as *const _)
        }
    }

    pub fn data<T: DeviceOperations>(&self) -> &T {
        unsafe {
            &*(self.extension().data as *const T)
        }
    }

    pub fn data_mut<T: DeviceOperations>(&self) -> &mut T {
        unsafe {
            &mut *(self.extension().data as *mut T)
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }

        unsafe {    
            if let Some(release) = self.vtable().release {
                release(self.raw);
            }

            IoDeleteDevice(self.raw);
        }
    }
}

pub trait DeviceOperations: Sync + Sized {
    fn create(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        request.complete(Ok(0));
        Ok(())
    }

    fn close(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        request.complete(Ok(0));
        Ok(())
    }

    fn cleanup(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        request.complete(Ok(0));
        Ok(())
    }

    fn read(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        request.complete(Ok(0));
        Ok(())
    }

    fn write(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        request.complete(Ok(0));
        Ok(())
    }

    fn ioctl(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        request.complete(Ok(0));
        Ok(())
    }
}

extern "C" fn dispatch_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
    major: u8,
) -> NTSTATUS {
    let device = unsafe { Device::from_raw(device) };
    let data: &mut T = device.data_mut();
    let request = unsafe { IoRequest::from_raw(irp) };

    let status = match major as _ {
        IRP_MJ_CREATE => data.create(&device, &request),
        IRP_MJ_CLOSE => data.close(&device, &request),
        IRP_MJ_CLEANUP => data.cleanup(&device, &request),
        IRP_MJ_READ => data.read(&device, &request),
        IRP_MJ_WRITE => data.write(&device, &request),
        IRP_MJ_DEVICE_CONTROL => data.ioctl(&device, &request),
        _ => {
            request.complete(Err(Error::INVALID_PARAMETER));
            Err(Error::INVALID_PARAMETER)
        }
    };

    device.into_raw();

    match status {
        Ok(()) => STATUS_SUCCESS,
        Err(e) => e.to_kernel_errno(),
    }
}

extern fn release_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
) {
    unsafe {
        let extension = (*device).DeviceExtension as *mut DeviceExtension;

        let ptr = core::mem::replace(&mut (*extension).data, core::ptr::null_mut());
        Box::from_raw(ptr as *mut T);
    }
}

pub(crate) struct DeviceOperationsVtable<T>(core::marker::PhantomData<T>);

impl<T: DeviceOperations> DeviceOperationsVtable<T> {
    pub(crate) const VTABLE: device_operations = device_operations {
        dispatch: Some(dispatch_callback::<T>),
        release: Some(release_callback::<T>),
    };
}

#[repr(C)]
pub struct DeviceExtension {
    pub(crate) vtable: *const device_operations,
    pub(crate) data: *mut cty::c_void,
}

pub extern "C" fn dispatch_device(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
) -> NTSTATUS {
    let stack_location = unsafe { &*IoGetCurrentIrpStackLocation(irp) };
    let device = unsafe { Device::from_raw(device) };
    let vtable = device.vtable();

    match vtable.dispatch {
        Some(dispatch) => dispatch(device.into_raw(), irp, stack_location.MajorFunction),
        _ => {
            device.into_raw();
            STATUS_SUCCESS
        }
    }
}
