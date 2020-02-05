use integrations::osx::ffi::consts::*;
use integrations::osx::ffi::types::*;
use libc::*;

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct in_addr {
//     pub s_addr: in_addr_t,
// }

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct in6_addr {
//     pub __u6_addr: __u6_addr_t, /* 128-bit IP6 address */
// }

#[repr(C)]
#[derive(Copy, Clone)]
pub union __u6_addr_t {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inp_depend4_t {
    pub inp4_ip_tos: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inp_depend6_t {
    pub inp6_hlim: uint8_t,
    pub inp6_cksum: c_int,
    pub inp6_ifindex: c_ushort,
    pub inp6_hops: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr_4in6 {
    pub ia46_pad32: [uint32_t; 3],
    pub ia46_addr4: in_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union inp_dependfaddr_t {
    /* foreign host table entry */
    pub inp46_foreign: in_addr_4in6,
    pub inp6_foreign: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union inp_dependladdr_t {
    /* local host table entry */
    pub inp46_local: in_addr_4in6,
    pub inp6_local: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xinpcb_n {
    pub xi_len: uint32_t,  /* length of this structure */
    pub xi_kind: uint32_t, /* XSO_INPCB */
    pub xi_inpp: uint64_t,
    pub inp_fport: c_ushort,   /* foreign port */
    pub inp_lport: c_ushort,   /* local port */
    pub inp_ppcb: uint64_t,    /* pointer to per-protocol pcb */
    pub inp_gencnt: inp_gen_t, /* generation count of this instance */
    pub inp_flags: c_int,      /* generic IP/datagram flags */
    pub inp_flow: uint32_t,
    pub inp_vflag: c_uchar,
    pub inp_ip_ttl: c_uchar, /* time to live */
    pub inp_ip_p: c_uchar,   /* protocol */
    pub inp_dependfaddr: inp_dependfaddr_t,
    pub inp_dependladdr: inp_dependladdr_t,
    pub inp_depend4: inp_depend4_t,
    pub inp_depend6: inp_depend6_t,
    pub inp_flowhash: uint32_t,
    pub inp_flags2: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xinpgen {
    pub xig_len: uint32_t,   /* length of this structure */
    pub xig_count: c_uint,   /* number of PCBs at this time */
    pub xig_gen: inp_gen_t,  /* generation count at this time */
    pub xig_sogen: so_gen_t, /* current socket generation count */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgen_n {
    pub xgn_len: uint32_t,  /* length of this structure */
    pub xgn_kind: uint32_t, /* number of PCBs at this time */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsocket_n {
    pub xso_len: uint32_t,  /* length of this structure */
    pub xso_kind: uint32_t, /* XSO_SOCKET */
    pub xso_so: uint64_t,   /* makes a convenient handle */
    pub so_type: c_short,
    pub so_options: uint32_t,
    pub so_linger: c_short,
    pub so_state: c_short,
    pub so_pcb: uint64_t, /* another convenient handle */
    pub xso_protocol: c_int,
    pub xso_family: c_int,
    pub so_qlen: c_short,
    pub so_incqlen: c_short,
    pub so_qlimit: c_short,
    pub so_timeo: c_short,
    pub so_error: c_ushort,
    pub so_pgid: pid_t,
    pub so_oobmark: uint32_t,
    pub so_uid: uid_t, /* XXX */
    pub so_last_pid: pid_t,
    pub so_e_pid: pid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsockbuf_n {
    pub xsb_len: uint32_t,  /* length of this structure */
    pub xsb_kind: uint32_t, /* XSO_RCVBUF or XSO_SNDBUF */
    pub sb_cc: uint32_t,
    pub sb_hiwat: uint32_t,
    pub sb_mbcnt: uint32_t,
    pub sb_mbmax: uint32_t,
    pub sb_lowat: int32_t,
    pub sb_flags: c_short,
    pub sb_timeo: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsockstat_n {
    pub xst_len: uint32_t,  /* length of this structure */
    pub xst_kind: uint32_t, /* XSO_STATS */
    pub xst_tc_stats: [data_stats; SO_TC_STATS_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_stats {
    pub rxpackets: uint64_t,
    pub rxbytes: uint64_t,
    pub txpackets: uint64_t,
    pub txbytes: uint64_t,
}

pub struct xtcpcb_n {
    pub xt_len: uint32_t,
    pub xt_kind: uint32_t, /* XSO_TCPCB */
    pub t_segq: uint64_t,
    pub t_dupacks: c_int,                   /* consecutive dup acks recd */
    pub t_timer: [c_int; TCPT_NTIMERS_EXT], /* tcp timers */
    pub t_state: c_int,                     /* state of this connection */
    pub t_flags: c_uint,
    pub t_force: c_int,         /* 1 if forcing out a byte */
    pub snd_una: tcp_seq,       /* send unacknowledged */
    pub snd_max: tcp_seq,       /* highest sequence number sent, used to recognize retransmits */
    pub snd_nxt: tcp_seq,       /* send next */
    pub snd_up: tcp_seq,        /* send urgent pointer */
    pub snd_wl1: tcp_seq,       /* window update seg seq number */
    pub snd_wl2: tcp_seq,       /* window update seg ack number */
    pub iss: tcp_seq,           /* initial send sequence number */
    pub irs: tcp_seq,           /* initial receive sequence number */
    pub rcv_nxt: tcp_seq,       /* receive next */
    pub rcv_adv: tcp_seq,       /* advertised window */
    pub rcv_wnd: uint32_t,      /* receive window */
    pub rcv_up: tcp_seq,        /* receive urgent pointer */
    pub snd_wnd: uint32_t,      /* send window */
    pub snd_cwnd: uint32_t,     /* congestion-controlled window */
    pub snd_ssthresh: uint32_t, /* snd_cwnd size threshold for for slow start exponential to linear switch */
    pub t_maxopd: c_uint,       /* mss plus options */
    pub t_rcvtime: uint32_t,    /* time at which a packet was received */
    pub t_starttime: uint32_t,  /* time connection was established */
    pub t_rtttime: c_int,       /* round trip time */
    pub t_rtseq: tcp_seq,       /* sequence number being timed */
    pub t_rxtcur: c_int,        /* current retransmit value (ticks) */
    pub t_maxseg: c_uint,       /* maximum segment size */
    pub t_srtt: c_int,          /* smoothed round-trip time */
    pub t_rttvar: c_int,        /* variance in round-trip time */
    pub t_rxtshift: c_int,      /* log(2) of rexmt exp. backoff */
    pub t_rttmin: c_uint,       /* minimum rtt allowed */
    pub t_rttupdated: uint32_t, /* number of times rtt sampled */
    pub max_sndwnd: uint32_t,   /* largest window peer has offered */
    pub t_softerror: c_int,     /* possible error not yet reported */
    /* out-of-band data */
    pub t_oobflags: char, /* have some */
    pub t_iobc: char,     /* input character */
    /* RFC 1323 variables */
    pub snd_scale: c_uchar,       /* window scaling for send window */
    pub rcv_scale: c_uchar,       /* window scaling for recv window */
    pub request_r_scale: c_uchar, /* pending window scaling */
    pub requested_s_scale: c_uchar,
    pub ts_recent: uint32_t,     /* timestamp echo data */
    pub ts_recent_age: uint32_t, /* when last updated */
    pub last_ack_sent: tcp_seq,
    /* RFC 1644 variables */
    pub cc_send: tcp_cc,      /* send connection count */
    pub cc_recv: tcp_cc,      /* receive connection count */
    pub snd_recover: tcp_seq, /* for use in fast recovery */
    /* experimental */
    pub snd_cwnd_prev: uint32_t,     /* cwnd prior to retransmit */
    pub snd_ssthresh_prev: uint32_t, /* ssthresh prior to retransmit */
}
