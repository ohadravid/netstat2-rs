use crate::integrations::windows::ffi::*;
use crate::integrations::windows::socket_table_extended::SocketTable;
use crate::types::error::*;
use crate::types::*;
use std::net::{IpAddr, Ipv4Addr};

impl SocketTable for MIB_TCPTABLE {
    fn get_table() -> Result<Vec<u8>, Error> {
        get_tcp_table(AF_INET)
    }
    fn get_rows_count(table: &[u8]) -> usize {
        let table = unsafe { &*(table.as_ptr() as *const MIB_TCPTABLE) };
        table.rows_count as usize
    }
    fn get_socket_info(table: &[u8], index: usize) -> SocketInfo {
        let table = unsafe { &*(table.as_ptr() as *const MIB_TCPTABLE) };
        let rows_ptr = &table.rows[0] as *const MIB_TCPROW;
        let row = unsafe { &*rows_ptr.add(index) };
        SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Tcp(TcpSocketInfo {
                local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                local_port: u16::from_be(row.local_port as u16),
                remote_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.remote_addr))),
                remote_port: u16::from_be(row.remote_port as u16),
                state: TcpState::from(row.state),
            }),
            associated_pids: vec![],
        }
    }
}

impl SocketTable for MIB_UDPTABLE {
    fn get_table() -> Result<Vec<u8>, Error> {
        get_udp_table(AF_INET)
    }
    fn get_rows_count(table: &[u8]) -> usize {
        let table = unsafe { &*(table.as_ptr() as *const MIB_UDPTABLE) };
        table.rows_count as usize
    }
    fn get_socket_info(table: &[u8], index: usize) -> SocketInfo {
        let table = unsafe { &*(table.as_ptr() as *const MIB_UDPTABLE) };
        let rows_ptr = &table.rows[0] as *const MIB_UDPROW;
        let row = unsafe { &*rows_ptr.add(index) };
        SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Udp(UdpSocketInfo {
                local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                local_port: u16::from_be(row.local_port as u16),
            }),
            associated_pids: vec![],
        }
    }
}

fn get_tcp_table(_address_family: ULONG) -> Result<Vec<u8>, Error> {
    let mut table_size: DWORD = 0;
    let mut err_code = unsafe { GetTcpTable(std::ptr::null_mut(), &mut table_size, FALSE) };
    let mut table = Vec::<u8>::new();
    let mut iterations = 0;
    while err_code == ERROR_INSUFFICIENT_BUFFER {
        table = Vec::<u8>::with_capacity(table_size as usize);
        err_code = unsafe { GetTcpTable(table.as_mut_ptr() as PVOID, &mut table_size, FALSE) };
        iterations += 1;
        if iterations > 100 {
            return Result::Err(Error::FailedToAllocateBuffer);
        }
    }
    if err_code == NO_ERROR {
        Ok(table)
    } else {
        Err(Error::FailedToGetTcpTable(err_code as i32))
    }
}

fn get_udp_table(_address_family: ULONG) -> Result<Vec<u8>, Error> {
    let mut table_size: DWORD = 0;
    let mut err_code = unsafe { GetUdpTable(std::ptr::null_mut(), &mut table_size, FALSE) };
    let mut table = Vec::<u8>::new();
    let mut iterations = 0;
    while err_code == ERROR_INSUFFICIENT_BUFFER {
        table = Vec::<u8>::with_capacity(table_size as usize);
        err_code = unsafe { GetUdpTable(table.as_mut_ptr() as PVOID, &mut table_size, FALSE) };
        iterations += 1;
        if iterations > 100 {
            return Result::Err(Error::FailedToAllocateBuffer);
        }
    }
    if err_code == NO_ERROR {
        Ok(table)
    } else {
        Err(Error::FailedToGetUdpTable(err_code as i32))
    }
}
