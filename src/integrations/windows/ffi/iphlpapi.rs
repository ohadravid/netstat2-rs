use integrations::windows::ffi::*;

#[link(name = "iphlpapi")]
extern "system" {
    pub fn GetExtendedTcpTable(
        pTcpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: TCP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
    pub fn GetExtendedUdpTable(
        pUdpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: UDP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
}
