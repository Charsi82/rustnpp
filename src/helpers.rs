use def::{from_wide_ptr, wchar_t};
use plugindata::getNppHandle;
use std::process::Command;
use winapi::shared::minwindef;
use winapi::um::winuser;

const NPPM_GETFULLCURRENTPATH: minwindef::UINT = winuser::WM_USER + 3001;

pub fn launchCmdWindow(command: String) {
    Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg("cmd")
        .arg("/c")
        .arg(format!("{} & pause ", command))
        .spawn()
        .unwrap();
}

#[allow(unused_mut)]
pub fn getCurrentNppFile() -> String {
    let mut fPath: Vec<wchar_t> = vec![0; minwindef::MAX_PATH];
    unsafe {
        winuser::SendMessageW(
            getNppHandle()._nppHandle,
            NPPM_GETFULLCURRENTPATH,
            0,
            fPath.as_ptr() as isize,
        );
    }
    from_wide_ptr(fPath.as_ptr())
}
