#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(unused)]
mod linux_bindings {
    include!(concat!(env!("OUT_DIR"), "/linux_bindings.rs"));
}

#[macro_use]
mod ffi;

mod api;
mod ext;
mod netlink_iterator;
mod procfs;

pub use self::api::*;
