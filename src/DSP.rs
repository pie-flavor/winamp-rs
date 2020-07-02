use libc::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

#[repr(C)]
pub struct winampDSPModule {
    pub description: *const c_char,
    pub hwndParent: HWND,
    pub hDllInstance: HINSTANCE,
    pub Config: unsafe extern "C" fn(this_mod: *mut winampDSPModule),
    pub Init: unsafe extern "C" fn(this_mod: *mut winampDSPModule) -> c_int,
    pub ModifySamples: unsafe extern "C" fn(this_mod: *mut winampDSPModule, samples: *mut c_short, numsamples: c_int, bps: c_int, nch: c_int, srate: c_int) -> c_int,
    pub Quit: unsafe extern "C" fn(this_mod: *mut winampDSPModule),
    pub userData: *mut c_void,
}

#[repr(C)]
pub struct winampDSPHeader {
    pub version: c_int,
    pub description: *mut c_char,
    pub getModule: unsafe extern "C" fn(c_int) -> *mut winampDSPModule,
    pub sf: unsafe extern "C" fn(key: c_int) -> c_int,
}

pub type winampDSPGetHeaderType = unsafe extern "C" fn(HWND) -> *mut winampDSPHeader;

pub const DSP_PLUGIN_UNINSTALL_NOW: c_int = 0x0;
pub const DSP_PLUGIN_UNINSTALL_REBOOT: c_int = 0x1;

pub const DSP_HDRVER: c_int = 0x20;
