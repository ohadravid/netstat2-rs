use crate::integrations::windows::ffi::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDPTABLE {
    pub rows_count: DWORD,
    pub rows: [MIB_UDPROW; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDPROW {
    pub local_addr: DWORD,
    pub local_port: DWORD,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCPTABLE {
    pub rows_count: DWORD,
    pub rows: [MIB_TCPROW; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCPROW {
    pub state: DWORD,
    pub local_addr: DWORD,
    pub local_port: DWORD,
    pub remote_addr: DWORD,
    pub remote_port: DWORD,
}

