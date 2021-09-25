#![no_std]

use windows_kernel_rs::{Driver, Error, kernel_module, KernelModule, println};

struct Module;

impl KernelModule for Module {
    fn init(_: Driver, _: &str) -> Result<Self, Error> {
        println!("Hello, world!");

        Ok(Module)
    }

    fn cleanup(&mut self, _: Driver) {
        println!("Bye bye!");
    }
}

kernel_module!(Module);
