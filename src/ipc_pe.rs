use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use libc::*;

pub const IPC_PE_GETCURINDEX: UINT =        100;
pub const IPC_PE_GETINDEXTOTAL: UINT =      101;
pub const IPC_PE_GETINDEXINFO: UINT =       102;
pub const IPC_PE_GETINDEXINFORESULT: UINT = 103;
pub const IPC_PE_DELETEINDEX: UINT =        104;
pub const IPC_PE_SWAPINDEX: UINT =          105;
pub const IPC_PE_INSERTFILENAME: UINT =     106;
pub const IPC_PE_GETDIRTY: UINT =           107;
pub const IPC_PE_SETCLEAN	: UINT =          108;
pub const IPC_PE_GETIDXFROMPOINT: UINT =    109;
pub const IPC_PE_SAVEEND: UINT =            110;
pub const IPC_PE_RESTOREEND: UINT =         111;
pub const IPC_PE_GETNEXTSELECTED: UINT =    112;
pub const IPC_PE_GETSELECTEDCOUNT: UINT =   113;
pub const IPC_PE_INSERTFILENAMEW: UINT =  114;
pub const IPC_PE_GETINDEXINFO_TITLE: UINT = 115;
pub const IPC_PE_GETINDEXINFORESULT_TITLE: UINT = 116;
pub const IPC_PE_GETINDEXTITLE: UINT =      200;
pub const IPC_PE_GETINDEXTITLEW: UINT =      201;
pub const IPC_PE_GETINDEXINFO_INPROC: UINT =      202;
pub const IPC_PE_GETINDEXINFOW_INPROC: UINT =      203;

#[repr(C)]
pub struct fileinfo {
    pub file: [c_char; MAX_PATH],
    pub index: c_int,
}

#[repr(C)]
pub struct fileinfoW {
    pub file: [wchar_t; MAX_PATH],
    pub index: c_int,
}

#[repr(C)]
pub struct callbackinfo {
    pub callback: HWND,
    pub index: c_int,
}

#[repr(C)]
pub struct fileinfo2 {
    pub fileindex: c_int,
    pub filetitle: [c_char; 256],
    pub filelength: [c_char; 16],
}

#[repr(C)]
pub struct fileinfo2W {
    pub fileindex: c_int,
    pub filetitle: [wchar_t; 256],
    pub filelength: [wchar_t; 16],
}
