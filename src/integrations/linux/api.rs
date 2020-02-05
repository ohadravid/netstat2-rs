use integrations::linux::netlink_iterator::*;
use integrations::linux::procfs::*;
use libc::*;
use types::*;

/// Iterate through sockets information.
pub fn iterate_sockets_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<impl Iterator<Item = Result<SocketInfo, Error>>, Error> {
    let ipv4 = af_flags.contains(AddressFamilyFlags::IPV4);
    let ipv6 = af_flags.contains(AddressFamilyFlags::IPV6);
    let tcp = proto_flags.contains(ProtocolFlags::TCP);
    let udp = proto_flags.contains(ProtocolFlags::UDP);
    let mut iterators = Vec::with_capacity(4);
    unsafe {
        if ipv4 {
            if tcp {
                iterators.push(NetlinkIterator::new(AF_INET as u8, IPPROTO_TCP as u8)?);
            }
            if udp {
                iterators.push(NetlinkIterator::new(AF_INET as u8, IPPROTO_UDP as u8)?);
            }
        }
        if ipv6 {
            if tcp {
                iterators.push(NetlinkIterator::new(AF_INET6 as u8, IPPROTO_TCP as u8)?);
            }
            if udp {
                iterators.push(NetlinkIterator::new(AF_INET6 as u8, IPPROTO_UDP as u8)?);
            }
        }
    }
    Ok(attach_pids(iterators.into_iter().flatten()))
}

fn attach_pids(
    sockets_info: impl Iterator<Item = Result<SocketInfo, Error>>,
) -> impl Iterator<Item = Result<SocketInfo, Error>> {
    let mut pids_by_inode = build_hash_of_pids_by_inode();
    sockets_info.map(move |r| {
        r.map(|socket_info| SocketInfo {
            associated_pids: pids_by_inode
                .remove(&socket_info.inode)
                .unwrap_or_default()
                .iter()
                .map(|x| *x)
                .collect(),
            ..socket_info
        })
    })
}
