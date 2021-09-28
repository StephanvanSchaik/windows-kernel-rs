use bitflags::bitflags;
use crate::error::Error;
use crate::ioctl::ControlCode;
use crate::user_ptr::UserPtr;
use windows_kernel_sys::base::{IO_NO_INCREMENT, IO_STACK_LOCATION, IRP, STATUS_SUCCESS};
use windows_kernel_sys::base::{IRP_MJ_READ, IRP_MJ_WRITE, IRP_MJ_DEVICE_CONTROL};
use windows_kernel_sys::base::_MM_PAGE_PRIORITY as MM_PAGE_PRIORITY;
use windows_kernel_sys::ntoskrnl::{IoCompleteRequest, IoGetCurrentIrpStackLocation};
use windows_kernel_sys::ntoskrnl::{MmGetMdlByteCount, MmGetMdlByteOffset, MmGetSystemAddressForMdlSafe};

bitflags! {
    pub struct IrpFlags: u32 {
        const NOCACHE = windows_kernel_sys::base::IRP_NOCACHE;
        const PAGING_IO = windows_kernel_sys::base::IRP_PAGING_IO;
        const MOUNT_COMPLETION =  windows_kernel_sys::base::IRP_MOUNT_COMPLETION;
        const SYNCHRONOUS_API = windows_kernel_sys::base::IRP_SYNCHRONOUS_API;
        const ASSOCIATED_IRP = windows_kernel_sys::base::IRP_ASSOCIATED_IRP;
        const BUFFERED_IO = windows_kernel_sys::base::IRP_BUFFERED_IO;
        const DEALLOCATE_BUFFER = windows_kernel_sys::base::IRP_DEALLOCATE_BUFFER;
        const INPUT_OPERATION = windows_kernel_sys::base::IRP_INPUT_OPERATION;
        const SYNCHRONOUS_PAGING_IO = windows_kernel_sys::base::IRP_SYNCHRONOUS_PAGING_IO;
        const CREATE_OPERATION = windows_kernel_sys::base::IRP_CREATE_OPERATION;
        const READ_OPERATION = windows_kernel_sys::base::IRP_READ_OPERATION;
        const WRITE_OPERATION = windows_kernel_sys::base::IRP_WRITE_OPERATION;
        const CLOSE_OPERATION = windows_kernel_sys::base::IRP_CLOSE_OPERATION;
        const DEFER_IO_COMPLETION = windows_kernel_sys::base::IRP_DEFER_IO_COMPLETION;
        const OB_QUERY_NAME = windows_kernel_sys::base::IRP_OB_QUERY_NAME;
        const HOLD_DEVICE_QUEUE = windows_kernel_sys::base::IRP_HOLD_DEVICE_QUEUE;
        const UM_DRIVER_INITIATED_IO = windows_kernel_sys::base::IRP_UM_DRIVER_INITIATED_IO;
    }
}

pub struct IoRequest {
    irp: *mut IRP,    
}

impl IoRequest {
    pub unsafe fn from_raw(irp: *mut IRP) -> Self {
        Self { irp }
    }

    pub fn irp(&self) -> &IRP {
        unsafe { &*self.irp }
    }

    pub fn irp_mut(&self) -> &mut IRP {
        unsafe { &mut *self.irp }
    }

    pub fn flags(&self) -> IrpFlags {
        IrpFlags::from_bits(self.irp().Flags)
            .unwrap_or(IrpFlags::empty())
    }

    pub fn stack_location(&self) -> &IO_STACK_LOCATION {
        unsafe { &*IoGetCurrentIrpStackLocation(self.irp_mut()) }
    }

    pub fn major(&self) -> u8 {
        self.stack_location().MajorFunction
    }

    pub fn complete(&self, value: Result<u32, Error>) {
        let irp = self.irp_mut();
        
        match value {
            Ok(value) => {
                irp.IoStatus.Information = value as _;
                irp.IoStatus.__bindgen_anon_1.Status = STATUS_SUCCESS;
            }
            Err(error) => {
                irp.IoStatus.Information = 0;
                irp.IoStatus.__bindgen_anon_1.Status = error.to_kernel_errno();
            }
        }

        unsafe {
            IoCompleteRequest(irp, IO_NO_INCREMENT as _);
        }
    }

    pub fn control_code(&self) -> Option<ControlCode> {
        let stack_location = self.stack_location();

        match self.major() as _ {
            IRP_MJ_DEVICE_CONTROL => Some(unsafe {
                stack_location.Parameters.DeviceIoControl.IoControlCode.into()
            }),
            _ => None,
        }
    }

    pub fn user_ptr(&self) -> Option<UserPtr> {
        let stack_location = self.stack_location();
        let irp = self.irp();
        let flags = self.flags();

        match self.major() as _ {
            IRP_MJ_READ => Some(if flags.contains(IrpFlags::BUFFERED_IO) {
                let ptr = unsafe { irp.AssociatedIrp.SystemBuffer };
                let size = unsafe { stack_location.Parameters.Read }.Length as usize;

                unsafe { UserPtr::new(ptr, 0, size) }
            } else {
                let ptr = unsafe {
                    MmGetSystemAddressForMdlSafe(irp.MdlAddress, MM_PAGE_PRIORITY::HighPagePriority as _)
                };

                let size = unsafe { MmGetMdlByteCount(irp.MdlAddress) } as usize;

                unsafe { UserPtr::new(ptr, 0, size) }
            }),
            IRP_MJ_WRITE => Some(if flags.contains(IrpFlags::BUFFERED_IO) {
                let ptr = unsafe { irp.AssociatedIrp.SystemBuffer };
                let size = unsafe { stack_location.Parameters.Read }.Length as usize;

                unsafe { UserPtr::new(ptr, size, 0) }
            } else {
                let ptr = unsafe {
                    MmGetSystemAddressForMdlSafe(irp.MdlAddress, MM_PAGE_PRIORITY::HighPagePriority as _)
                };

                let size = unsafe { MmGetMdlByteCount(irp.MdlAddress) } as usize;

                unsafe { UserPtr::new(ptr, size, 0) }
            }),
            IRP_MJ_DEVICE_CONTROL => {
                let ptr = unsafe { irp.AssociatedIrp.SystemBuffer };
                let input_size = unsafe {
                    stack_location.Parameters.DeviceIoControl.InputBufferLength
                } as usize;
                let output_size = unsafe {
                    stack_location.Parameters.DeviceIoControl.OutputBufferLength
                } as usize;

                Some(unsafe { UserPtr::new(ptr, input_size, output_size) })
            },
            _ => None,
        }
    }

    pub fn offset(&self) -> Option<i64> {
        let stack_location = self.stack_location();
        let irp = self.irp();
        let flags = self.flags();

        match self.major() as _ {
            IRP_MJ_READ => Some(if flags.contains(IrpFlags::BUFFERED_IO) {
                unsafe { stack_location.Parameters.Read.ByteOffset.QuadPart }
            } else {
                (unsafe { MmGetMdlByteOffset(irp.MdlAddress) }) as i64
            }),
            IRP_MJ_WRITE => Some(if flags.contains(IrpFlags::BUFFERED_IO) {
                unsafe { stack_location.Parameters.Write.ByteOffset.QuadPart }
            } else {
                (unsafe { MmGetMdlByteOffset(irp.MdlAddress) }) as i64
            }),
            _ => None,
        }
    }
}
