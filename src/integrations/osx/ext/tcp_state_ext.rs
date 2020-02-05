use types::TcpState;

impl<'a> From<&'a str> for TcpState {
    fn from(tcp_state: &'a str) -> TcpState {
        match tcp_state {
            "CLOSED" => TcpState::Closed,
            "LISTEN" => TcpState::Listen,
            "SYN_SENT" => TcpState::SynSent,
            "SYN_RCVD" => TcpState::SynReceived,
            "ESTABLISHED" => TcpState::Established,
            "FIN_WAIT_1" => TcpState::FinWait1,
            "FIN_WAIT_2" => TcpState::FinWait2,
            "CLOSE_WAIT" => TcpState::CloseWait,
            "CLOSING" => TcpState::Closing,
            "LAST_ACK" => TcpState::LastAck,
            "TIME_WAIT" => TcpState::TimeWait,
            _ => panic!("Unknown TcpState!"),
        }
    }
}
