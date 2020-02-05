use libc::*;

/*
 * From "linux/sock_diag.h"
 */

pub const SOCK_DIAG_BY_FAMILY: __u16 = 20;

/*
 * From "linux/inet_diag.h"
 */

pub type INET_DIAG_TYPE = c_int;
pub const INET_DIAG_INFO: INET_DIAG_TYPE = 2;
