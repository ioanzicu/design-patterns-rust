fn main() {
    // Get current directory
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Tell Cargo to link against C library.
    // Assumes libmy_c_lib.a (or .lib) is in the project root or a known path.
    // Let's assume it is in the project root.
    // If it's in a subdirectory like "clib", use:
    // println!("cargo:rustc-link-search=native={}/clib", project_dir);
    println!("cargo:rustc-link-search=native={}", project_dir); // Search in project root

    // Link against the static library "my_c_lib"
    // Cargo will look for libmy_c_lib.a on Unix-like systems or my_c_lib.lib on Windows.
    println!("cargo:rustc-link-lib=static=my_c_lib");

    // If C lib had other dependencies, they would require to be linked too.
    // println!("cargo:rustc-link-lib=dylib=some_other_system_lib");

    // The Cargo to re-run this build script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    // If C library source changes, it will require to recompile it using `cc` crate.
}
