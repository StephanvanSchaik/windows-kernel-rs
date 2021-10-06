#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use windows_kernel_rs::{Access, Device, DeviceFlags, DeviceOperations, DeviceType, Driver, Error, kernel_module, KernelModule, SymbolicLink, ReadRequest, WriteRequest};

struct MyDevice {
    data: Vec<u8>,
}

impl DeviceOperations for MyDevice {
    fn read(&mut self, _device: &Device, request: &ReadRequest) -> Result<(), Error> {
        let mut user_ptr = request.user_ptr();
        let size = user_ptr.write_size().min(self.data.len());
        let slice = user_ptr.as_mut_slice();

        slice[0..size].copy_from_slice(&self.data[0..size]);

        request.complete(Ok(size as u32));

        Ok(())
    }

    fn write(&mut self, _device: &Device, request: &WriteRequest) -> Result<(), Error> {
        let user_ptr = request.user_ptr();
        let slice = user_ptr.as_slice();

        self.data = slice[0..slice.len().min(4096)].to_vec();

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
            MyDevice {
                data: vec![],
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
