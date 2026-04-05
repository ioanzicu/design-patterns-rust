use std::ffi::{CString, NulError};
use std::fmt;
use std::os::raw::{c_char, c_int};

// Declare the C functions we wasn to call
unsafe extern "C" {
    unsafe fn multiply(a: c_int, b: c_int) -> c_int;
    unsafe fn greet_person(name: *const c_char);
}

// Define a custom error type for safe wrapper module
#[derive(Debug)]
pub enum CLibError {
    StringConversion(NulError), // Error from CString::new if string has interior nulls
                                // Other potential C library error here
}

impl fmt::Display for CLibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CLibError::StringConversion(e) => {
                write!(f, "Failed to convert Rust string to C string: {}", e)
            }
        }
    }
}

impl std::error::Error for CLibError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CLibError::StringConversion(e) => Some(e),
        }
    }
}

// Safe Rust wrapper functions
mod my_c_lib_wrapper {
    use super::*; // To access extern "C" block, CString, CLibError
    pub fn safe_multiply(a: i32, b: i32) -> i32 {
        // The call to the extern "C" function must be in an unsafe block.
        unsafe { multiply(a as c_int, b as c_int) as i32 }
    }

    pub fn safe_greet(name: &str) -> Result<(), CLibError> {
        // Convert Rust &str to CString (null-terminated)
        match CString::new(name) {
            Ok(c_name) => {
                // Call C function within an unsafe block.
                // We are responsible to check if c_name.as_ptr() is valid.
                unsafe {
                    greet_person(c_name.as_ptr());
                }
                Ok(())
            }
            Err(e) => Err(CLibError::StringConversion(e)),
        }
    }
}

fn main() {
    println!("---  Testing FFI with Custom C library  ---");
    let num1 = 11;
    let num2 = 3;
    let product = my_c_lib_wrapper::safe_multiply(num1, num2);
    println!("Rust calling C multiply({}, {}): {}", num1, num2, product); // 33

    let name_to_greet = "Rustacean via FFI";
    match my_c_lib_wrapper::safe_greet(name_to_greet) {
        Ok(_) => println!("Greeting sent to C library successfully."),
        Err(e) => eprintln!("Error sending greeting: {}", e),
    }

    // Test with a name that would casue CString::new to fail (if it had interior null)
    let problematic_name = "Rust\0FFI";
    if let Err(e) = my_c_lib_wrapper::safe_greet(problematic_name) {
        eprintln!("Correctly handled error form problematic name: {}", e);
    }
}
