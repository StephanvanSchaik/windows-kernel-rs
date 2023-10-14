#![no_std]

pub mod base;

#[cfg(feature = "intrin")]
pub mod intrin;
#[cfg(feature = "netio")]
pub mod netio;
#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

pub use cty::*;
