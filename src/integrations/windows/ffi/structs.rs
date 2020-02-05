use integrations::windows::ffi::*;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_UDPROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDP6TABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_UDP6ROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDPROW_OWNER_PID {
    pub local_addr: DWORD,
    pub local_port: DWORD,
    pub owning_pid: DWORD,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDP6ROW_OWNER_PID {
    pub local_addr: [UCHAR; 16],
    pub local_scope_id: DWORD,
    pub local_port: DWORD,
    pub owning_pid: DWORD,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_TCPROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCP6TABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_TCP6ROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCPROW_OWNER_PID {
    pub state: DWORD,
    pub local_addr: DWORD,
    pub local_port: DWORD,
    pub remote_addr: DWORD,
    pub remote_port: DWORD,
    pub owning_pid: DWORD,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCP6ROW_OWNER_PID {
    pub local_addr: [UCHAR; 16],
    pub local_scope_id: DWORD,
    pub local_port: DWORD,
    pub remote_addr: [UCHAR; 16],
    pub remote_scope_id: DWORD,
    pub remote_port: DWORD,
    pub state: DWORD,
    pub owning_pid: DWORD,
}
