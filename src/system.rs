use std::convert::From;

use crate::errno;
use crate::externs;
use crate::types::uid_t;
use crate::errors::setuid::SetUidError;

/// SAFETY: On success, zero is returned.
/// On error, -1 is returned, and errno is set to indicate the error.
pub fn setuid(newuid: uid_t) -> Result<i32, SetUidError> {
    let setuid_e = unsafe { externs::setuid(newuid) };
    // setuid returns 0 if it works, -1 if it fails.
    // We assume any result other than 0 is a failure for robustness.
    match setuid_e {
        // No Error
        0 => { return Ok(0); },
        // Error Detected
        _ => {
            // Get the errno set by setuid.
            let error_no = errno::errno();
            // Convert the errno to our SetUidError type
            let error: SetUidError = SetUidError::from(error_no);
            // Return the SetUidError, we have failed.
            return Err(error);
        }
    };
}

/// SAFETY: The `getuid()` function is documented as always being successful
pub fn getuid() -> uid_t {
    unsafe { externs::getuid() }
}

