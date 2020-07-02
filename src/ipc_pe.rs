use libc::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use std::fmt::{Debug, Formatter, Result as FmtResult};

pub const IPC_PE_GETCURINDEX: UINT = 100;
pub const IPC_PE_GETINDEXTOTAL: UINT = 101;
pub const IPC_PE_GETINDEXINFO: UINT = 102;
pub const IPC_PE_GETINDEXINFORESULT: UINT = 103;
pub const IPC_PE_DELETEINDEX: UINT = 104;
pub const IPC_PE_SWAPINDEX: UINT = 105;
pub const IPC_PE_INSERTFILENAME: UINT = 106;
pub const IPC_PE_GETDIRTY: UINT = 107;
pub const IPC_PE_SETCLEAN: UINT = 108;
pub const IPC_PE_GETIDXFROMPOINT: UINT = 109;
pub const IPC_PE_SAVEEND: UINT = 110;
pub const IPC_PE_RESTOREEND: UINT = 111;
pub const IPC_PE_GETNEXTSELECTED: UINT = 112;
pub const IPC_PE_GETSELECTEDCOUNT: UINT = 113;
pub const IPC_PE_INSERTFILENAMEW: UINT = 114;
pub const IPC_PE_GETINDEXINFO_TITLE: UINT = 115;
pub const IPC_PE_GETINDEXINFORESULT_TITLE: UINT = 116;
pub const IPC_PE_GETINDEXTITLE: UINT = 200;
pub const IPC_PE_GETINDEXTITLEW: UINT = 201;
pub const IPC_PE_GETINDEXINFO_INPROC: UINT = 202;
pub const IPC_PE_GETINDEXINFOW_INPROC: UINT = 203;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileinfo {
    pub file: [c_char; MAX_PATH],
    pub index: c_int,
}

impl Debug for fileinfo {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let &Self { ref file, index } = self;
        dbg::fileinfo { file, index }.fmt(f)
    }
}

impl PartialEq for fileinfo {
    fn eq(&self, other: &Self) -> bool {
        let &Self { ref file, index } = self;
        let s = dbg::fileinfo { file, index };
        let &Self { ref file, index } = other;
        let o = dbg::fileinfo { file, index };
        s == o
    }
}

impl Eq for fileinfo {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileinfoW {
    pub file: [wchar_t; MAX_PATH],
    pub index: c_int,
}

impl Debug for fileinfoW {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let &Self { ref file, index } = self;
        dbg::fileinfoW { file, index }.fmt(f)
    }
}

impl PartialEq for fileinfoW {
    fn eq(&self, other: &Self) -> bool {
        let &Self { ref file, index } = self;
        let s = dbg::fileinfoW { file, index };
        let &Self { ref file, index } = other;
        let o = dbg::fileinfoW { file, index };
        s == o
    }
}

impl Eq for fileinfoW {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct callbackinfo {
    pub callback: HWND,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileinfo2 {
    pub fileindex: c_int,
    pub filetitle: [c_char; 256],
    pub filelength: [c_char; 16],
}

impl Debug for fileinfo2 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let &Self { fileindex, ref filetitle, ref filelength } = self;
        dbg::fileinfo2 { fileindex, filetitle, filelength }.fmt(f)
    }
}

impl PartialEq for fileinfo2 {
    fn eq(&self, other: &Self) -> bool {
        let &Self { fileindex, ref filetitle, ref filelength } = self;
        let s = dbg::fileinfo2 { fileindex, filetitle, filelength };
        let &Self { fileindex, ref filetitle, ref filelength } = other;
        let o = dbg::fileinfo2 { fileindex, filetitle, filelength };
        s == o
    }
}

impl Eq for fileinfo2 {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fileinfo2W {
    pub fileindex: c_int,
    pub filetitle: [wchar_t; 256],
    pub filelength: [wchar_t; 16],
}

impl Debug for fileinfo2W {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let &Self { fileindex, ref filetitle, ref filelength } = self;
        dbg::fileinfo2W { fileindex, filetitle, filelength }.fmt(f)
    }
}

impl PartialEq for fileinfo2W {
    fn eq(&self, other: &Self) -> bool {
        let &Self { fileindex, ref filetitle, ref filelength } = self;
        let s = dbg::fileinfo2W { fileindex, filetitle, filelength };
        let &Self { fileindex, ref filetitle, ref filelength } = other;
        let o = dbg::fileinfo2W { fileindex, filetitle, filelength };
        s == o
    }
}

impl Eq for fileinfo2W {}

mod dbg {
    use libc::*;
    #[derive(PartialEq, Debug)]
    pub struct fileinfo<'a> {
        pub file: &'a [c_char],
        pub index: c_int,
    }
    #[derive(PartialEq, Debug)]
    pub struct fileinfoW<'a> {
        pub file: &'a [wchar_t],
        pub index: c_int,
    }
    #[derive(PartialEq, Debug)]
    pub struct fileinfo2<'a> {
        pub fileindex: c_int,
        pub filetitle: &'a [c_char],
        pub filelength: &'a [c_char],
    }
    #[derive(PartialEq, Debug)]
    pub struct fileinfo2W<'a> {
        pub fileindex: c_int,
        pub filetitle: &'a [wchar_t],
        pub filelength: &'a [wchar_t],
    }
}
