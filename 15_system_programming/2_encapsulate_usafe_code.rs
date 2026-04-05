/// # Safety
///
/// The caller *must* ensure that the pointer `ptr` is valid,
/// non-null and points to a valid `i32`.
pub unsafe fn do_something_with_ptr(ptr: *mut i32) {
    // This check runs only in debug builds. It helps catch
    // incorrect usage of this unsafe function during development.
    // In a release build, this check disappears, and passing a null
    // pointer would lead to undefined behavior.
    debug_assert!(
        !ptr.is_null(),
        "do_something_with_ptr called with a null pointer!"
    );
    // The actual unsafe operation
    *ptr += 1;
}

fn main() {
    let mut x = 5;
    let ptr = &mut x as *mut i32;
    unsafe {
        do_something_with_ptr(ptr);
    }
    println!("x is now: {}", x); // 6
                                 // By running in a debug build the
                                 // unsafe {
                                 //     do_something_with_ptr(std::ptr::null_mut()); // This would panic!
                                 // }
}
