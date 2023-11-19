use def::wchar_t;
use plugindata::getNppHandle;
use std::process::Command;
use winapi::shared::minwindef::{MAX_PATH, UINT};
use winapi::um::winuser;

const NPPM_GETCURRENTLANGTYPE: UINT = winuser::WM_USER + 1005;
const NPPM_SAVEALLFILES: UINT = winuser::WM_USER + 1039;
const NPPM_GETFULLCURRENTPATH: UINT = winuser::WM_USER + 3001;
const NPPM_SETMENUITEMCHECK: UINT = winuser::WM_USER + 1040;

pub fn launchCmdWindow(command: String) {
    Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg("cmd")
        .arg("/c")
        .arg(command + " & pause")
        .spawn()
        .unwrap();
}

macro_rules! SendNpp {
    ($param1:ident, $param2:expr, $param3:expr) => {
        unsafe {
            winuser::SendMessageW(
                getNppHandle()._nppHandle,
                $param1,
                $param2 as usize,
                $param3 as isize,
            );
        };
    };
}

pub fn check_doctype() -> bool {
    // save all files
    SendNpp!(NPPM_SAVEALLFILES, 0, 0);
    const L_EXTERNAL: i32 = 86;
    const L_RUST: i32 = 81;
    let ftype = L_EXTERNAL;
    // get lang type
    SendNpp!(NPPM_GETCURRENTLANGTYPE, 0, &ftype as *const i32);
    ftype == L_RUST
}

pub fn getCurrentNppFile() -> String {
    let fPath: Vec<wchar_t> = vec![0; MAX_PATH];
    SendNpp!(NPPM_GETFULLCURRENTPATH, MAX_PATH, fPath.as_ptr());
    String::from_utf16_lossy(fPath.as_slice())
}

pub fn checkMenuItem(id: usize, state: u8) {
    SendNpp!(NPPM_SETMENUITEMCHECK, crate::FUNC_ITEMS[id]._cmdID, state);
}
