# windows-kernel-rs

**Note**: this is still work in progress!

This is a Windows kernel framework in Rust that consists of [windows-kernel-sys](windows-kernel-sys), a crate that provides low-level unsafe bindings generated using [bindgen](https://github.com/rust-lang/rust-bindgen), and [windows-kernel-rs](windows-kernel-rs), a crate that provides safe abstractions in Rust on top.

In addition, there are articles that covers implementing Windows kernel drivers in Rust from the ground up, including corresponding examples provided as part of this repository:

1. [Prerequisites](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-prerequisites)
2. [Hello World](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-hello-world) - [02-hello-world](02-hello-world)
3. [Generating Bindings](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-generating-bindings) - [03-generating-bindings](03-generating-bindings)
4. [Safe Framework](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-safe-framework) - [04-safe-framework](04-safe-framework)
5. [Creating Devices](https://codentium.com/guides/windows-dev/windows-drivers-in-rust-creating-devices/) - [05-creating-devices](05-creating-devices)
