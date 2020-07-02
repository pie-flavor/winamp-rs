use libc::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;

pub const IN_UNICODE: c_int = 0x0F000000;
pub type in_char = wchar_t;
pub const IN_VER: c_int = IN_UNICODE | 0x100;

pub const IN_MODULE_FLAG_USES_OUTPUT_PLUGIN: c_int = 1;
pub const IN_MODULE_FLAG_EQ: c_int = 2;
pub const IN_MODULE_FLAG_REPLAYGAIN: c_int = 8;
pub const IN_MODULE_FLAG_REPLAYGAIN_PREAMP: c_int = 16;
pub const GETFILEINFO_TITLE_LENGTH: c_int = 2048;
pub const INFOBOX_EDITED: c_int = 0;
pub const INFOBOX_UNCHANGED: c_int = 1;

#[repr(C)]
pub struct In_Module {
    pub version: c_int,
    pub description: *mut c_char,
    pub hMainWindow: HWND,
    pub hDllInstance: HINSTANCE,
    pub FileExtensions: *mut c_char,
    pub is_seekable: c_int,
    pub UsesOutputPlug: c_int,
    pub Config: unsafe extern "C" fn(hwndParent: HWND),
    pub About: unsafe extern "C" fn(hwndParent: HWND),
    pub Init: unsafe extern "C" fn(),
    pub Quit: unsafe extern "C" fn(),
    pub GetFileInfo: unsafe extern "C" fn(file: *const in_char, title: *mut in_char, length_in_ms: *mut c_int),
    pub InfoBox: unsafe extern "C" fn(file: *const in_char, hwndParent: HWND) -> c_int,
    pub IsOurFile: unsafe extern "C" fn(r#fn: *const in_char) -> c_int,
    pub Play: unsafe extern "C" fn(r#fn: *const in_char) -> c_int,
    pub Pause: unsafe extern "C" fn(),
    pub UnPause: unsafe extern "C" fn(),
    pub IsPaused: unsafe extern "C" fn() -> c_int,
    pub Stop: unsafe extern "C" fn(),
    pub GetLength: unsafe extern "C" fn() -> c_int,
    pub GetOutputTime: unsafe extern "C" fn() -> c_int,
    pub SetOutputTime: unsafe extern "C" fn(time_in_ms: c_int),
    pub SetVolume: unsafe extern "C" fn(volume: c_int),
    pub SetPan: unsafe extern "C" fn(pan: c_int),
    pub SAVSAInit: unsafe extern "C" fn(maxlatency_in_ms: c_int, srate: c_int),
    pub SAVSADeInit: unsafe extern "C" fn(),
    pub SAAddPCMData: unsafe extern "C" fn(PCMData: *mut c_void, nch: c_int, bps: c_int, timestamp: c_int),
    pub SAGetMode: unsafe extern "C" fn() -> c_int,
    pub SAAdd: unsafe extern "C" fn(data: *mut c_void, timestamp: c_int, csa: c_int) -> c_int,
    pub VSAAddPCMData: unsafe extern "C" fn(PCMData: *mut c_void, nch: c_int, bps: c_int, timestamp: c_int),
    pub VSAGetMode: unsafe extern "C" fn(specNch: *mut c_int, waveNch: *mut c_int) -> c_int,
    pub VSAAdd: unsafe extern "C" fn(data: *mut c_void, timestamp: c_int) -> c_int,
    pub VSASetInfo: unsafe extern "C" fn(srate: c_int, nch: c_int),
    pub dsp_isactive: unsafe extern "C" fn() -> c_int,
    pub dsp_dosamples: unsafe extern "C" fn(samples: *mut c_short, numsamples: c_int, bps: c_int, nch: c_int, srate: c_int) -> c_int,
    pub EQSet: unsafe extern "C" fn(on: c_int, data: *mut c_char, preamp: c_int),
    pub SetInfo: unsafe extern "C" fn(bitrate: c_int, srate: c_int, stereo: c_int, synched: c_int),
    pub outMod: *mut crate::OUT::Out_Module,
}

pub const IN_PLUGIN_UNINSTALL_NOW: c_int = 0x1;
pub const IN_PLUGIN_UNINSTALL_REBOOT: c_int = 0x0;