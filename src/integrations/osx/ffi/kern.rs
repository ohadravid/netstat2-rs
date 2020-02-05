use libc::*;

extern "C" {
    pub fn sysctlbyname(
        a: *const c_char,
        b: *mut c_void,
        c: *mut size_t,
        d: *mut c_void,
        e: size_t,
    ) -> c_int;
}
