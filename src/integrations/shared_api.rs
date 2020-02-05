use integrations::*;
use types::*;

/// Retrieve sockets information as a vector.
/// Short-circuits on any error along the way.
pub fn get_sockets_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<Vec<SocketInfo>, Error> {
    iterate_sockets_info(af_flags, proto_flags)?.collect()
}
