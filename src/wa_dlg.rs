use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use libc::*;

pub const DCW_SUNKENBORDER: UINT = 0x00010000;
pub const DCW_DIVIDER: UINT = 0x00020000;

#[repr(C)]
pub enum WADLG {
    ITEMBG,
    ITEMFG,
    WNDBG,
    BUTTONFG,
    WNDFG,
    HILITE,
    SELCOLOR,
    LISTHEADER_BGCOLOR,
    LISTHEADER_FONTCOLOR,
    LISTHEADER_FRAME_TOPCOLOR,
    LISTHEADER_FRAME_MIDDLECOLOR,
    LISTHEADER_FRAME_BOTTOMCOLOR,
    LISTHEADER_EMPTY_BGCOLOR,
    SCROLLBAR_FGCOLOR,
    SCROLLBAR_BGCOLOR,
    SCROLLBAR_INV_FGCOLOR,
    SCROLLBAR_INV_BGCOLOR,
    SCROLLBAR_DEADAREA_COLOR,
    SELBAR_FGCOLOR,
    SELBAR_BGCOLOR,
    INACT_SELBAR_FGCOLOR,
    INACT_SELBAR_BGCOLOR,
    ITEMBG2,
    ITEMFG2,
    NUM_COLORS
}

#[repr(C)]
pub enum WACURSOR {
    VOLUME = 0,
    POSITION = 1,
    BTN_WINSHADE = 2,	// winshade
    BTN_MINIMIZE = 3,	// minimize
    BTN_CLOSE = 4,		// close
    MENU = 	5,			// main menu
    TITLEBAR = 6,		// title bar
    SONGNAME = 7,		
    NORMAL = 8,
    WINSHADE_BTN_WINSHADE = 9,
    WINSHADE_BTN_MINIMIZE = 10,
    WINSHADE_POSITION = 11,
    WINSHADE_BTN_CLOSE = 12,
    WINSHADE_MENU = 13,
    WINSHADE_NORMAL = 14,
    PL_BTN_WINSHADE = 15,
    PL_BTN_CLOSE = 16,
    PL_TITLEBAR = 17,
    PL_VSCROLL = 18,
    PL_RESIZE = 19,
    PL_NORMAL = 20,
    PL_WINSHADE_BTN_WINSHADE = 21,
    PL_WINSHADE_BTN_CLOSE = 22,
    PL_WINSHADE_HSIZE = 23,
    PL_WINSHADE_NORMAL = 24,
    EQ_SLIDER = 25,
    EQ_BTN_CLOSE = 26,
    EQ_TITLEBAR = 27,
    EQ_NORMAL = 28,
}

extern "C" {
    pub fn WADlg_init(hwndWinamp: HWND);
    pub fn WADlg_close();
    pub fn WADlg_getColor(idx: c_int) -> c_int;
    pub fn WADlg_initted() -> c_int;

    pub fn WADlg_handleDialogMsgs(hwndDlg: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn WADlg_DrawChildWindowBorders(hwndDl: HWND, tab: *mut c_int, tabsize: c_int);
    pub fn WADlg_getBitmap() -> HBITMAP;
}

// todo WA_DLG_IMPLEMENT
