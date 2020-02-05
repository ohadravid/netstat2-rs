use integrations::osx::ffi::enums::*;
use libc::*;

pub const SO_TC_STATS_MAX: usize = 4;
pub const TCPT_NTIMERS_EXT: usize = 4;

pub const ALL_XGN_KIND_INP: uint32_t =
    (XSO_SOCKET | XSO_RCVBUF | XSO_SNDBUF | XSO_STATS | XSO_INPCB);
pub const ALL_XGN_KIND_TCP: uint32_t = (ALL_XGN_KIND_INP | XSO_TCPCB);

pub const INP_IPV4: c_uchar = 0x1;
pub const INP_IPV6: c_uchar = 0x2;
