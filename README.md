# Dangling Pointer Bug in Rust
This repository demonstrates a common error in Rust: creating a dangling pointer by accessing a vector's memory after it has been deallocated.  The `bug.rs` file contains the erroneous code, while `bugSolution.rs` provides a corrected version.

The core issue is that the vector `v` is dropped at the end of its scope, making the pointer `ptr` invalid. Accessing this invalid pointer leads to undefined behavior, potentially causing crashes or data corruption.

The solution utilizes safe Rust techniques to avoid this error.