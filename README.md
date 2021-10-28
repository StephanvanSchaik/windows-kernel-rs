# windows-kernel-rs

**Note**: this is still work in progress!

This is a Windows kernel framework in Rust that consists of [windows-kernel-sys](windows-kernel-sys), a crate that provides low-level unsafe bindings generated using [bindgen](https://github.com/rust-lang/rust-bindgen), and [windows-kernel-rs](windows-kernel-rs), a crate that provides safe abstractions in Rust on top.

## Features

To give you an idea of whether these crates are useful to you, here is a non-exhaustive overview of the features that are currently supported and that are more or less planned:

* [x] `KernelModule` to provide safe entry and exit points to your driver.
* [x] Batteries included: panic handler, global allocator, etc.
* [x] Rust error handling using `Result`.
* [x] Device API to quickly set up with devices with a trait to provide the various callbacks.
* [x] Support for reading from and writing to devices.
* [x] Support for handling device I/O controls.
* [x] Basic safe abstraction on top of I/O Request Packets (IRPs) using Rust ownership to model their lifetimes.
* [x] Basic support for Memory Descriptor Lists (MDLs).
* [x] Version API to query the current version of Microsoft Windows.
* [x] Affinity API to run closures on a specific CPU or all CPUs in the system.
* [x] Device API
* [x] `FastMutex` (similar to [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)) based on the `FAST_MUTEX` API.
* [x] `PushLock` (similar to [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)) based on the `EX_PUSH_LOCK` API.
* [x] Abstraction for processes and process attachments to execute code in the context of a process.
* [x] Abstraction for sections.
* [x] Basic x86-64 intrinsics.
* [ ] More complete model of IRP handling.
* [ ] Support for x86 and AArch64.

## Articles

**Note**: this framework may progress faster than I can keep my articles up-to-date. They may currently be due for a bit of a rewrite to reflect some of the changes that made it into this repository since I have written the articles.

In addition, there are articles that cover implementing Windows kernel drivers in Rust from the ground up, including corresponding examples provided as part of this repository:

1. [Prerequisites](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-prerequisites)
2. [Hello World](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-hello-world) - [02-hello-world](02-hello-world)
3. [Generating Bindings](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-generating-bindings) - [03-generating-bindings](03-generating-bindings)
4. [Safe Framework](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-safe-framework) - [04-safe-framework](04-safe-framework)
5. [Creating Devices](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-creating-devices/) - [05-creating-devices](05-creating-devices) - [user/05-creating-devices](user/05-creating-devices)
6. [Reading and Writing](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-reading-and-writing/) - [06-reading-and-writing](06-reading-and-writing) - [user/06-reading-and-writing](user/06-reading-and-writing)
7. [I/O Controls](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-io-controls/) - [07-io-controls](07-io-controls) - [user/07-io-controls](user/07-io-controls)
