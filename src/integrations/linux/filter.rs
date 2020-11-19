use crate::integrations::linux::ffi::*;
use core::marker::PhantomData;
use libc::*;
use std::mem::{size_of, size_of_val};
use std::net::Ipv4Addr;

pub fn create_filter(conds: &[PortCond]) -> Vec<inet_diag_bc_op> {
    let unit_len = size_of::<inet_diag_bc_op>();
    let count = conds.len();
    let total_len = count * unit_len * 2;
    let mut ret = Vec::with_capacity(total_len);

    for cond in conds {
        let end = total_len - ret.len() * unit_len;
        let next = unit_len * 2;
        let fail = end + unit_len;

        cond.write_bytecode(&mut ret, next as u8, fail as u8);
    }

    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PortCondOp {
    Eq,
    Ne,
    Le,
    Ge,
}

#[derive(Debug, Clone, Copy)]
pub enum PortCondType {
    Src,
    Dst,
}

impl PortCondType {
    pub fn eq(self, value: u16) -> PortCond {
        PortCond::new(PortCondOp::Eq, self, value)
    }
    pub fn ne(self, value: u16) -> PortCond {
        PortCond::new(PortCondOp::Ne, self, value)
    }
    pub fn le(self, value: u16) -> PortCond {
        PortCond::new(PortCondOp::Le, self, value)
    }
    pub fn ge(self, value: u16) -> PortCond {
        PortCond::new(PortCondOp::Ge, self, value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PortCond {
    op: PortCondOp,
    value: u16,
    typ: PortCondType,
}

impl PortCond {
    fn new(op: PortCondOp, typ: PortCondType, value: u16) -> Self {
        Self { op, value, typ }
    }

    fn write_bytecode(&self, ret: &mut Vec<inet_diag_bc_op>, mut yes: u8, mut no: u8) {
        let code = match self.typ {
            PortCondType::Src => match self.op {
                PortCondOp::Eq | PortCondOp::Ne => FilterOpRaw::S_EQ,
                PortCondOp::Le => FilterOpRaw::S_LE,
                PortCondOp::Ge => FilterOpRaw::S_GE,
            },
            PortCondType::Dst => match self.op {
                PortCondOp::Eq | PortCondOp::Ne => FilterOpRaw::D_EQ,
                PortCondOp::Le => FilterOpRaw::D_LE,
                PortCondOp::Ge => FilterOpRaw::D_GE,
            },
        };

        if self.op == PortCondOp::Ne {
            std::mem::swap(&mut yes, &mut no);
        }

        ret.push(inet_diag_bc_op {
            code,
            yes: yes,
            no: no as u16,
        });
        ret.push(inet_diag_bc_op {
            code: FilterOpRaw::NOP,
            yes: 0,
            no: self.value,
        })
    }
}
