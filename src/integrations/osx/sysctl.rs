use integrations::osx::ffi::*;
use libc::*;
use std;
use std::ffi::*;
use std::mem::*;
use types::*;
use utils::*;

// todo: remove after finish
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 30;
pub const IPPROTO_TCP: c_int = 6;
// endremove

lazy_static! {
    static ref tcp_ctl_name: CString = CString::new("net.inet.tcp.pcblist_n").unwrap();
}

pub fn test() {
    unsafe {
        collect_tcp_info(AF_INET, IPPROTO_TCP);
    }
}

unsafe fn collect_tcp_info(af: c_int, protocol: c_int) -> Result<(), Error> {
    let mut len = 0;
    let len_request_retcode = sysctlbyname(
        tcp_ctl_name.as_ptr(),
        std::ptr::null_mut(),
        &mut len,
        std::ptr::null_mut(),
        0,
    );
    if len_request_retcode < 0 {
        return Result::Err(Error {
            method_name: "sysctlbyname",
            error_details: get_os_error_details(),
        });
    }
    let mut buf = Vec::<u8>::with_capacity(len);
    let buf_ptr = buf.as_mut_ptr();
    let buf_request_retcode = sysctlbyname(
        tcp_ctl_name.as_ptr(),
        buf_ptr as *mut _,
        &mut len,
        std::ptr::null_mut(),
        0,
    );
    if buf_request_retcode < 0 {
        return Result::Err(Error {
            method_name: "sysctlbyname",
            error_details: get_os_error_details(),
        });
    }
    if len <= size_of::<xinpgen>() {
        return Result::Ok(());
    }
    let mut so = std::mem::uninitialized();
    let mut tp = std::mem::uninitialized();
    let mut inp = std::mem::uninitialized();
    let mut which = 0;
    let xig = &*(buf_ptr as *mut xinpgen);
    let mut next = buf_ptr.offset(ROUNDUP64!(xig.xig_len as isize));
    loop {
        if next >= buf_ptr.offset(len as isize) {
            break;
        }
        let xgn = next as *mut xgen_n;
        let xgn_ref = &*xgn;
        /*
         * Bail-out to avoid logic error in the loop below when
         * there is in fact no more control block to process
         */
        if xgn_ref.xgn_len as usize <= size_of::<xinpgen>() {
            break;
        }
        if which & xgn_ref.xgn_kind == 0 {
            which |= xgn_ref.xgn_kind;
            match xgn_ref.xgn_kind {
                XSO_SOCKET => so = &*(xgn as *mut xsocket_n),
                XSO_INPCB => inp = &*(xgn as *mut xinpcb_n),
                XSO_TCPCB => tp = xgn as *mut xtcpcb_n,
                _ => break,
                // case XSO_SOCKET:
                //     so = (struct xsocket_n *)xgn;
                //     break;
                // case XSO_RCVBUF:
                //     so_rcv = (struct xsockbuf_n *)xgn;
                //     break;
                // case XSO_SNDBUF:
                //     so_snd = (struct xsockbuf_n *)xgn;
                //     break;
                // case XSO_STATS:
                //     so_stat = (struct xsockstat_n *)xgn;
                //     break;
                // case XSO_INPCB:
                //     inp = (struct xinpcb_n *)xgn;
                //     break;
                // case XSO_TCPCB:
                //     tp = (struct xtcpcb_n *)xgn;
                //     break;
                // default:
                //     printf("unexpected kind %d\n", xgn->xgn_kind);
                //     break;
                // }
            }
        }
        let is_tcp = protocol == IPPROTO_TCP;
        if (is_tcp && which != ALL_XGN_KIND_TCP) || (!is_tcp && which != ALL_XGN_KIND_INP) {
            continue;
        }
        which = 0;
        /* Ignore sockets for protocols other than the desired one. */
        if so.xso_protocol != protocol as i32 {
            continue;
        }
        /* Ignore PCBs which were freed during copyout. */
        if inp.inp_gencnt > xig.xig_gen {
            continue;
        }
        next = next.offset(ROUNDUP64!(xig.xig_len) as isize);
        if (af == AF_INET && (inp.inp_vflag & INP_IPV4) == 0)
            || (af == AF_INET6 && (inp.inp_vflag & INP_IPV6) == 0)
        {
            continue;
        }
        if inp.inp_vflag & INP_IPV4 != 0 {
            // inetprint(&inp->inp_laddr, (int)inp->inp_lport, name, 1);
            // inetprint(&inp->inp_faddr, (int)inp->inp_fport, name, 1);
            inetprint(&inp.inp_dependladdr.inp46_local.ia46_addr4, inp.inp_lport);
            inetprint(&inp.inp_dependfaddr.inp46_foreign.ia46_addr4, inp.inp_fport);
        //             #define	inp_faddr	inp_dependfaddr.inp46_foreign.ia46_addr4
        // #define	inp_laddr	inp_dependladdr.inp46_local.ia46_addr4
        // #define	in6p_faddr	inp_dependfaddr.inp6_foreign
        // #define	in6p_laddr	inp_dependladdr.inp6_local
        } else if inp.inp_vflag & INP_IPV6 != 0 {
            inet6print(&inp.inp_dependladdr.inp6_local, inp.inp_lport);
            inet6print(&inp.inp_dependfaddr.inp6_foreign, inp.inp_fport);
            // inet6print(&inp->in6p_laddr, (int)inp->inp_lport, name, 1);
            // inet6print(&inp->in6p_faddr, (int)inp->inp_fport, name, 1);
        }
        println!("pid = {}", so.so_last_pid);
        // printf(" %6u %6u %6u %6u", so_rcv->sb_hiwat, so_snd->sb_hiwat,
        //            so->so_last_pid, so->so_e_pid);
    }
    Result::Ok(())
}

unsafe fn inetprint(in_: *const in_addr, port: c_ushort) {
    let addr_bytes = &*(in_ as *const u8 as *const [u8; 4]);
    println!("{:?}:{}", addr_bytes, port);
}

unsafe fn inet6print(in_: *const in6_addr, port: c_ushort) {
    let addr_bytes = &*(in_ as *const u8 as *const [u8; 4]);
    println!("{:?}:{}", addr_bytes, port);
}
