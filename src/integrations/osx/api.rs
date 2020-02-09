use crate::integrations::osx::netstat::*;
use crate::types::error::Error;
use crate::types::*;

/// Iterate through sockets information.
pub fn iterate_sockets_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<impl Iterator<Item = Result<SocketInfo, Error>>, Error> {
    iterate_netstat_info(af_flags, proto_flags)
}
