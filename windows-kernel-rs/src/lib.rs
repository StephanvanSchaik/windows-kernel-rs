#![no_std]

#![feature(alloc_error_handler)]

extern crate alloc;

pub mod allocator;
pub mod error;
pub mod io;

pub use crate::error::Error;
