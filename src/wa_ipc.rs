use libc::{c_char, c_double, c_int, c_uchar, c_uint, c_ulong, c_void, intptr_t, size_t, wchar_t};
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HBITMAP, HWND, RECT};
use winapi::um::winuser::{SendMessageW, WM_USER};

pub const WM_WA_IPC: UINT = WM_USER;

#[inline(always)]
pub const fn WINAMP_VERSION_MAJOR(version: u32) -> u32 {
    (version & 0x0000FF00) >> 12
}

#[inline(always)]
pub const fn WINAMP_VERSION_MINOR(version: u32) -> u32 {
    version & 0x000000FF
}

pub const IPC_GETVERSION: UINT = 0;
pub const IPC_GETVERSIONSTRING: UINT = 1;
pub const IPC_GETREGISTEREDVERSION: UINT = 770;

#[repr(C)]
pub struct enqueueFileWithMetaStruct {
    pub filename: *mut c_char,
    pub title: *mut c_char,
    pub length: c_int,
}

#[repr(C)]
pub struct enqueueFileWithMetaStructW {
    pub filename: *mut wchar_t,
    pub title: *mut wchar_t,
    pub length: c_int,
}

pub const IPC_PLAYFILE: UINT = 100;
pub const IPC_ENQUEUEFILE: UINT = 100;
pub const IPC_PLAYFILEW: UINT = 1100;
pub const IPC_ENQUEUEFILEW: UINT = 1100;

pub const IPC_DELETE: UINT = 101;
pub const IPC_DELETE_INT: UINT = 1101;

pub const IPC_STARTPLAY: UINT = 102;
pub const IPC_STARTPLAY_INT: UINT = 1102;
pub const IPC_IPC_CHDIR: UINT = 103;
pub const IPC_ISPLAYING: UINT = 104;
pub const IPC_GETOUTPUTTIME: UINT = 105;
pub const IPC_JUMPTOTIME: UINT = 106;
pub const IPC_GETMODULENAME: UINT = 109;
pub const IPC_EX_ISRIGHTEXE: UINT = 666;
pub const IPC_WRITEPLAYLIST: UINT = 120;
pub const IPC_SETPLAYLISTPOS: UINT = 121;
pub const IPC_SETVOLUME: UINT = 122;

#[inline(always)]
pub unsafe fn IPC_GETVOLUME(hwnd: HWND) -> LRESULT {
    SendMessageW(hwnd, WM_WA_IPC, -666_isize as _, IPC_SETVOLUME as _)
}

pub const IPC_SETPANNING: UINT = 123;
pub const IPC_GETLISTLENGTH: UINT = 124;
pub const IPC_GETLISTPOS: UINT = 125;
pub const IPC_GETINFO: UINT = 126;
pub const IPC_GETEQDATA: UINT = 127;
pub const IPC_SETEQDATA: UINT = 128;
pub const IPC_ADDBOOKMARK: UINT = 129;
pub const IPC_ADDBOOKMARKW: UINT = 131;
pub const IPC_INSTALLPLUGIN: UINT = 130;
pub const IPC_RESTARTWINAMP: UINT = 135;
pub const IPC_ISFULLSTOP: UINT = 400;
pub const IPC_INETAVAILABLE: UINT = 242;
pub const IPC_UPDTITLE: UINT = 243;
pub const IPC_REFRESHPLCACHE: UINT = 247;
pub const IPC_GET_SHUFFLE: UINT = 250;
pub const IPC_GET_REPEAT: UINT = 251;
pub const IPC_SET_SHUFFLE: UINT = 252;
pub const IPC_SET_REPEAT: UINT = 253;
pub const IPC_ENABLEDISABLE_ALL_WINDOWS: UINT = 259;
pub const IPC_GETWND: UINT = 260;
pub const IPC_GETWND_EQ: UINT = 0;
pub const IPC_GETWND_PE: UINT = 1;
pub const IPC_GETWND_MB: UINT = 2;
pub const IPC_GETWND_VIDEO: UINT = 3;
pub const IPC_ISWNDVISIBLE: UINT = 261;
pub const IPC_SETSKINW: UINT = 199;
pub const IPC_SETSKIN: UINT = 200;
pub const IPC_GETSKIN: UINT = 201;
pub const IPC_GETSKINW: UINT = 1201;
pub const IPC_EXECPLUG: UINT = 202;
pub const IPC_GETPLAYLISTFILE: UINT = 211;
pub const IPC_GETPLAYLISTFILEW: UINT = 214;
pub const IPC_GETPLAYLISTTITLE: UINT = 212;
pub const IPC_GETPLAYLISTTITLEW: UINT = 213;
pub const IPC_GETHTTPGETTER: UINT = 240;
pub const IPC_GETHTTPGETTERW: UINT = 1240;
pub const IPC_MBOPEN: UINT = 241;
pub const IPC_CHANGECURRENTFILE: UINT = 245;
pub const IPC_CHANGECURRENTFILEW: UINT = 1245;

