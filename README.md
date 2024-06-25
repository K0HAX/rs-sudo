# rs-sudo
This is a Rust program that I wrote to learn some unsafe Rust, and to learn how to build a SUID root executable in Rust.

## Disclaimer
While I believe this program does not contain undefined behavior, it does contain `unsafe` code in order to interface with the kernel.

I read the man pages for `setuid`, and `getuid` and did my best to implement safe abstractions for these syscalls.

For `errno` I coppied the code from the Rust standard library, in `std/src/sys/pal/unix/os.rs`.
