use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("\\??\\Example")?;

    file.write_all("Hello, world!".as_bytes())?;

    let mut data = vec![0u8; 4096];
    let size = file.read(&mut data)?;
    
    match std::str::from_utf8(&data[..size]) {
        Ok(s) => println!("read {} bytes: \"{}\"", size, s),
        _ => println!("read {} bytes: {:x?}", size, &data[..size]),
    }

    Ok(())
}