pub const IPC_GETMBURL: UINT = 246;

pub const IPC_MBBLOCK: UINT = 248;

pub const IPC_MBOPENREAL: UINT = 249;

pub const IPC_ADJUST_OPTIONSMENUPOS: UINT = 280;

pub const IPC_GET_HMENU: UINT = 281;

pub const IPC_GET_EXTENDED_FILE_INFO: UINT = 290;
pub const IPC_GET_EXTENDED_FILE_INFO_HOOKABLE: UINT = 296;

#[repr(C)]
pub struct extendedFileInfoStruct {
    pub filename: *mut c_char,
    pub metadata: *mut c_char,
    pub ret: *mut c_char,
    pub retlen: size_t,
}

pub const IPC_GET_BASIC_FILE_INFO: UINT = 291;
#[repr(C)]
pub struct basicFileInfoStruct {
    pub filename: *mut c_char,
    pub quickCheck: c_int,
    pub length: c_int,
    pub title: *mut c_char,
    pub titleLen: c_int,
}

pub const IPC_GET_BASIC_FILE_INFOW: UINT = 1291;
#[repr(C)]
pub struct basicFileInfoStructW {
    pub filename: *mut wchar_t,
    pub quickCheck: c_int,
    pub length: c_int,
    pub title: *mut wchar_t,
    pub titleLen: c_int,
}
pub const IPC_GET_EXTLIST: UINT = 292;
pub const IPC_GET_EXTLISTW: UINT = 1292;
pub const IPC_INFOBOX: UINT = 293;
#[repr(C)]
pub struct infoBoxParam {
    pub parent: HWND,
    pub filename: *mut wchar_t,
}

pub const IPC_SET_EXTENDED_FILE_INFO: UINT = 294;
pub const IPC_WRITE_EXTENDED_FILE_INFO: UINT = 295;
pub const IPC_FORMAT_TITLE: UINT = 297;
#[repr(C)]
pub struct waFormatTitle {
    pub spec: *mut c_char,
    pub p: *mut c_void,
    pub out: *mut c_char,
    pub out_len: c_int,
    pub TAGFUNC: unsafe extern "C" fn(tag: *const c_char, p: *mut c_void) -> *mut c_char,
    pub TAGFREEFUNC: unsafe extern "C" fn(tag: *mut c_char, p: *mut c_void),
}
pub const IPC_FORMAT_TITLE_EXTENDED: UINT = 298;
#[repr(C)]
pub struct waFormatTitleExtended {
    pub filename: *const wchar_t,
    pub useExtendedInfo: c_int,
    pub spec: *const wchar_t,
    pub p: *mut c_void,
    pub out: *mut wchar_t,
    pub out_len: c_int,
    pub TAGFUNC: unsafe extern "C" fn(tag: *const wchar_t, p: *mut c_void) -> *mut wchar_t,
    pub TAGFREEFUNC: unsafe extern "C" fn(tag: *mut wchar_t, p: *mut c_void),
}

