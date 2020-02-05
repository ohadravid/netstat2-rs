macro_rules! NLMSG_OK {
    ($nlh:expr, $len:expr) => {{
        let nlmsghdr_size = std::mem::size_of::<nlmsghdr>();
        $len >= nlmsghdr_size as isize
            && (&*$nlh).nlmsg_len >= nlmsghdr_size as __u32
            && (&*$nlh).nlmsg_len <= $len as __u32
    }};
}

macro_rules! NLMSG_ALIGN {
    ($len:expr) => {
        ($len + 3) & !3
    };
}

macro_rules! NLMSG_LENGTH {
    ($len:expr) => {
        $len + NLMSG_ALIGN!(std::mem::size_of::<nlmsghdr>())
    };
}

macro_rules! NLMSG_DATA {
    ($nlh:expr) => {
        ($nlh as *const u8).offset(NLMSG_LENGTH!(0) as isize)
    };
}

macro_rules! NLMSG_NEXT {
    ($nlh:expr, $len:expr) => {{
        let nlh_len = (&*$nlh).nlmsg_len;
        $len -= NLMSG_ALIGN!(nlh_len) as isize;
        ($nlh as *const u8).offset(NLMSG_ALIGN!(nlh_len) as isize) as *const nlmsghdr
    }};
}

macro_rules! RTA_ALIGN {
    ($len:expr) => {
        ($len + 3) & !3
    };
}

macro_rules! RTA_OK {
    ($rta:expr, $len:expr) => {{
        let rtattr_size = std::mem::size_of::<rtattr>();
        $len >= rtattr_size as isize
            && (&*$rta).rta_len >= rtattr_size as u16
            && (&*$rta).rta_len <= $len as u16
    }};
}

macro_rules! RTA_NEXT {
    ($rta:expr, $len:expr) => {{
        let rta_len = (&*$rta).rta_len as isize;
        $len -= RTA_ALIGN!(rta_len);
        ($rta as *const u8).offset(RTA_ALIGN!(rta_len)) as *const rtattr
    }};
}

macro_rules! RTA_LENGTH {
    ($len:expr) => {
        $len + RTA_ALIGN!(std::mem::size_of::<rtattr>())
    };
}

macro_rules! RTA_DATA {
    ($rta:expr) => {
        ($rta as *const u8).offset(RTA_LENGTH!(0) as isize)
    };
}
