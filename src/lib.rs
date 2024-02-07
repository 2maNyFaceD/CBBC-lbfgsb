#![allow(nonstandard_style)]

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::{integer, logical};

extern "C" {
    #[allow(clashing_extern_declarations)]
    pub fn setulb(
        n: *mut integer,
        m: *mut integer,
        x: *mut f64,
        l: *const f64,
        u: *const f64,
        nbd: *const integer,
        f: *mut f64,
        g: *mut f64,
        factr: *const f64,
        pgtol: *const f64,
        wa: *mut f64,
        iwa: *mut integer,
        task: *mut u32,
        iprint: *const integer,
        csave: *mut integer,
        lsave: *mut logical,
        isave: *mut integer,
        dsave: *mut f64,
    ) -> ::std::os::raw::c_int;
}


mod lbfgsb;

pub use crate::lbfgsb::lbfgsb;
pub use crate::lbfgsb::LbfgsbState;