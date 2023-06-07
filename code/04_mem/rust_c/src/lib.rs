use std::ffi::CStr;

/// # Safety
/// Use a valid C-String!
#[no_mangle]
pub unsafe extern "C" fn hello(name: *const libc::c_char) {
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name = name_cstr.to_str().unwrap();
    println!("Hello {name}");
}