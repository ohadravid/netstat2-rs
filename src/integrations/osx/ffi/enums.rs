use libc::*;

pub type XSO_KIND = uint32_t;
pub const XSO_SOCKET: XSO_KIND = 0x001;
pub const XSO_RCVBUF: XSO_KIND = 0x002;
pub const XSO_SNDBUF: XSO_KIND = 0x004;
pub const XSO_STATS: XSO_KIND = 0x008;
pub const XSO_INPCB: XSO_KIND = 0x010;
pub const XSO_TCPCB: XSO_KIND = 0x020;
pub const XSO_KCREG: XSO_KIND = 0x040;
pub const XSO_KCB: XSO_KIND = 0x080;
pub const XSO_EVT: XSO_KIND = 0x100;
