extern crate bindgen;
use std::env;
use std::path::PathBuf;

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> Result<(), Box<dyn std::error::Error>> {
    embuild::espidf::sysenv::output();

    let libdir_path = PathBuf::from("bme68x")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_paths = [libdir_path.join("bme68x.h")];

    let headers_paths_str: Vec<&str> = headers_paths
        .iter()
        .map(|path| path.to_str().expect("Path is not a valid string"))
        .collect();

    let obj_path = libdir_path.join("bme68x.o");
    let lib_path = libdir_path.join("libbme68x.a");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    println!("cargo:rustc-link-lib=bme68x");

    for path in &headers_paths {
        println!(
            "cargo:rerun-if-changed={}",
            path.to_str().expect("Path is not a valid string")
        );
    }

    if !std::process::Command::new("xtensa-esp32-elf-gcc")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg("-mlongcalls")
        .arg(libdir_path.join("bme68x.c"))
        .output()
        .expect("could not spawn `xtensa-esp32-elf-gcc`")
        .status
        .success()
    {
        panic!("could not compile object file");
    }

    if !std::process::Command::new("xtensa-esp32-elf-ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `xtensa-esp32-elf-ar`")
        .status
        .success()
    {
        panic!("could not emit library file");
    }

    let mut builder = bindgen::Builder::default();

    // Add each header file to the builder
    for path in headers_paths_str {
        builder = builder.header(path);
    }

    // Other options for the builder if needed
    let bindings = builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

    Ok(())
}
