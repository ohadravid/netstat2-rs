use integrations::linux::ffi::types::*;
use libc::*;

/*
 * From "linux/rtnetlink.h"
 */

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct rtattr {
    pub rta_len: u16,
    pub rta_type: u16,
}

/*
 * From "linux/inet_diag.h"
 */

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct inet_diag_sockid {
    pub sport: __be16,
    pub dport: __be16,
    pub src: [__be32; 4],
    pub dst: [__be32; 4],
    pub if_: __u32,
    pub cookie: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct inet_diag_msg {
    pub family: __u8,
    pub state: __u8,
    pub timer: __u8,
    pub retrans: __u8,
    pub id: inet_diag_sockid,
    pub expires: __u32,
    pub rqueue: __u32,
    pub wqueue: __u32,
    pub uid: __u32,
    pub inode: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct inet_diag_req {
    pub family: __u8, /* Family of addresses. */
    pub src_len: __u8,
    pub dst_len: __u8,
    pub ext: __u8, /* Query extended information */
    pub id: inet_diag_sockid,
    pub states: __u32, /* States to dump */
    pub dbs: __u32,    /* Tables to dump (NI) */
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct inet_diag_req_v2 {
    pub family: __u8,
    pub protocol: __u8,
    pub ext: __u8,
    pub pad: __u8,
    pub states: __u32,
    pub id: inet_diag_sockid,
}

/*
 * From "linux/tcp.h"
 */

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct tcp_info {
    pub state: __u8,
    pub ca_state: __u8,
    pub retransmits: __u8,
    pub probes: __u8,
    pub backoff: __u8,
    pub options: __u8,
    pub snd_wscale: __u8,                // : 4
    pub rcv_wscale: __u8,                // : 4
    pub delivery_rate_app_limited: __u8, // : 1
    pub rto: __u32,
    pub ato: __u32,
    pub snd_mss: __u32,
    pub rcv_mss: __u32,
    pub unacked: __u32,
    pub sacked: __u32,
    pub lost: __u32,
    pub retrans: __u32,
    pub fackets: __u32,
    /* Times. */
    pub last_data_sent: __u32,
    pub last_ack_sent: __u32, /* Not remembered, sorry. */
    pub last_data_recv: __u32,
    pub last_ack_recv: __u32,
    /* Metrics. */
    pub pmtu: __u32,
    pub rcv_ssthresh: __u32,
    pub rtt: __u32,
    pub rttvar: __u32,
    pub snd_ssthresh: __u32,
    pub snd_cwnd: __u32,
    pub advmss: __u32,
    pub reordering: __u32,
    pub rcv_rtt: __u32,
    pub rcv_space: __u32,
    pub total_retrans: __u32,
    pub pacing_rate: __u64,
    pub max_pacing_rate: __u64,
    pub bytes_acked: __u64,    /* RFC4898 tcpEStatsAppHCThruOctetsAcked */
    pub bytes_received: __u64, /* RFC4898 tcpEStatsAppHCThruOctetsReceived */
    pub segs_out: __u32,       /* RFC4898 tcpEStatsPerfSegsOut */
    pub segs_in: __u32,        /* RFC4898 tcpEStatsPerfSegsIn */
    pub notsent_bytes: __u32,
    pub min_rtt: __u32,
    pub data_segs_in: __u32,  /* RFC4898 tcpEStatsDataSegsIn */
    pub data_segs_out: __u32, /* RFC4898 tcpEStatsDataSegsOut */
    pub delivery_rate: __u64,
    pub busy_time: __u64,      /* Time (usec) busy sending data */
    pub rwnd_limited: __u64,   /* Time (usec) limited by receive window */
    pub sndbuf_limited: __u64, /* Time (usec) limited by send buffer */
}
