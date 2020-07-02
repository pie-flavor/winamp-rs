use libc::*;
use winapi::shared::{minwindef::*, windef::*};

pub const OUT_VER: c_int = 0x10;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Out_Module {
    pub version: c_int,
    pub description: *mut c_char,
    pub id: intptr_t,
    pub hMainWindow: HWND,
    pub hDllInstance: HINSTANCE,
    pub Config: unsafe extern "C" fn(hwndParent: HWND),
    pub About: unsafe extern "C" fn(hwndParent: HWND),
    pub Init: unsafe extern "C" fn(),
    pub Quit: unsafe extern "C" fn(),
    pub Open: unsafe extern "C" fn(
        samplerate: c_int,
        numchannels: c_int,
        bitspersamp: c_int,
        bufferlenms: c_int,
        prebufferms: c_int,
    ) -> c_int,
    pub Close: unsafe extern "C" fn(),
    pub Write: unsafe extern "C" fn(buf: *mut c_char, len: c_int) -> c_int,
    pub CanWrite: unsafe extern "C" fn() -> c_int,
    pub IsPlaying: unsafe extern "C" fn() -> c_int,
    pub Pause: unsafe extern "C" fn(pause: c_int) -> c_int,
    pub SetVolume: unsafe extern "C" fn(volume: c_int),
    pub SetPan: unsafe extern "C" fn(pan: c_int),
    pub Flush: unsafe extern "C" fn(t: c_int),
    pub GetOutputTime: unsafe extern "C" fn() -> c_int,
    pub GetWrittenTime: unsafe extern "C" fn() -> c_int,
}
