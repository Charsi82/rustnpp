extern crate libc;
extern crate winapi;

use winapi::shared::{ntdef::UCHAR, windef};
const ITEM_NAME_MAX_SIZE:usize = 64;
pub type wchar_t = winapi::ctypes::wchar_t;

#[repr(C)]
pub struct NppData {
    pub _nppHandle: windef::HWND,
    pub _scintillaMainHandle: windef::HWND,
    pub _scintillaSecondHandle: windef::HWND,
}

#[repr(C)]
pub struct ShortcutKey {
    pub _isCtrl: bool,
    pub _isAlt: bool,
    pub _isShift: bool,
    pub _key: UCHAR,
}

#[repr(C)]
pub struct FuncItem {
    pub _itemName: [wchar_t; ITEM_NAME_MAX_SIZE],
    pub _pFunc: extern "C" fn(),
    pub _cmdID: i32,
    pub _init2Check: bool,
    pub _pShKey: usize,
}

impl FuncItem {
    pub fn new(caption: &str, func: extern "C" fn(), pSKey: usize) -> FuncItem {
        FuncItem {
            _itemName: function_item_text(caption),
            _pFunc: func,
            _cmdID: 0,
            _init2Check: false,
            _pShKey: pSKey,
        }
    }
}

pub fn to_wide_chars(s: &str) -> Vec<wchar_t> {
    let mut v: Vec<wchar_t> = s.encode_utf16().collect();
    v.push(0);
    v
}

pub fn function_item_text(s: &str) -> [wchar_t; ITEM_NAME_MAX_SIZE] {
    let mut arr: [wchar_t; ITEM_NAME_MAX_SIZE] = [0; ITEM_NAME_MAX_SIZE];
    let vecStr = to_wide_chars(s);
    for (i, ch) in vecStr.iter().enumerate() {
        if i < ITEM_NAME_MAX_SIZE {
            arr[i] = *ch
        };
    }
    arr
}
