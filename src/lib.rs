#![crate_type = "cdylib"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate core;
extern crate libc;
extern crate winapi;

use def::{to_wide_chars, wchar_t, FuncItem, NppData, ShortcutKey};
use winapi::shared::minwindef;
use winapi::um::winuser::VK_F10;

mod def;
mod functions;
mod helpers;
mod plugindata;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PROG_NAME: Vec<wchar_t> = to_wide_chars("Rust plugin");
    pub static ref FUNC_ITEMS: Vec<FuncItem> = vec![
        FuncItem::new(
            "Run",
            functions::runProgram,
            &SHORT_KEY_F10 as *const ShortcutKey as usize,
        ),
        FuncItem::new(
            "Build",
            functions::buildProgram,
            &SHORT_KEY_CTRL_F10 as *const ShortcutKey as usize,
        ),
        FuncItem::new("Format", functions::fmtProgram, 0),
        FuncItem::new("Clippy", functions::runCargoClippy, 0),
        FuncItem::new("Test", functions::runCargoTest, 0),
        FuncItem::new("Switch Dev\\Release target", functions::switchConfig, 0),
        FuncItem::new("About", functions::runAboutDlg, 0),
    ];
}

#[no_mangle]
pub extern "C" fn isUnicode() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn setInfo(notpadPlusData: NppData) {
    unsafe {
        plugindata::NPP_DATA = Some(notpadPlusData);
    }
}

#[no_mangle]
pub extern "C" fn getName() -> *const wchar_t {
    PROG_NAME.as_ptr()
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn getFuncsArray(nbF: *mut usize) -> *const FuncItem {
    unsafe { *nbF = FUNC_ITEMS.len() };
    FUNC_ITEMS.as_ptr()
}

#[no_mangle]
pub extern "C" fn beNotified(_notifyCode: *mut libc::c_void) {}

#[no_mangle]
pub extern "C" fn messageProc(
    _Message: minwindef::UINT,
    _wParam: minwindef::WPARAM,
    _lParam: minwindef::LPARAM,
) -> minwindef::LRESULT {
    /*
        if (Message == WM_MOVE)
        {
            ::MessageBox(NULL, "move", "", MB_OK);
        }
    */
    minwindef::TRUE as minwindef::LRESULT
}

static SHORT_KEY_F10: ShortcutKey = ShortcutKey {
    _isCtrl: false,
    _isAlt: false,
    _isShift: false,
    _key: VK_F10 as u8,
};

static SHORT_KEY_CTRL_F10: ShortcutKey = ShortcutKey {
    _isCtrl: true,
    _isAlt: false,
    _isShift: false,
    _key: VK_F10 as u8,
};
