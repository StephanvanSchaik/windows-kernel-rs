use alloc::boxed::Box;
use crate::device::{Access, Device, DeviceExtension, DeviceDoFlags, DeviceFlags, DeviceOperations, DeviceOperationsVtable, DeviceType};
use crate::error::{Error, IntoResult};
use crate::string::create_unicode_string;
use widestring::U16CString;
use windows_kernel_sys::base::DRIVER_OBJECT;
use windows_kernel_sys::ntoskrnl::{IoCreateDevice};

pub struct Driver {
    pub(crate) raw: *mut DRIVER_OBJECT,
}

impl Driver {
    pub unsafe fn from_raw(raw: *mut DRIVER_OBJECT) -> Self {
        Self {
            raw,
        }
    }

    pub unsafe fn as_raw(&self) -> *const DRIVER_OBJECT {
        self.raw as _
    }

    pub unsafe fn as_raw_mut(&mut self) -> *mut DRIVER_OBJECT {
        self.raw as _
    }

    pub fn create_device<T>(
        &mut self,
        name: &str,
        device_type: DeviceType,
        device_flags: DeviceFlags,
        device_do_flags: DeviceDoFlags,
        access: Access,
        data: T,
    ) -> Result<Device, Error>
    where
        T: DeviceOperations
    {
        // Box the data.
        let data = Box::new(data);

        // Convert the name to UTF-16 and then create a UNICODE_STRING.
        let name = U16CString::from_str(name).unwrap();
        let mut name = create_unicode_string(name.as_slice());

        // Create the device.
        let mut device = core::ptr::null_mut();

        unsafe {
            IoCreateDevice(
                self.raw,
                core::mem::size_of::<DeviceExtension>() as u32,
                &mut name,
                device_type.into(),
                device_flags.bits(),
                access.is_exclusive() as _,
                &mut device,
            )
        }.into_result()?;

        unsafe {
            (*device).Flags |= device_do_flags.bits();
        }

        let device = unsafe { Device::from_raw(device) };

        // Store the boxed data and vtable.
        let extension = device.extension_mut();
        extension.device_type = device_type;
        extension.vtable = &DeviceOperationsVtable::<T>::VTABLE;
        extension.data = Box::into_raw(data) as *mut cty::c_void;

        Ok(device)
    }
}
