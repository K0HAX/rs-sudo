use std::convert::From;
use std::error::Error;
use std::fmt;

use crate::errno;

/// A Rust error type for the possible errors that can be thrown when calling setuid()
#[derive(Debug)]
pub enum SetUidError {
    /// The call would change the caller's real UID (i.e., uid does not match the caller’s real
    /// UID), but there was a temporary failure allocating the necessary kernel data structures.
    EAGAIN,

    /// The user ID specified in uid is not valid in this user namespace.
    EINVAL,

    /// The user is not privileged (Linux: does not have the CAP_SETUID capability in its user
    /// namespace) and uid does not match the real UID or saved set-user-ID of the calling process.
    EPERM,

    /// Unknown error code
    Unknown(i32),
}

/// Convert from the errno `i32` type to our SetUidError type.
/// The `i32` values are defined in /usr/include/asm-generic/errno-base.h
impl From<i32> for SetUidError {
    fn from(item: i32) -> Self {
        match item {
            1 => SetUidError::EPERM,
            11 => SetUidError::EAGAIN,
            22 => SetUidError::EINVAL,
            _ => SetUidError::Unknown(item)
        }
    }
}

/// Pretty formatted error messages for SetUidError, instead of just the errno or the C error value
impl fmt::Display for SetUidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetUidError::EPERM => write!(f, "[errno: 1 EPERM] Operation not permitted."),
            SetUidError::EAGAIN => write!(f, "[errno: 11 EAGAIN] Try again."),
            SetUidError::EINVAL => write!(f, "[errno: 22 EINVAL] Invalid argument."),
            SetUidError::Unknown(e) => write!(f, "[errno: {} Unknown] {}", e, errno::error_string(*e)),
        }
    }
}

/// Implement Error for SetUidError so we can use the `?` operator for error checking
impl Error for SetUidError {}

