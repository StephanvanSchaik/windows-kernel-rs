use alloc::boxed::Box;
use crate::error::Error;
use windows_kernel_sys::base::{DEVICE_OBJECT, IO_NO_INCREMENT, IRP, NTSTATUS};
use windows_kernel_sys::base::{STATUS_INVALID_PARAMETER, STATUS_SUCCESS};
use windows_kernel_sys::base::{IRP_MJ_CREATE, IRP_MJ_CLOSE, IRP_MJ_CLEANUP, IRP_MJ_DEVICE_CONTROL};
use windows_kernel_sys::ntoskrnl::{IoCompleteRequest, IoDeleteDevice, IoGetCurrentIrpStackLocation};

#[repr(C)]
pub struct device_operations {
    create: Option<unsafe extern "C" fn (*mut DEVICE_OBJECT) -> NTSTATUS>,
    close: Option<unsafe extern "C" fn (*mut DEVICE_OBJECT) -> NTSTATUS>,
    cleanup: Option<unsafe extern "C" fn (*mut DEVICE_OBJECT) -> NTSTATUS>,
    ioctl: Option<unsafe extern "C" fn (*mut DEVICE_OBJECT, u32, *mut cty::c_void, usize, usize, *mut usize) -> NTSTATUS>,
    release: Option<unsafe extern "C" fn (*mut DEVICE_OBJECT)>,
}

pub struct Device {
    pub(crate) raw: *mut DEVICE_OBJECT,
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {    
            let extension = (*self.raw).DeviceExtension as *mut DeviceExtension;
            let vtable = (*extension).vtable;

            if let Some(release) = (*vtable).release {
                release(self.raw);
            }

            IoDeleteDevice(self.raw);
        }
    }
}

pub trait DeviceOperations: Sync + Sized {
    fn create(&mut self, device: Device) -> Result<(), Error>;
    fn close(&mut self, device: Device) -> Result<(), Error>;
    fn cleanup(&mut self, device: Device) -> Result<(), Error>;
    fn ioctl(&mut self, device: Device, ioctl_num: u32, user_ptr: &mut UserPtr) -> Result<(), Error>;
}

unsafe extern "C" fn create_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
) -> NTSTATUS {
    let extension = (*device).DeviceExtension as *mut DeviceExtension;
    let data = &mut *((*extension).data as *mut T);
    let device = Device { raw: device };

    match data.create(device) {
        Ok(()) => STATUS_SUCCESS,
        Err(e) => e.to_kernel_errno(),
    }
}

unsafe extern "C" fn close_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
) -> NTSTATUS {
    let extension = (*device).DeviceExtension as *mut DeviceExtension;
    let data = &mut *((*extension).data as *mut T);
    let device = Device { raw: device };

    match data.close(device) {
        Ok(()) => STATUS_SUCCESS,
        Err(e) => e.to_kernel_errno(),
    }
}

unsafe extern "C" fn cleanup_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
) -> NTSTATUS {
    let extension = (*device).DeviceExtension as *mut DeviceExtension;
    let data = &mut *((*extension).data as *mut T);
    let device = Device { raw: device };

    match data.cleanup(device) {
        Ok(()) => STATUS_SUCCESS,
        Err(e) => e.to_kernel_errno(),
    }
}

unsafe extern "C" fn ioctl_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
    ioctl_num: u32,
    ptr: *mut cty::c_void,
    read_size: usize,
    write_size: usize,
    return_size: *mut usize,
) -> NTSTATUS {
    let extension = (*device).DeviceExtension as *mut DeviceExtension;
    let data = &mut *((*extension).data as *mut T);
    let device = Device { raw: device };
    let mut user_ptr = UserPtr::new(ptr, read_size, write_size);

    let status = match data.ioctl(device, ioctl_num, &mut user_ptr) {
        Ok(()) => STATUS_SUCCESS,
        Err(e) => e.to_kernel_errno(),
    };

    *return_size = user_ptr.return_size();

    status
}

