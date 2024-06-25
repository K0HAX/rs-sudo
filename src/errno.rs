use std::ffi::{c_int, c_char, CStr};

#[allow(non_camel_case_types)]
pub type size_t = u64;

pub const TMPBUF_SZ: usize = 128;

extern "C" {
    #[cfg(not(any(target_os = "dragonfly", target_os = "vxworks")))]
    #[cfg_attr(
        any(
            target_os = "linux",
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "l4re"
        ),
        link_name = "__errno_location"
    )]
    #[cfg_attr(
        any(
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "android",
            target_os = "redox",
            target_env = "newlib"
        ),
        link_name = "__errno"
    )]
    #[cfg_attr(any(target_os = "solaris", target_os = "illumos"), link_name = "___errno")]
    #[cfg_attr(target_os = "nto", link_name = "__get_errno_ptr")]
    #[cfg_attr(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "freebsd",
            target_os = "watchos"
        ),
        link_name = "__error"
    )]
    #[cfg_attr(target_os = "haiku", link_name = "_errnop")]
    fn errno_location() -> *mut c_int;
}

/// Returns the platform-specific value of errno
#[cfg(not(any(target_os = "dragonfly", target_os = "vxworks")))]
pub fn errno() -> i32 {
    unsafe { (*errno_location()) as i32 }
}

#[cfg(target_os = "vxworks")]
pub fn errno() -> i32 {
    unsafe { libc::errnoGet() }
}

#[cfg(target_os = "dragonfly")]
pub fn errno() -> i32 {
    extern "C" {
        #[thread_local]
        static errno: c_int;
    }

    unsafe { errno as i32 }
}

/// Sets the platform-specific value of errno
#[cfg(all(not(target_os = "dragonfly"), not(target_os = "vxworks")))] // needed for readdir and syscall!
#[allow(dead_code)] // but not all target cfgs actually end up using it
pub fn set_errno(e: i32) {
    unsafe { *errno_location() = e as c_int }
}

#[cfg(target_os = "dragonfly")]
#[allow(dead_code)]
pub fn set_errno(e: i32) {
    extern "C" {
        #[thread_local]
        static mut errno: c_int;
    }

    unsafe {
        errno = e;
    }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(errno: i32) -> String {
    extern "C" {
        #[cfg_attr(
            all(any(target_os = "linux", target_env = "newlib"), not(target_env = "ohos")),
            link_name = "__xpg_strerror_r"
        )]
        fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    }

    let mut buf = [0 as c_char; TMPBUF_SZ];

    let p = buf.as_mut_ptr();
    unsafe {
        if strerror_r(errno as c_int, p, buf.len().try_into().unwrap()) < 0 {
            panic!("strerror_r failure");
        }

        let p = p as *const _;
        // We can't always expect a UTF-8 environment. When we don't get that luxury,
        // it's better to give a low-quality error message than none at all.
        String::from_utf8_lossy(CStr::from_ptr(p).to_bytes()).into()
    }
}

