use helpers;
use plugindata::getNppHandle;
use std::env;
use std::path::Path;
use winapi::shared::minwindef;
use winapi::um::winuser;

const NPPM_GETCURRENTLANGTYPE: minwindef::UINT = winuser::WM_USER + 1005;

fn cargoProject(currentFile: &String, cmd: String) -> bool {
    let mut p = Some(Path::new(&currentFile));
    while p.unwrap().parent().is_some() {
        if p.unwrap().file_name().unwrap() == "src" {
            let par = p.unwrap().parent().unwrap();
            if par.join("Cargo.toml").is_file() {
                let cwd = env::current_dir();
                env::set_current_dir(par).unwrap();
                helpers::launchCmdWindow(format!("cargo {}", cmd));
                env::set_current_dir(cwd.unwrap()).unwrap();
                return true;
            }
        } else {
            p = p.unwrap().parent();
        }
    }
    false
}

fn rustFile(currentFile: &String, run: bool) -> bool {
    let p = Path::new(&currentFile);
    let fname = p.file_name().unwrap();
    let fstem = p.file_stem().unwrap();
    let par = p.parent().unwrap();
    let cwd = env::current_dir();

    let mut cmd: String = String::new();
    if run {
        cmd = format!(" & {}", fstem.to_str().unwrap());
    }
    env::set_current_dir(par).unwrap();
    helpers::launchCmdWindow(format!("rustc {} {}", fname.to_str().unwrap(), cmd));
    env::set_current_dir(cwd.unwrap()).unwrap();

    true
}

fn check_rust() -> bool {
    const L_EXTERNAL: i32 = 86;
    const L_RUST: i32 = 81;
    let ftype = L_EXTERNAL;
    unsafe {
        winuser::SendMessageW(
            getNppHandle()._nppHandle,
            NPPM_GETCURRENTLANGTYPE,
            0,
            &ftype as *const i32 as isize,
        )
    };
    ftype == L_RUST
}

macro_rules! action {
    ($cmd:literal, $run:expr) => {
        if check_rust() {
            let s = helpers::getCurrentNppFile();
            if !cargoProject(&s, String::from($cmd)) {
                rustFile(&s, $run);
            }
        }
    };
}

//  exposed functions

pub extern "C" fn runProgram() {
    action!("run", true);
}

pub extern "C" fn buildProgram() {
    action!("build", false);
}
