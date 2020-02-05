use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{read_dir, read_link};

pub fn build_hash_of_pids_by_inode() -> HashMap<u32, HashSet<u32>> {
    let pids = read_dir("/proc/")
        .expect("Can't read /proc/")
        .filter_map(|d| d.ok()?.file_name().to_str()?.parse::<u32>().ok());
    let mut pid_by_inode = HashMap::new();
    for pid in pids {
        if let Result::Ok(fds) = read_dir(format!("/proc/{}/fd", pid)) {
            let inodes = fds.filter_map(|fd| {
                let fd_file_name = fd.ok()?.file_name();
                let fd_str = fd_file_name.to_str()?;
                let path_buf = read_link(format!("/proc/{}/fd/{}", pid, fd_str)).ok()?;
                let link_str = path_buf.to_str()?;
                if link_str.starts_with("socket:[") {
                    let inode_str = &link_str[8..link_str.len() - 1];
                    inode_str.parse::<u32>().ok()
                } else {
                    Option::None
                }
            });
            for inode in inodes {
                pid_by_inode
                    .entry(inode)
                    .and_modify(|v: &mut HashSet<u32>| {
                        v.insert(pid);
                    })
                    .or_insert_with(|| {
                        let mut s = HashSet::new();
                        s.insert(pid);
                        s
                    });
            }
        }
    }
    pid_by_inode
}
