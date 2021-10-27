#![no_std]

use windows_kernel_rs::device::{
    Completion, Device, DeviceDoFlags, DeviceFlags, DeviceOperations, DeviceType, RequestError};
use windows_kernel_rs::println;
use windows_kernel_rs::request::{IoRequest};
use windows_kernel_rs::{Access, Driver, Error, kernel_module, KernelModule, SymbolicLink};

struct MyDevice;

impl DeviceOperations for MyDevice {
    fn create(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        println!("userspace opened the device");

        Ok(Completion::Complete(0, request))
    }

    fn close(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        println!("userspace closed the device");

        Ok(Completion::Complete(0, request))
    }

    fn cleanup(&mut self, _device: &Device, request: IoRequest) -> Result<Completion, RequestError> {
        println!("device is no longer in use by userspace");

        Ok(Completion::Complete(0, request))
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
