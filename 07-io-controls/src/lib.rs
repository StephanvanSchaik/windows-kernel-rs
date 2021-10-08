#![no_std]

use windows_kernel_rs::{Access, ControlCode, Device, DeviceDoFlags, DeviceFlags, DeviceOperations, DeviceType, Driver, Error, IoControlRequest, kernel_module, KernelModule, println, RequiredAccess, SymbolicLink, TransferMethod};

struct MyDevice {
    value: u32,
}

const IOCTL_PRINT_VALUE: u32 = 0x800;
const IOCTL_READ_VALUE:  u32 = 0x801;
const IOCTL_WRITE_VALUE: u32 = 0x802;

impl DeviceOperations for MyDevice {
    fn ioctl(&mut self, _device: &Device, request: &IoControlRequest) -> Result<(), Error> {
        match request.function() {
            (_, IOCTL_PRINT_VALUE) => {
                println!("value: {}", self.value);

                request.complete(Ok(0));
            }
            (RequiredAccess::READ_DATA, IOCTL_READ_VALUE) => {
                let mut user_ptr = request.user_ptr();

                user_ptr.write(&self.value)?;

                request.complete(Ok(core::mem::size_of::<u32>() as u32))
            }
            (RequiredAccess::WRITE_DATA, IOCTL_WRITE_VALUE) => {
                let user_ptr = request.user_ptr();

                self.value = user_ptr.read()?;

                request.complete(Ok(0))
            }
            _ => {
                return Err(Error::INVALID_PARAMETER);
            }
        }

        Ok(())
    }
}

struct Module {
    _device: Device,
    _symbolic_link: SymbolicLink,
}

impl KernelModule for Module {
    fn init(mut driver: Driver, _: &str) -> Result<Self, Error> {
        let device = driver.create_device(
            "\\Device\\Example",
            DeviceType::Unknown,
            DeviceFlags::SECURE_OPEN,
            DeviceDoFlags::DO_BUFFERED_IO,
            Access::NonExclusive,
            MyDevice {
                value: 0,
            },
        )?;
        let symbolic_link = SymbolicLink::new("\\??\\Example", "\\Device\\Example")?;

        Ok(Module {
            _device: device,
            _symbolic_link: symbolic_link,
        })
    }

    fn cleanup(&mut self, _driver: Driver) {
    }
}

kernel_module!(Module);
