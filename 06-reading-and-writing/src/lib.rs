#![no_std]

use windows_kernel_rs::{Access, Device, DeviceFlags, DeviceOperations, DeviceType, Driver, Error, IoRequest, kernel_module, KernelModule, SymbolicLink};

struct MyDevice {
    value: u32,
}

impl DeviceOperations for MyDevice {
    fn read(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        let mut user_ptr = request.user_ptr().unwrap();

        user_ptr.write(&self.value)?;

        request.complete(Ok(0));

        Ok(())
    }

    fn write(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        let user_ptr = request.user_ptr().unwrap();

        user_ptr.read(&mut self.value)?;

        request.complete(Ok(0));

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
            Access::NonExclusive,
            MyDevice { value: 0 },
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
