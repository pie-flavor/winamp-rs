#![allow(nonstandard_style)]

#[cfg(not(all(target_arch = "x86", windows)))]
compile_error!("The only supported platform is Windows 32-bit");

use std::os::raw::*;

pub mod wa_ipc;
pub mod ipc_pe;
pub mod wa_dlg;
pub mod DSP;
pub mod GEN;
pub mod IN2;
pub mod OUT;
