#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod libproc_bindings {
    include!(concat!(env!("OUT_DIR"), "/libproc_bindings.rs"));
}

mod api;
mod ext;
mod netstat;

pub use self::api::*;
