
extern crate bindgen;
extern crate cmake;
use cmake::Config;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=libvec/vec.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("libvec/vec.h")
        //----------I add from here
        // Not generate the layout test code
        .layout_tests(false)
        // Not derive `Debug, Clone, Copy, Default` trait by default
        .derive_debug(false)
        .derive_copy(false)
        .derive_default(false)
        // Enable C++ namespace support
        .enable_cxx_namespaces()
        // Add extra clang args for supporting `C++`
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .clang_arg("-stdlib=libc++")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        //------------ to here 
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build static library
    let dst = Config::new("libvec").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=vec");
}
