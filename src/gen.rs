use libc::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

pub const GEN_INIT_SUCCESS: c_int = 0x1;
pub const GEN_PLUGIN_UNINSTALL_REBOOT: c_int = 0x0;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct winampGeneralPurposePlugin {
    pub version: c_int,
    pub description: *mut c_char,
    pub init: unsafe extern "C" fn() -> c_int,
    pub config: unsafe extern "C" fn(),
    pub quit: unsafe extern "C" fn(),
    pub hwnd_parent: HWND,
    pub h_dll_instance: HINSTANCE,
}

pub const GPPHDR_VER: c_int = 0x10;
pub type winampGeneralPurposePluginGetter =
    unsafe extern "C" fn() -> *mut winampGeneralPurposePlugin;
