extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("bluez_sbc")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    let library_name = "bluezsbc";
    // This is the path to the static library file.
    let lib_path = libdir_path.join(format!("lib{}.a", library_name));


    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib={}", library_name);

    let source_files = [
        "sbc.c",
        "sbc_primitives.c",
        "sbcdec.c",
        "sbcenc.c",
        "sbcinfo.c",
        "sbctester.c",
    ];
    let header_files = [
        // "sbc_primitives.h",
        "formats.h",
        "sbc.h",
    ];

    for header_file in header_files {
        // This is the path to the `c` headers file.
        let header_path = libdir_path.join(header_file);
        let header_path_str = header_path.to_str().expect("Path is not a valid string");

        // Tell cargo to invalidate the built crate whenever the header changes.
        println!("cargo:rerun-if-changed={}", header_path_str);
    }

    // This is the path to the intermediate object file for our library.
    let mut obj_paths = Vec::new();

    for source_file in source_files {
        let obj_file = source_file.replace(".c", ".o");
        let obj_path = libdir_path.join(obj_file);

        // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
        // Unwrap if it is not possible to spawn the process.
        compile(&libdir_path, &obj_path, source_file);
        obj_paths.push(obj_path);
    }

    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    link(&obj_paths, lib_path);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default();

    for header_file in header_files {
        let header_path = libdir_path.join(header_file);
        let header_path_str = header_path.to_str().expect("Path is not a valid string");
        // The input header we would like to generate
        // bindings for.
        builder = builder.header(header_path_str);
    }

    let bindings = builder
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // no_std
        .use_core()
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

fn compile(libdir_path: &PathBuf, obj_path: &PathBuf, source_file: &str) {
    if !std::process::Command::new("clang")
        .arg("-std=gnu11")
        .arg("-DHAVE_CONFIG_H")
        .arg(format!("-I{}", libdir_path.display()))
        .arg("-c")
        .arg(libdir_path.join(source_file))
        .arg("-o")
        .arg(&obj_path)
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }
}

fn link(obj_files: &[PathBuf], lib_path: PathBuf) {
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .args(obj_files)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }
}
