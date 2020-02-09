use std::fmt;

/// State of TCP connection.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
    DeleteTcb,
}

impl fmt::Display for TcpState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TcpState::Closed => "CLOSED",
                TcpState::Listen => "LISTEN",
                TcpState::SynSent => "SYN_SENT",
                TcpState::SynReceived => "SYN_RCVD",
                TcpState::Established => "ESTABLISHED",
                TcpState::FinWait1 => "FIN_WAIT_1",
                TcpState::FinWait2 => "FIN_WAIT_2",
                TcpState::CloseWait => "CLOSE_WAIT",
                TcpState::Closing => "CLOSING",
                TcpState::LastAck => "LAST_ACK",
                TcpState::TimeWait => "TIME_WAIT",
                TcpState::DeleteTcb => "DELETE_TCB",
            }
        )
    }
}
