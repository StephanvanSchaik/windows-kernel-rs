use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let _file = File::open("\\??\\Example")?;

    Ok(())
}
