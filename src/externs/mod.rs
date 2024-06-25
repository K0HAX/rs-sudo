use crate::types::uid_t;
// These functions are defined in libc. Here, we import them into Rust.
extern "C" {
    pub(super) fn getuid() -> uid_t;
    pub(super) fn setuid(uid: uid_t) -> i32;
}

