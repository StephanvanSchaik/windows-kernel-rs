use std::path::PathBuf;
use thiserror::Error;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("cannot find the directory")]
    DirectoryNotFound,
}

pub enum DirectoryType {
    Include,
    Library,
}

/// Retrieves the path to the Windows Kits directory. The default should be
/// `C:\Program Files (x86)\Windows Kits\10`.
pub fn get_windows_kits_dir() -> Result<PathBuf, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let dir: String = hklm.open_subkey(key)?.get_value("KitsRoot10")?;

    Ok(dir.into())
}

/// Retrieves the path to the kernel mode libraries. The path may look something like:
/// `C:\Program Files (x86)\Windows Kits\10\lib\10.0.18362.0\km`.
pub fn get_km_dir(dir_type: DirectoryType) -> Result<PathBuf, Error> {
    // We first append lib to the path and read the directory..
    let dir = get_windows_kits_dir()?
        .join(match dir_type {
            DirectoryType::Include => "Include",
            DirectoryType::Library => "Lib",
        })
        .read_dir()?;

    // In the lib directory we may have one or more directories named after the version of Windows,
    // we will be looking for the highest version number.
    let max_libdir = dir
        .filter_map(|dir| dir.ok())
        .map(|dir| dir.path())
        .filter(|dir| {
            dir.components()
                .last()
                .and_then(|c| c.as_os_str().to_str())
                .map(|c| c.starts_with("10.") && dir.join("km").is_dir())
                .unwrap_or(false)
        })
        .max()
        .ok_or_else(|| Error::DirectoryNotFound)?;

    // Finally append km to the path to get the path to the kernel mode libraries.
    Ok(max_libdir.join("km"))
}

fn generate_intrin() {
    println!("cargo:rerun-if-changed=src/wrapper_intrin.h");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();

    cc::Build::new()
        .flag("/kernel")
        .include(include_dir)
        .file("src/wrapper_intrin.c")
        .compile("wrapper_intrin");
}

fn generate_ntoskrnl() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rustc-link-lib=ntoskrnl");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();
    let out_path = PathBuf::from(
        std::env::var_os("OUT_DIR")
            .expect("the environment variable OUT_DIR is undefined")
    );

    bindgen::Builder::default()
        .header("src/wrapper.h")
        .use_core()
        .derive_debug(false)
        .layout_tests(false)
        .ctypes_prefix("cty")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type(".*")
        .allowlist_function(".*")
        .allowlist_recursively(false)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("ntoskrnl.rs"))
        .unwrap();

    cc::Build::new()
        .flag("/kernel")
        .include(include_dir)
        .file("src/wrapper.c")
        .compile("wrapper_ntoskrnl");
}

fn generate_netio() {
    println!("cargo:rerun-if-changed=src/wrapper_netio.h");
    println!("cargo:rustc-link-lib=netio");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();
    let out_path = PathBuf::from(
        std::env::var_os("OUT_DIR")
            .expect("the environment variable OUT_DIR is undefined")
    );

    bindgen::Builder::default()
        .header("src/wrapper.h")
        .use_core()
        .derive_debug(false)
        .layout_tests(false)
        .ctypes_prefix("cty")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_type(".*")
        .allowlist_function(".*")
        .allowlist_recursively(false)
        .generate()
        .unwrap()
        .write_to_file(out_path.join("netio.rs"))
        .unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();
    let out_path = PathBuf::from(
        std::env::var_os("OUT_DIR")
            .expect("the environment variable OUT_DIR is undefined")
    );

    bindgen::Builder::default()
        .header("src/wrapper.h")
        .use_core()
        .derive_debug(false)
        .layout_tests(false)
        .ctypes_prefix("cty")
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .ignore_functions()
        .generate()
        .unwrap()
        .write_to_file(out_path.join("base.rs"))
        .unwrap();

    #[cfg(feature = "intrin")]
    generate_intrin();

    #[cfg(feature = "ntoskrnl")]
    generate_ntoskrnl();

    #[cfg(feature = "netio")]
    generate_netio();
}
