#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use windows_kernel_rs::device::{
    Completion, Device, DeviceDoFlags, DeviceFlags, DeviceOperations, DeviceType, RequestError};
use windows_kernel_rs::request::{ReadRequest, WriteRequest};
use windows_kernel_rs::{Access, Driver, Error, kernel_module, KernelModule, SymbolicLink};

struct MyDevice {
    data: Vec<u8>,
}

impl DeviceOperations for MyDevice {
    fn read(&mut self, _device: &Device, request: ReadRequest) -> Result<Completion, RequestError> {
        let mut user_ptr = request.user_ptr();
        let slice = user_ptr.as_mut_slice();

        let offset = (request.offset() as usize).min(self.data.len());
        let size = slice.len().min(self.data.len() - offset);

        slice[0..size].copy_from_slice(&self.data[offset..offset + size]);

        Ok(Completion::Complete(size as u32, request.into()))
    }

    fn write(&mut self, _device: &Device, request: WriteRequest) -> Result<Completion, RequestError> {
        let user_ptr = request.user_ptr();

        if request.offset() > 0 {
            return Err(RequestError(Error::END_OF_FILE, request.into()))?;
        }

        let slice = user_ptr.as_slice();
        let size = slice.len().min(4096);

        self.data = slice[0..size].to_vec();

        Ok(Completion::Complete(size as u32, request.into()))
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
                data: vec![],
            },
        )?;
        let symbolic_link = SymbolicLink::new("\\??\\Example", "\\Device\\Example")?;

        Ok(Module {
            _device: device,
            _symbolic_link: symbolic_link,
        })
    }
}

kernel_module!(Module);
