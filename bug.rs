fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    // v goes out of scope here, invalidating the pointer
    unsafe {
        *ptr = 42;
    }
}