#![allow(nonstandard_style)]

#[cfg(not(all(target_arch = "x86", windows)))]
compile_error!("The only supported platform is Windows 32-bit");

use std::os::raw::*;

#[repr(C)]
pub struct GeneralPurposePlugin {
    pub version: c_int,
    pub description: *mut c_char,
    pub init: unsafe extern "C" fn() -> c_int,
    pub config: unsafe extern "C" fn(),
    pub quit: unsafe extern "C" fn(),
    pub hwnd_parent: *mut (),
    pub h_dll_instance: *mut (),
}

pub mod wa_ipc;
