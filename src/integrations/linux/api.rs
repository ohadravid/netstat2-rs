use crate::filter::{create_filter, PortCond, PortCondType};
use crate::integrations::linux::netlink_iterator::*;
use crate::integrations::linux::procfs::*;
use crate::types::error::Error;
use crate::types::*;
use libc::*;

/// Iterate through sockets information.
pub fn iterate_sockets_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<impl Iterator<Item = Result<SocketInfo, Error>>, Error> {
    Ok(attach_pids(iterate_sockets_info_without_pids(
        af_flags,
        proto_flags,
        &[],
    )?))
}

/// Iterate through sockets information without attaching PID.
pub fn iterate_sockets_info_without_pids(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
    filters: &[PortCond],
) -> Result<impl Iterator<Item = Result<SocketInfo, Error>>, Error> {
    let ipv4 = af_flags.contains(AddressFamilyFlags::IPV4);
    let ipv6 = af_flags.contains(AddressFamilyFlags::IPV6);
    let tcp = proto_flags.contains(ProtocolFlags::TCP);
    let udp = proto_flags.contains(ProtocolFlags::UDP);
    let mut iterators = Vec::with_capacity(4);
    let mut filters = if filters.is_empty() {
        vec![]
    } else {
        create_filter(filters)
    };

    unsafe {
        if ipv4 {
            if tcp {
                iterators.push(NetlinkIterator::new(
                    AF_INET as u8,
                    IPPROTO_TCP as u8,
                    &mut filters,
                )?);
            }
            if udp {
                iterators.push(NetlinkIterator::new(
                    AF_INET as u8,
                    IPPROTO_UDP as u8,
                    &mut filters,
                )?);
            }
        }
        if ipv6 {
            if tcp {
                iterators.push(NetlinkIterator::new(
                    AF_INET6 as u8,
                    IPPROTO_TCP as u8,
                    &mut filters,
                )?);
            }
            if udp {
                iterators.push(NetlinkIterator::new(
                    AF_INET6 as u8,
                    IPPROTO_UDP as u8,
                    &mut filters,
                )?);
            }
        }
    }
    Ok(iterators.into_iter().flatten())
}

pub fn iterate_sockets_info_filtered(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
    filters: &[PortCond],
    associate_pid: bool,
) -> Result<Box<dyn Iterator<Item = Result<SocketInfo, Error>>>, Error> {
    let iter = iterate_sockets_info_without_pids(af_flags, proto_flags, filters)?;
    Ok(if associate_pid {
        Box::new(attach_pids(iter))
    } else {
        Box::new(iter)
    })
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
