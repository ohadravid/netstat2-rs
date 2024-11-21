bitflags! {
    /// Set of protocols.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ProtocolFlags: u8 {
        const TCP = 0b00000001;
        const UDP = 0b00000010;
    }
}
