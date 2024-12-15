fn main() {
    let mut v = vec![1, 2, 3];
    let mut value = 0; 
    // Use a temporary variable to avoid issues
    unsafe {
        value = *v.as_mut_ptr();
        *v.as_mut_ptr() = 42;
    }
    println!("Value: {}",value);
    println!("Vector: {:?}",v);
}