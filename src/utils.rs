use std;

#[allow(dead_code)]
pub fn get_raw_os_error() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(-1)
}
