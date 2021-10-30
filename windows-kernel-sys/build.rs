use std::path::PathBuf;
use windows_kernel_build::DirectoryType;

use bindgen::callbacks::*;

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        Some(match name {
            "TRUE" | "FALSE" => IntKind::UChar,
            _ => return None,
        })
    }
}

fn generate_base() {
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let include_dir = windows_kernel_build::get_km_dir(DirectoryType::Include).unwrap();
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
        .parse_callbacks(Box::new(Callbacks))
        .ignore_functions()
        .generate()
        .unwrap()
        .write_to_file(out_path.join("base.rs"))
        .unwrap();
}

#[cfg(feature = "intrin")]
fn generate_intrin() {
    println!("cargo:rerun-if-changed=src/wrapper_intrin.c");

    let include_dir = windows_kernel_build::get_km_dir(DirectoryType::Include).unwrap();

    cc::Build::new()
        .flag("/kernel")
        .include(include_dir)
        .file("src/wrapper_intrin.c")
        .compile("wrapper_intrin");
}

#[cfg(not(feature = "intrin"))]
fn generate_intrin() {
}

#[cfg(feature = "ntoskrnl")]
fn generate_ntoskrnl() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/wrapper.c");
    println!("cargo:rustc-link-lib=ntoskrnl");

    let include_dir = windows_kernel_build::get_km_dir(DirectoryType::Include).unwrap();
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

#[cfg(not(feature = "ntoskrnl"))]
fn generate_ntoskrnl() {
}

#[cfg(feature = "netio")]
fn generate_netio() {
    println!("cargo:rerun-if-changed=src/wrapper_netio.h");
    println!("cargo:rustc-link-lib=netio");

    let include_dir = windows_kernel_build::get_km_dir(DirectoryType::Include).unwrap();
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

#[cfg(not(feature = "netio"))]
fn generate_netio() {
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    generate_base();
    generate_intrin();
    generate_ntoskrnl();
    generate_netio();
}
