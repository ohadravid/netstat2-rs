mod api;
mod ext;
mod ffi;
mod socket_table;
mod socket_table_extended;
mod socket_table_iterator;

pub use self::api::*;

#[cfg(test)]
mod tests {
    use crate::integrations::windows::ffi::*;
    use crate::integrations::windows::socket_table_iterator::SocketTableIterator;

    #[test]
    fn test_iterate_over_all_supported_tables() {
        let table: Vec<_> = SocketTableIterator::new::<MIB_TCPTABLE_OWNER_PID>()
            .unwrap()
            .collect();
        assert!(!table.is_empty());

        let table: Vec<_> = SocketTableIterator::new::<MIB_UDPTABLE_OWNER_PID>()
            .unwrap()
            .collect();
        assert!(!table.is_empty());

        let table: Vec<_> = SocketTableIterator::new::<MIB_TCP6TABLE_OWNER_PID>()
            .unwrap()
            .collect();
        assert!(!table.is_empty());

        let table: Vec<_> = SocketTableIterator::new::<MIB_UDP6TABLE_OWNER_PID>()
            .unwrap()
            .collect();
        assert!(!table.is_empty());

        // Old API versions.
        let table: Vec<_> = SocketTableIterator::new::<MIB_TCPTABLE>()
            .unwrap()
            .collect();
        assert!(!table.is_empty());

        let table: Vec<_> = SocketTableIterator::new::<MIB_UDPTABLE>()
            .unwrap()
            .collect();
        assert!(!table.is_empty());
    }
}
