extern crate libc;
use libc::c_uchar;
use std::ffi::CString;

#[link(name="test")]
extern {
    fn print_stuff(array_of_strings: *const *const c_uchar);
}


fn main() {
    println!("Hello, world!");
    let strs = vec!["Hey, it works.", "This", "is", "definitely", "a very cool test", "of", "ffi"];
    
    let mut c_strs: Vec<CString> = Vec::with_capacity(strs.len());
    let mut c_str_ptrs: Vec<*const c_uchar> = Vec::with_capacity(strs.len());
    for string in strs {

        //create new CString and take ownership of it in c_strs
        c_strs.push(CString::new(string).unwrap());

        //create a pointer to that CString's raw data and store it in c_str_ptrs
        c_str_ptrs.push(c_strs[c_strs.len() - 1].as_ptr() as *const c_uchar);
    }

    unsafe { print_stuff(c_str_ptrs.as_ptr()); } //call print_stuff

    /* 
     * At this point, c_strs and c_str_ptrs will fall out of scope.
     *
     * In some undetermined order,
     * c_str_ptrs will then destroy its array of pointers
     * c_strs will deallocate each CString and then destroy its own array
     * so there will be no memory leaks, and no double frees.
     *
     * Raw pointers are not covered by the memory safety guarantees of Rust,
     * so they can continue to exist even beyond the lifetime of the object
     * they refer to. They can be created in safe code, but they cannot be
     * dereferenced or otherwise manipulated in safe code.
     *
     * It is our job to make sure the memory they point to outlives the pointers.
     *
     * Safe Rust has no use for raw pointers whatsoever, since they can't be
     * dereferenced, so the only time they're likely to be seen is in FFIs
     * or operating system development, where they get used in Unsafe Rust blocks.
     *
     */
}
