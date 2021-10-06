use std::fs::OpenOptions;
use std::os::windows::io::AsRawHandle;
use winioctl::{DeviceType, Error};
use winioctl::{ioctl_none, ioctl_read, ioctl_write};

const IOCTL_PRINT_VALUE: u32 = 0x800;
const IOCTL_READ_VALUE:  u32 = 0x801;
const IOCTL_WRITE_VALUE: u32 = 0x802;

ioctl_none!(ioctl_print_value, DeviceType::Unknown, IOCTL_PRINT_VALUE);
ioctl_read!(ioctl_read_value, DeviceType::Unknown, IOCTL_READ_VALUE, i32);
ioctl_write!(ioctl_write_value, DeviceType::Unknown, IOCTL_WRITE_VALUE, i32);

fn main() -> Result<(), Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("\\??\\Example")?;
    let mut value = 0;

    unsafe {
        ioctl_read_value(file.as_raw_handle(), &mut value)?;
    }

    value += 1;

    unsafe {
        ioctl_write_value(file.as_raw_handle(), &value)?;
    }

    unsafe {
        ioctl_print_value(file.as_raw_handle())?;
    }

    Ok(())
}
