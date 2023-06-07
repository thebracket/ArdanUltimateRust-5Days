fn allocate_memory_with_libc() {
    unsafe {
        // Allocate memory with libc (one 32-bit integer)
        let my_num: *mut i32 = libc::malloc(std::mem::size_of::<i32>() as libc::size_t) as *mut i32;
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }

        // Set the allocated variable - dereference the pointer and set to 42
        *my_num = 42;
        assert_eq!(42, *my_num);

        // Free the memory with libc - this is NOT automatic
        libc::free(my_num as *mut libc::c_void);
    }
}

fn allocate_memory_with_rust() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        // Allocate memory with Rust. It's safer to force alignment.
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);

        // Access as a `u16`, even though it's just a byte array
        let num: *mut u16 = std::mem::transmute(ptr);

        // Set the allocated variable - dereference the pointer and set to 42
        *num = 42;
        assert_eq!(42, *num);

        // Free the memory - this is not automatic
        dealloc(ptr, layout);
    }
}

fn main() {
    allocate_memory_with_libc();
    allocate_memory_with_rust();
}
