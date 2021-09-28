#![no_std]

use windows_kernel_rs::{Access, Device, DeviceFlags, DeviceOperations, DeviceType, Driver, Error, IoRequest, kernel_module, KernelModule, println, SymbolicLink};

struct MyDevice;

impl DeviceOperations for MyDevice {
    fn create(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        println!("userspace opened the device");

        request.complete(Ok(0));

        Ok(())
    }

    fn close(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        println!("userspace closed the device");

        request.complete(Ok(0));

        Ok(())
    }

    fn cleanup(&mut self, _device: &Device, request: &IoRequest) -> Result<(), Error> {
        println!("device is no longer in use by userspace");

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
            MyDevice,
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
