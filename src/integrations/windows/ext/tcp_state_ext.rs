use integrations::windows::ffi;
use types::TcpState;

impl From<ffi::DWORD> for TcpState {
    fn from(tcp_state: ffi::DWORD) -> TcpState {
        match tcp_state {
            ffi::MIB_TCP_STATE_CLOSED => TcpState::Closed,
            ffi::MIB_TCP_STATE_LISTEN => TcpState::Listen,
            ffi::MIB_TCP_STATE_SYN_SENT => TcpState::SynSent,
            ffi::MIB_TCP_STATE_SYN_RCVD => TcpState::SynReceived,
            ffi::MIB_TCP_STATE_ESTAB => TcpState::Established,
            ffi::MIB_TCP_STATE_FIN_WAIT1 => TcpState::FinWait1,
            ffi::MIB_TCP_STATE_FIN_WAIT2 => TcpState::FinWait2,
            ffi::MIB_TCP_STATE_CLOSE_WAIT => TcpState::CloseWait,
            ffi::MIB_TCP_STATE_CLOSING => TcpState::Closing,
            ffi::MIB_TCP_STATE_LAST_ACK => TcpState::LastAck,
            ffi::MIB_TCP_STATE_TIME_WAIT => TcpState::TimeWait,
            ffi::MIB_TCP_STATE_DELETE_TCB => TcpState::DeleteTcb,
            _ => panic!("Unknown TcpState!"),
        }
    }
}
