use std;

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to call ffi")]
    OsError(io::Error),

    #[error("Unsupported SocketFamily {0}")]
    UnsupportedSocketFamily(u32),

    #[error("Failed to list processes")]
    FailedToListProcesses(io::Error),

    #[error("Not a valid socket")]
    NotAValidSocket,

    #[error("{0} is not a valid proc_fdtype")]
    NotAValidFDType(u32),

    #[error("Failed to query file descriptors")]
    FailedToQueryFileDescriptors(io::Error),

    #[error("Unsupported file descriptor")]
    UnsupportedFileDescriptor,

    #[error("Failed to allocate buffer")]
    FailedToAllocateBuffer,

    #[error("Failed to get UDP table")]
    FailedToGetTcpTable(i32),

    #[error("Failed to get UDP table")]
    FailedToGetUdpTable(i32),

    #[error("NetLink Error")]
    NetLinkError,

    #[error("Found unknown protocol {0}")]
    UnknownProtocol(u8),
}
