use integrations::windows::socket_table::SocketTable;
use types::*;

pub struct SocketTableIterator {
    table: Vec<u8>,
    rows_count: usize,
    current_row_index: usize,
    info_getter: fn(&Vec<u8>, usize) -> SocketInfo,
}

impl SocketTableIterator {
    pub fn new<Table: SocketTable>() -> Result<Self, Error> {
        let table = Table::get_table()?;
        Ok(SocketTableIterator {
            rows_count: Table::get_rows_count(&table),
            info_getter: Table::get_socket_info,
            current_row_index: 0,
            table,
        })
    }
}

impl Iterator for SocketTableIterator {
    type Item = Result<SocketInfo, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row_index == self.rows_count {
            None
        } else {
            let socket_info = (self.info_getter)(&self.table, self.current_row_index);
            self.current_row_index += 1;
            Some(Ok(socket_info))
        }
    }
}
