#[allow(unused_imports)]
use crate::integrations::linux::ffi::types::*;
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

pub type INET_DIAG_REQ = u16;
pub const INET_DIAG_REQ_BYTECODE: INET_DIAG_REQ = 1;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum FilterOpRaw {
    NOP = 0,
    JMP = 1,
    S_GE = 2,
    S_LE = 3,
    D_GE = 4,
    D_LE = 5,
    AUTO = 6,
    S_COND = 7,
    D_COND = 8,
    DEV_COND = 9, /* u32 ifindex */
    MARK_COND = 10,
    S_EQ = 11,
    D_EQ = 12,
    CGROUP_COND = 13, /* u64 cgroup v2 ID */
}
