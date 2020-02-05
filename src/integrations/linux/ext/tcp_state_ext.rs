use types::TcpState;

impl From<u8> for TcpState {
    fn from(tcp_state: u8) -> TcpState {
        match tcp_state {
            1 => TcpState::Established,
            2 => TcpState::SynSent,
            3 => TcpState::SynReceived,
            4 => TcpState::FinWait1,
            5 => TcpState::FinWait2,
            6 => TcpState::TimeWait,
            7 => TcpState::Closed,
            8 => TcpState::CloseWait,
            9 => TcpState::LastAck,
            10 => TcpState::Listen,
            11 => TcpState::Closing,
            _ => panic!("Unknown TcpState!"),
        }
    }
}
