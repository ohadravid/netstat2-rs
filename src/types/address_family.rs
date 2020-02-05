bitflags! {
    /// Set of address families.
    pub struct AddressFamilyFlags: u8 {
        const IPV4 = 0b00000001;
        const IPV6 = 0b00000010;
    }
}
