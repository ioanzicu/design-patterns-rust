extern crate cc;

fn main() {
    // Compile my_c_code.c and link it into Rust executable/library
    // It will create a static library (libmy_c_code.a) and link it.
    cc::Build::new()
        .file("src/my_c_code.c") // path to C source file
        .compile("my_c_code"); // output library name will be libmy_c_code.a (or .lib)

    // In case of linking against a pre-compiled library like libcustom.so is in /opt/custom_lib/lib:
    // println!("cargo:rustc-link-lib=static=custom"); // Link against libcustom.a
    // OR for a dynamic library:
    // pritnln!("cargo:rustc-link-lib=dylib=custom"); // Link against libcustom.so/dylib/dll
}
