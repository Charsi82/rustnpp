#![crate_type = "cdylib"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate core;
extern crate libc;
extern crate winapi;

use def::{to_wide_chars, wchar_t, FuncItem, NppData};
use winapi::shared::minwindef;

mod def;
mod functions;
mod helpers;
mod plugindata;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PROG_NAME: Vec<wchar_t> = to_wide_chars("Rust plugin");
    static ref FUNC_ITEMS: Vec<FuncItem> =
        vec![plugindata::FuncItem_Run(), plugindata::FuncItem_Build(),];
}

#[no_mangle]
pub extern "C" fn isUnicode() -> bool {
    true
}

#[allow(unused_variables)]
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
    //unsafe { *nbF = std::mem::transmute( FUNC_ITEMS.len() ) };
    unsafe { *nbF = FUNC_ITEMS.len() };
    FUNC_ITEMS.as_ptr()
}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn beNotified(notifyCode: *mut libc::c_void) {}

#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn messageProc(
    Message: minwindef::UINT,
    wParam: minwindef::WPARAM,
    lParam: minwindef::LPARAM,
) -> minwindef::LRESULT {
    /*
        if (Message == WM_MOVE)
        {
            ::MessageBox(NULL, "move", "", MB_OK);
        }
    */
    minwindef::TRUE as minwindef::LRESULT
}
