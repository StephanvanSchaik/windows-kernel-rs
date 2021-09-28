use windows_kernel_build::DirectoryType;

fn main() {
    // Get the path to the kernel libraries.
    let dir = windows_kernel_build::get_km_dir(DirectoryType::Library).unwrap();

    // Append the architecture based on our target.
    let target = std::env::var("TARGET").unwrap();

    let arch = if target.contains("x86_64") {
        "x64"
    } else if target.contains("i686") {
        "x86"
    } else {
        panic!("The target {} is currently not supported.", target);
    };

    let dir = dir.join(arch);

    // Specify the link path.
    println!("cargo:rustc-link-search=native={}", dir.to_str().unwrap());
}
