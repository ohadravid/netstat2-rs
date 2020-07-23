use libc::*;

#[cfg(target_os = "android")]
pub type __u8 = u8;
#[cfg(target_os = "android")]
pub type __u16 = u16;
#[cfg(target_os = "android")]
pub type __u32 = u32;
#[cfg(target_os = "android")]
pub type __u64 = u64;

pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
