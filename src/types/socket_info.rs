use crate::types::tcp_state::TcpState;
use std::net::IpAddr;

/// General socket information.
#[derive(Clone, Debug, PartialEq)]
pub struct SocketInfo {
    /// Protocol-specific socket information.
    pub protocol_socket_info: ProtocolSocketInfo,
    /// Identifiers of processes associated with this socket.
    pub associated_pids: Vec<u32>,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    /// Inode number of this socket.
    pub inode: u32,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    /// Owner UID of this socket.
    pub uid: u32,
}

/// Protocol-specific socket information.
#[derive(Clone, Debug, PartialEq)]
pub enum ProtocolSocketInfo {
    /// TCP-specific socket information.
    Tcp(TcpSocketInfo),
    /// UDP-specific socket information.
    Udp(UdpSocketInfo),
}

/// TCP-specific socket information.
#[derive(Clone, Debug, PartialEq)]
pub struct TcpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub state: TcpState,
}

/// UDP-specific socket information.
#[derive(Clone, Debug, PartialEq)]
pub struct UdpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
}