pub const IPC_COPY_EXTENDED_FILE_INFO: UINT = 299;
#[repr(C)]
pub struct copyFileInfoStruct {
    pub source: *const c_char,
    pub dest: *const c_char,
}
pub const IPC_COPY_EXTENDED_FILE_INFOW: UINT = 1299;
#[repr(C)]
pub struct copyFileInfoStructW {
    pub source: *const wchar_t,
    pub dest: *const wchar_t,
}
#[repr(C)]
pub struct wa_inflate_struct {
    pub inflateReset: unsafe extern "C" fn(strm: *mut c_void) -> c_int,
    pub inflateInit_: unsafe extern "C" fn(
        strm: *mut c_void,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int,
    pub inflate: unsafe extern "C" fn(strm: *mut c_void, flush: c_int) -> c_int,
    pub inflateEnd: unsafe extern "C" fn(strm: *mut c_void) -> c_int,
    pub crc32: unsafe extern "C" fn(crc: c_ulong, buf: *const c_uchar, len: c_uint) -> c_ulong,
}

pub const IPC_GETUNCOMPRESSINTERFACE: UINT = 331;

#[repr(C)]
pub struct prefsDlgRec {
    pub hInst: HINSTANCE,
    pub dlgID: c_int,
    pub proc: *mut c_void,
    pub name: *mut c_char,
    pub r#where: intptr_t,
    pub _id: intptr_t,
    pub next: *mut prefsDlgRec,
}

#[repr(C)]
pub struct prefsDlgRecW {
    pub hInst: HINSTANCE,
    pub dlgID: c_int,
    pub proc: *mut c_void,
    pub name: *mut wchar_t,
    pub r#where: intptr_t,
    pub _id: intptr_t,
    pub next: *mut prefsDlgRec,
}
pub const IPC_ADD_PREFS_DLG: UINT = 332;
pub const IPC_ADD_PREFS_DLGW: UINT = 1332;
pub const IPC_REMOVE_PREFS_DLG: UINT = 333;

pub const IPC_OPENPREFSTOPAGE: UINT = 380;

pub const IPC_GETINIFILE: UINT = 334;

pub const IPC_GETINIDIRECTORY: UINT = 335;

pub const IPC_GETPLUGINDIRECTORY: UINT = 336;

pub const IPC_GETM3UDIRECTORY: UINT = 337;

pub const IPC_GETM3UDIRECTORYW: UINT = 338;

pub const IPC_SPAWNBUTTONPOPUP: UINT = 361;

pub const IPC_OPENURLBOX: UINT = 360;

pub const IPC_OPENFILEBOX: UINT = 362;

pub const IPC_OPENDIRBOX: UINT = 363;

pub const IPC_SETDIALOGBOXPARENT: UINT = 364;

pub const IPC_GETDIALOGBOXPARENT: UINT = 365;

pub const IPC_UPDATEDIALOGBOXPARENT: UINT = 366;

pub const IPC_DRO_MIN: UINT = 401;
pub const IPC_SET_JTF_COMPARATOR: UINT = 409;

pub const IPC_SET_JTF_COMPARATOR_W: UINT = 410;

pub const IPC_SET_JTF_DRAWTEXT: UINT = 416;

pub const IPC_DRO_MAX: UINT = 499;

pub const IPC_GET_GENSKINBITMAP: UINT = 503;

pub const EMBED_FLAGS_NORESIZE: c_int = 0x1;
pub const EMBED_FLAGS_NOTRANSPARENCY: c_int = 0x2;
pub const EMBED_FLAGS_NOWINDOWMENU: c_int = 0x4;
pub const EMBED_FLAGS_GUID: c_int = 0x8;

#[inline(always)]
pub unsafe fn SET_EMBED_GUID(state: *mut embedWindowState, guid: GUID) {
    (*state).flags |= EMBED_FLAGS_GUID as c_int;
    *(&mut (*state).extra_data[4] as *mut _ as *mut GUID) = guid;
}

#[inline(always)]
pub unsafe fn GET_EMBED_GUID(state: *mut embedWindowState) -> GUID {
    *(&(*state).extra_data[4] as *const _ as *const GUID)
}

#[repr(C)]
pub struct embedWindowState {
    pub me: HWND,
    pub flags: c_int,
    pub r: RECT,
    pub user_ptr: *mut c_void,
    pub extra_data: [c_int; 64],
}

pub const IPC_GET_EMBEDIF: UINT = 505;
pub const IPC_SKINWINDOW: UINT = 534;

#[repr(C)]
pub struct __SKINWINDOWPARAM {
    pub hwndToSkin: HWND,
    pub windowGuid: GUID,
}

pub const IPC_EMBED_ENUM: UINT = 532;
#[repr(C)]
pub struct embedEnumStruct {
    pub enumProc:
        unsafe extern "C" fn(ws: *mut embedWindowState, param: *mut embedEnumStruct) -> c_int,
    pub user_data: c_int,
}

pub const IPC_EMBED_ISVALID: UINT = 533;
pub const IPC_CONVERTFILE: UINT = 506;
#[repr(C)]
pub struct convertFileStruct {
    pub sourcefile: *mut c_char,
    pub destfile: *mut c_char,
    pub destformat: [intptr_t; 8],
    pub callbackhwnd: HWND,
    pub error: *mut c_char,
    pub bytes_done: c_int,
    pub bytes_total: c_int,
    pub bytes_out: c_int,
    pub killswitch: c_int,
    pub extra_data: [intptr_t; 64],
}
pub const IPC_CONVERTFILEW: UINT = 515;
#[repr(C)]
pub struct convertFileStructW {
    pub sourcefile: *mut wchar_t,
    pub destfile: *mut wchar_t,
    pub destformat: [intptr_t; 8],
    pub callbackhwnd: HWND,
    pub error: *mut wchar_t,
    pub bytes_done: c_int,
    pub bytes_total: c_int,
    pub bytes_out: c_int,
    pub killswitch: c_int,
    pub extra_data: [intptr_t; 64],
}
pub const IPC_CONVERTFILE_END: UINT = 507;
pub const IPC_CONVERTFILEW_END: UINT = 516;
#[repr(C)]
pub struct convertConfigStruct {
    pub hwndParent: HWND,
    pub format: c_int,
    pub hwndConfig: HWND,
    pub extra_data: [c_int; 8],
}

pub const IPC_CONVERT_CONFIG: UINT = 508;
pub const IPC_CONVERT_CONFIG_END: UINT = 509;
#[repr(C)]
pub struct converterEnumFmtStruct {
    pub enumProc: unsafe extern "C" fn(user_data: intptr_t, desc: *const c_char, fourcc: c_int),
    pub user_data: intptr_t,
}
#[repr(C)]
pub struct burnCDStruct {
    pub cdletter: c_char,
    pub playlist_file: *mut c_char,
    pub callback_hwnd: HWND,
    pub error: *mut c_char,
}
pub const IPC_BURN_CD: UINT = 511;
#[repr(C)]
pub struct convertSetPriority {
    pub cfs: *mut convertFileStruct,
    pub priority: c_int,
}
pub const IPC_CONVERT_SET_PRIORITY: UINT = 512;
#[repr(C)]
pub struct convertsetPriorityW {
    pub cfs: *mut convertFileStructW,
    pub priority: c_int,
}
pub const IPC_CONVERT_SET_PRIORITYW: UINT = 517;
#[repr(C)]
pub struct convertConfigItem {
    pub format: c_uint,
    pub item: *mut c_char,
    pub data: *mut c_char,
    pub len: c_int,
    pub configfile: *mut c_char,
}

pub const IPC_CONVERT_CONFIG_SET_ITEM: UINT = 513;
pub const IPC_CONVERT_CONFIG_GET_ITEM: UINT = 514;
#[repr(C)]
pub struct waHookTitleStruct {
    pub filename: *const c_char,
    pub title: *mut c_char,
    pub length: c_int,
    pub force_useformatting: c_int,
}

pub const IPC_HOOK_TITLES: UINT = 850;
#[repr(C)]
pub struct waHookTitleStructW {
    pub filename: *const wchar_t,
    pub title: *mut wchar_t,
    pub length: c_int,
    pub force_useformatting: c_int,
}

pub const IPC_HOOK_TITLESW: UINT = 851;

pub const IPC_GETSADATAFUNC: UINT = 800;

pub const IPC_GETVUDATAFUNC: UINT = 801;

pub const IPC_ISMAINWNDVISIBLE: UINT = 900;

#[repr(C)]
pub struct waSetPlColorsStruct {
    pub numElems: c_int,
    pub elems: *mut c_int,
    pub bm: HBITMAP,
}

pub const IPC_SETPLEDITCOLORS: UINT = 920;
#[repr(C)]
pub struct waSpawnMenuParms {
    pub wnd: HWND,
    pub xpos: c_int,
    pub ypos: c_int,
}
#[repr(C)]
pub struct waSpawnMenuParms2 {
    pub wnd: HWND,
    pub xpos: c_int,
    pub ypos: c_int,
    pub width: c_int,
    pub height: c_int,
}
pub const IPC_SPAWNEQPRESETMENU: UINT = 933;
pub const IPC_SPAWNFILEMENU: UINT = 934;
pub const IPC_SPAWNOPTIONSMENU: UINT = 935;
pub const IPC_SPAWNWINDOWSMENU: UINT = 936;
pub const IPC_SPAWNHELPMENU: UINT = 937;
pub const IPC_SPAWNPLAYMENU: UINT = 938;
pub const IPC_SPAWNPEFILEMENU: UINT = 939;
pub const IPC_SPAWNPEPLAYLISTMENU: UINT = 940;
pub const IPC_SPAWNPESORTMENU: UINT = 941;
pub const IPC_SPAWNPEHELPMENU: UINT = 942;
pub const IPC_SPAWNMLFILEMENU: UINT = 943;
pub const IPC_SPAWNMLVIEWMENU: UINT = 944;
pub const IPC_SPAWNMLHELPMENU: UINT = 945;
pub const IPC_SPAWNPELISTOFPLAYLISTS: UINT = 946;
pub const WM_WA_SYSTRAY: UINT = WM_USER + 1;

pub const WM_WA_MPEG_EOF: UINT = WM_USER + 2;

pub const IPC_IS_PLAYING_VIDEO: UINT = 501;
pub const IPC_GET_IVIDEOOUTPUT: UINT = 500;
#[inline(always)]
pub fn VIDEO_MAKETYPE(a: u32, b: u32, c: u32, d: u32) -> u32 {
    A | (B << 8) | (C << 16) | (D << 24)
}

pub const VIDUSER_SET_INFOSTRING: UINT = 0x1000;
pub const VIDUSER_GET_VIDEOHWND: UINT = 0x1001;
pub const VIDUSER_SET_VFLIP: UINT = 0x1002;
pub const VIDUSER_SET_TRACKSELINTERFACE: UINT = 0x1003;
pub const VIDUSER_OPENVIDEORENDERER: UINT = 0x1004;
pub const VIDUSER_CLOSEVIDEORENDERER: UINT = 0x1005;
pub const VIDUSER_GETPOPUPMENU: UINT = 0x1006;
pub const VIDUSER_SET_INFOSTRINGW: UINT = 0x1007;

#[repr(C)]
pub struct VideoOpenStruct {
    pub w: c_int,
    pub h: c_int,
    pub vflip: c_int,
    pub aspectratio: c_double,
    pub fmt: c_uint,
}

pub const IPC_CB_WND_EQ: UINT = 0;
pub const IPC_CB_WND_PE: UINT = 1;
pub const IPC_CB_WND_MB: UINT = 2;
pub const IPC_CB_WND_VIDEO: UINT = 3;
pub const IPC_CB_WND_MAIN: UINT = 4;

pub const IPC_CB_ONSHOWWND: UINT = 600;
pub const IPC_CB_ONHIDEWND: UINT = 601;

pub const IPC_CB_GETTOOLTIP: UINT = 602;

pub const IPC_CB_MISC: UINT = 603;
pub const IPC_CB_MISC_TITLE: UINT = 0;
pub const IPC_CB_MISC_VOLUME: UINT = 1;
pub const IPC_CB_MISC_STATUS: UINT = 2;
pub const IPC_CB_MISC_EQ: UINT = 3;
pub const IPC_CB_MISC_INFO: UINT = 4;
pub const IPC_CB_MISC_VIDEOINFO: UINT = 5;
pub const IPC_CB_MISC_TITLE_RATING: UINT = 6;

pub const IPC_CB_CONVERT_STATUS: UINT = 604;
pub const IPC_CB_CONVERT_DONE: UINT = 605;

pub const IPC_ADJUST_FFWINDOWSMENUPOS: UINT = 606;

pub const IPC_ISDOUBLESIZE: UINT = 608;

pub const IPC_ADJUST_FFOPTIONSMENUPOS: UINT = 609;

pub const IPC_GETTIMEDISPLAYMODE: UINT = 610;

pub const IPC_SETVISWND: UINT = 611;

pub const ID_VIS_NEXT: UINT = 40382;
pub const ID_VIS_PREV: UINT = 40383;
pub const ID_VIS_RANDOM: UINT = 40384;
pub const ID_VIS_FS: UINT = 40389;
pub const ID_VIS_CFG: UINT = 40390;
pub const ID_VIS_MENU: UINT = 40391;

pub const IPC_GETVISWND: UINT = 612;

pub const IPC_ISVISRUNNING: UINT = 613;

pub const IPC_CB_VISRANDOM: UINT = 628;

pub const IPC_SETIDEALVIDEOSIZE: UINT = 614;

pub const IPC_GETSTOPONVIDEOCLOSE: UINT = 615;

pub const IPC_SETSTOPONVIDEOCLOSE: UINT = 616;

#[repr(C)]
pub struct transAccelStruct {
    pub hwnd: HWND,
    pub uMsg: c_int,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
}

pub const IPC_TRANSLATEACCELERATOR: UINT = 617;
#[repr(C)]
pub struct windowCommand {
    pub cmd: c_int,
    pub x: c_int,
    pub y: c_int,
    pub align: c_int,
}

pub const IPC_CB_ONTOGGLEAOT: UINT = 618;

pub const IPC_GETPREFSWND: UINT = 619;

pub const IPC_SET_PE_WIDTHHEIGHT: UINT = 620;

pub const IPC_GETLANGUAGEPACKINSTANCE: UINT = 621;

pub const LANG_IDENT_STR: UINT = 0;
pub const LANG_LANG_CODE: UINT = 1;
pub const LANG_COUNTRY_CODE: UINT = 2;

pub const IPC_CB_PEINFOTEXT: UINT = 622;

pub const IPC_CB_OUTPUTCHANGED: UINT = 623;

pub const IPC_GETOUTPUTPLUGIN: UINT = 625;

pub const IPC_SETDRAWBORDERS: UINT = 626;

pub const IPC_DISABLESKINCURSORS: UINT = 627;

pub const IPC_GETSKINCURSORS: UINT = 628;

pub const IPC_CB_RESETFONT: UINT = 629;

pub const IPC_IS_FULLSCREEN: UINT = 630;

pub const IPC_SET_VIS_FS_FLAG: UINT = 631;

pub const IPC_SHOW_NOTIFICATION: UINT = 632;

pub const IPC_GETSKININFO: UINT = 633;
pub const IPC_GETSKININFOW: UINT = 1633;

pub const IPC_GET_MANUALPLADVANCE: UINT = 634;

pub const IPC_SET_MANUALPLADVANCE: UINT = 635;

pub const IPC_GET_NEXT_PLITEM: UINT = 636;

pub const IPC_GET_PREVIOUS_PLITEM: UINT = 637;

pub const IPC_IS_WNDSHADE: UINT = 638;

pub const IPC_SETRATING: UINT = 639;

pub const IPC_GETRATING: UINT = 640;

pub const IPC_GETNUMAUDIOTRACKS: UINT = 641;

pub const IPC_GETNUMVIDEOTRACKS: UINT = 642;

pub const IPC_GETAUDIOTRACK: UINT = 643;

pub const IPC_GETVIDEOTRACK: UINT = 644;

pub const IPC_SETAUDIOTRACK: UINT = 645;

pub const IPC_SETVIDEOTRACK: UINT = 646;

pub const IPC_PUSH_DISABLE_EXIT: UINT = 647;

pub const IPC_POP_DISABLE_EXIT: UINT = 648;

pub const IPC_IS_EXIT_ENABLED: UINT = 649;

pub const IPC_IS_AOT: UINT = 650;

pub const IPC_USES_RECYCLEBIN: UINT = 651;

pub const IPC_FLUSHAUDITS: UINT = 652;

pub const IPC_GETPLAYITEM_START: UINT = 653;
pub const IPC_GETPLAYITEM_END: UINT = 654;

pub const IPC_GETVIDEORESIZE: UINT = 655;
pub const IPC_SETVIDEORESIZE: UINT = 656;

pub const IPC_INITIAL_SHOW_STATE: UINT = 657;

pub const IPC_PLCMD: UINT = 1000;

pub const PLCMD_ADD: UINT = 0;
pub const PLCMD_REM: UINT = 1;
pub const PLCMD_SEL: UINT = 2;
pub const PLCMD_MISC: UINT = 3;
pub const PLCMD_LIST: UINT = 4;

pub const MBCMD_BACK: UINT = 0;
pub const MBCMD_FORWARD: UINT = 1;
pub const MBCMD_STOP: UINT = 2;
pub const MBCMD_RELOAD: UINT = 3;
pub const MBCMD_MISC: UINT = 4;

pub const IPC_VIDCMD: UINT = 1002;

pub const VIDCMD_FULLSCREEN: UINT = 0;
pub const VIDCMD_1X: UINT = 1;
pub const VIDCMD_2X: UINT = 2;
pub const VIDCMD_LIB: UINT = 3;
pub const VIDPOPUP_MISC: UINT = 4;

pub const IPC_STATS_LIBRARY_ITEMCNT: UINT = 1300;

pub const IPC_FF_FIRST: UINT = 2000;
pub const IPC_FF_COLOURTHEME_CHANGE: UINT = IPC_FF_ONCOLORTHEMECHANGED;
pub const IPC_FF_ONCOLORTHEMECHANGED: UINT = IPC_FF_FIRST + 3;
pub const IPC_FF_ISMAINWND: UINT = IPC_FF_FIRST + 4;
pub const IPC_FF_GETCONTENTWND: UINT = IPC_FF_FIRST + 5;
pub const IPC_FF_NOTIFYHOTKEY: UINT = IPC_FF_FIRST + 6;

pub const IPC_FF_LAST: UINT = 3000;

pub const IPC_GETDROPTARGET: UINT = 3001;

pub const IPC_PLAYLIST_MODIFIED: UINT = 3002;

pub const IPC_PLAYING_FILE: UINT = 3003;

pub const IPC_PLAYING_FILEW: UINT = 13003;

pub const IPC_FILE_TAG_MAY_HAVE_UPDATED: UINT = 3004;
pub const IPC_FILE_TAG_MAY_HAVE_UPDATEDW: UINT = 3005;

pub const IPC_ALLOW_PLAYTRACKING: UINT = 3007;

pub const IPC_HOOK_OKTOQUIT: UINT = 3010;

pub const IPC_WRITECONFIG: UINT = 3011;

pub const IPC_UPDATE_URL: UINT = 3012;

pub const IPC_GET_RANDFUNC: UINT = 3015;

pub const IPC_METADATA_CHANGED: UINT = 3017;

pub const IPC_SKIN_CHANGED: UINT = 3018;

pub const IPC_REGISTER_LOWORD_COMMAND: UINT = 3019;

pub const IPC_GET_DISPATCH_OBJECT: UINT = 3020;
pub const IPC_GET_UNIQUE_DISPATCH_ID: UINT = 3021;
pub const IPC_ADD_DISPATCH_OBJECT: UINT = 3022;

#[repr(C)]
pub struct DispatchInfo {
    pub name: *mut wchar_t,
    pub dispatch: *mut (),
    pub id: DWORD,
}
pub const IPC_GET_PROXY_STRING: UINT = 3023;

pub const IPC_USE_REGISTRY: UINT = 3024;

pub const IPC_GET_API_SERVICE: UINT = 3025;

#[repr(C)]
pub struct extendedFileInfoStructW {
    pub filename: *const wchar_t,
    pub metadata: *const wchar_t,
    pub ret: *mut wchar_t,
    pub retlen: size_t,
}

pub const IPC_GET_EXTENDED_FILE_INFOW: UINT = 3026;

pub const IPC_GET_EXTENDED_FILE_INFOW_HOOKABLE: UINT = 3027;
pub const IPC_SET_EXTENDED_FILE_INFOW: UINT = 3028;

pub const IPC_PLAYLIST_GET_NEXT_SELECTED: UINT = 3029;

pub const IPC_PLAYLIST_GET_SELECTED_COUNT: UINT = 3030;

pub const IPC_GET_PLAYING_FILENAME: UINT = 3031;

pub const IPC_OPEN_URL: UINT = 3032;

pub const IPC_USE_UXTHEME_FUNC: UINT = 3033;

pub const IPC_ISWINTHEMEPRESENT: UINT = 0;

pub const IPC_ISAEROCOMPOSITIONACTIVE: UINT = 1;

pub const IPC_GET_PLAYING_TITLE: UINT = 3034;

pub const IPC_CANPLAY: UINT = 3035;

#[repr(C)]
pub struct artFetchData {
    pub size: size_t,
    pub parent: HWND,
    pub artist: *const wchar_t,
    pub album: *const wchar_t,
    pub year: c_int,
    pub amgArtistId: c_int,
    pub amgAlbumId: c_int,
    pub showCancelAll: c_int,
    pub imgData: *mut c_void,
    pub imgDataLen: c_int,
    pub r#type: [wchar_t; 10],
    pub gracenoteFileId: *const wchar_t,
}

pub const IPC_FETCH_ALBUMART: UINT = 3036;

pub const IPC_JSAPI2_GET_DISPATCH_OBJECT: UINT = 3037;

pub const IPC_REGISTER_WINAMP_IPCMESSAGE: UINT = 65536;
