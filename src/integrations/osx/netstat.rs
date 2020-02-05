use std::io::{BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::{Command, Stdio};
use types::*;

pub fn iterate_netstat_info(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
) -> Result<impl Iterator<Item = Result<SocketInfo, Error>>, Error> {
    let child = Command::new("netstat")
        .arg("-anv")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| Error::InternalError("Failed to run netstat utility!"))?;
    Ok(BufReader::new(child.stdout.unwrap())
        .lines()
        .filter_map(|ln| ln.ok())
        .skip_while(|ln| {
            let lower = ln.to_lowercase();
            !lower.contains("proto")
                || !lower.contains("recv")
                || !lower.contains("send")
                || !lower.contains("state")
                || !lower.contains("pid")
        }).skip(1)
        .filter_map(move |ln| match parse_line(af_flags, proto_flags, &ln) {
            Err(Termination::Skip) => None,
            r => Some(r),
        }).take_while(|r| match r {
            Err(Termination::Break) => false,
            _ => true,
        }).map(|r| r.map_err(|e| e.unwrap())))
}

enum Termination {
    Skip,
    Break,
    Error(Error),
}

impl Termination {
    fn unwrap(self) -> Error {
        match self {
            Termination::Error(e) => e,
            _ => unreachable!(),
        }
    }
}

impl From<Error> for Termination {
    fn from(e: Error) -> Self {
        Termination::Error(e)
    }
}

fn parse_line(
    af_flags: AddressFamilyFlags,
    proto_flags: ProtocolFlags,
    line: &String,
) -> Result<SocketInfo, Termination> {
    let parts: Vec<&str> = line
        .trim()
        .split(|c: char| c.is_whitespace() || c.is_control())
        .filter(|&s| s.len() > 0)
        .collect();
    if parts.len() < 9 {
        return Err(Termination::Break);
    }
    let is_tcp = parts[0].starts_with("tcp");
    let is_udp = parts[0].starts_with("udp");
    let is_ipv4 = parts[0].ends_with("4");
    let is_ipv6 = !is_ipv4;
    let skip = is_tcp && !proto_flags.contains(ProtocolFlags::TCP)
        || is_udp && !proto_flags.contains(ProtocolFlags::UDP)
        || is_ipv4 && !af_flags.contains(AddressFamilyFlags::IPV4)
        || is_ipv6 && !af_flags.contains(AddressFamilyFlags::IPV6);
    if skip {
        return Err(Termination::Skip);
    }
    let (local_addr, local_port) = split_endpoint(parts[3]);
    let (remote_addr, remote_port) = split_endpoint(parts[4]);
    let pid = if is_tcp {
        parts[8]
    } else if is_udp {
        parts[7]
    } else {
        panic!("Unknown netstat output format!");
    };
    if is_tcp {
        Ok(SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Tcp(TcpSocketInfo {
                local_addr: parse_ip(local_addr, is_ipv4)?,
                local_port: parse_port(local_port)?,
                remote_addr: parse_ip(remote_addr, is_ipv4)?,
                remote_port: parse_port(remote_port)?,
                state: TcpState::from(parts[5]),
            }),
            associated_pids: vec![
                pid.parse::<u32>()
                    .map_err(|_| Error::InternalError("Failed parsing pid!"))?,
            ],
        })
    } else if is_udp {
        Ok(SocketInfo {
            protocol_socket_info: ProtocolSocketInfo::Udp(UdpSocketInfo {
                local_addr: parse_ip(local_addr, is_ipv4)?,
                local_port: parse_port(local_port)?,
            }),
            associated_pids: vec![
                pid.parse::<u32>()
                    .map_err(|_| Error::InternalError("Failed parsing pid!"))?,
            ],
        })
    } else {
        Err(Termination::Skip)
    }
}

fn parse_ip(ip_str: &str, is_ipv4: bool) -> Result<IpAddr, Error> {
    let ip_str = remove_zone_index(ip_str);
    if ip_str == "*" {
        Result::Ok(match is_ipv4 {
            true => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            false => IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        })
    } else {
        Result::Ok(match is_ipv4 {
            true => IpAddr::V4(
                ip_str
                    .parse::<Ipv4Addr>()
                    .map_err(|_| Error::InternalError("Failed parsing Ipv4Addr!"))?,
            ),
            false => IpAddr::V6(
                ip_str
                    .parse::<Ipv6Addr>()
                    .map_err(|_| Error::InternalError("Failed parsing Ipv6Addr!"))?,
            ),
        })
    }
}

fn parse_port(port_str: &str) -> Result<u16, Error> {
    match port_str {
        "*" => Result::Ok(0),
        _ => port_str
            .parse::<u16>()
            .map_err(|_| Error::InternalError("Failed parsing port!")),
    }
}

fn split_endpoint(endpoint: &str) -> (&str, &str) {
    for (i, c) in endpoint.chars().rev().enumerate() {
        if c == '.' {
            return (
                &endpoint[0..endpoint.len() - i - 1],
                &endpoint[endpoint.len() - i..],
            );
        }
    }
    (endpoint, &endpoint[0..0])
}

fn remove_zone_index(ip_str: &str) -> &str {
    ip_str.splitn(2, '%').nth(0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_endpoint_default() {
        let (ip, port) = split_endpoint("192.168.48.128.123");
        assert_eq!(ip, "192.168.48.128");
        assert_eq!(port, "123");
    }

    #[test]
    fn split_endpoint_asterisk() {
        let (ip, port) = split_endpoint("*");
        assert_eq!(ip, "*");
        assert_eq!(port, "");
    }
}