unsafe extern fn release_callback<T: DeviceOperations>(
    device: *mut DEVICE_OBJECT,
) {
    let extension = (*device).DeviceExtension as *mut DeviceExtension;
    
    let ptr = core::mem::replace(&mut (*extension).data, core::ptr::null_mut());
    Box::from_raw(ptr as *mut T);
}

pub(crate) struct DeviceOperationsVtable<T>(core::marker::PhantomData<T>);

impl<T: DeviceOperations> DeviceOperationsVtable<T> {
    pub(crate) const VTABLE: device_operations = device_operations {
        create: Some(create_callback::<T>),
        close: Some(close_callback::<T>),
        cleanup: Some(cleanup_callback::<T>),
        ioctl: Some(ioctl_callback::<T>),
        release: Some(release_callback::<T>),
    };
}

#[repr(C)]
pub struct DeviceExtension {
    pub(crate) vtable: *const device_operations,
    pub(crate) data: *mut cty::c_void,
}

pub struct UserPtr {
    ptr: *mut cty::c_void,
    read_size: usize,
    write_size: usize,
    return_size: usize,
}

impl UserPtr {
    pub unsafe fn new(
        ptr: *mut cty::c_void,
        read_size: usize,
        write_size: usize,
    ) -> Self {
        Self {
            ptr,
            read_size,
            write_size,
            return_size: 0,
        }
    }

    pub fn return_size(&self) -> usize {
        self.return_size
    }

    pub fn read<T>(&self, obj: &mut T) -> Result<(), Error> {
        if core::mem::size_of::<T>() != self.read_size {
            return Err(Error::ACCESS_VIOLATION);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                self.ptr as _,
                obj,
                core::mem::size_of::<T>(),
            );
        }

        Ok(())
    }

    pub fn write<T>(&mut self, obj: &T) -> Result<(), Error> {
        if core::mem::size_of::<T>() > self.write_size {
            return Err(Error::ACCESS_VIOLATION);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(
                obj,
                self.ptr as _,
                core::mem::size_of::<T>(),
            );
        }

        self.return_size = core::mem::size_of::<T>();

        Ok(())
    }
}

pub unsafe extern "C" fn dispatch_device(
    device: *mut DEVICE_OBJECT,
    irp: *mut IRP,
) -> NTSTATUS {
    let stack_location = IoGetCurrentIrpStackLocation(irp);

    // Get the device extension.
    let extension = (*device).DeviceExtension as *mut DeviceExtension;
    let vtable = (*extension).vtable;

    let (status, return_size) = match (*stack_location).MajorFunction as _ {
        IRP_MJ_CREATE => (match (*vtable).create {
            Some(create) => create(device),
            _ => STATUS_SUCCESS,
        }, 0),
        IRP_MJ_CLOSE => (match (*vtable).close {
            Some(close) => close(device),
            _ => STATUS_SUCCESS,
        }, 0),
        IRP_MJ_CLEANUP => (match (*vtable).cleanup {
            Some(cleanup) => cleanup(device),
            _ => STATUS_SUCCESS,
        }, 0),
        IRP_MJ_DEVICE_CONTROL => {
            let ioctl_num = (*stack_location).Parameters.DeviceIoControl.IoControlCode;
            let ptr = (*irp).AssociatedIrp.SystemBuffer;
            let read_size = (*stack_location).Parameters.DeviceIoControl.InputBufferLength as _;
            let write_size = (*stack_location).Parameters.DeviceIoControl.OutputBufferLength as _;
            let mut return_size = 0;

            let status = match (*vtable).ioctl {
                Some(ioctl) => ioctl(device, ioctl_num, ptr, read_size, write_size, &mut return_size),
                _ => STATUS_SUCCESS,
            };

            (status, return_size)
        }
        _ => (STATUS_INVALID_PARAMETER, 0),
    };

    (*irp).IoStatus.Information = return_size as _;
    (*irp).IoStatus.__bindgen_anon_1.Status = status;

    IoCompleteRequest(irp, IO_NO_INCREMENT as _);

    status
}
