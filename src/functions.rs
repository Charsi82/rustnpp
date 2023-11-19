use helpers::{checkMenuItem, check_doctype, getCurrentNppFile, launchCmdWindow};
use std::env;
use std::path::Path;
use winapi::um::winuser::{MessageBoxW, MB_OK};

use crate::def::to_wide_chars;

static mut CFG_STATE: u8 = 0;

// check is cargo project or single file
fn cargoProject(currentFilePath: &Path) -> Option<&str> {
    let mut p = Some(currentFilePath);
    while p.unwrap().parent().is_some() {
        if p.unwrap().file_name().unwrap() == "src" {
            let par = p.unwrap().parent().unwrap();
            if par.join("Cargo.toml").is_file() {
                return par.to_str();
            }
        } else {
            p = p.unwrap().parent();
        }
    }
    None
}

fn runCmd(path: &Path, cmd: String) {
    let cwd = env::current_dir();
    let _ = env::set_current_dir(path);
    launchCmdWindow(cmd);
    _ = env::set_current_dir(cwd.unwrap());
}

fn action(f1: &str, f2: impl Fn(&Path) -> String) {
    if check_doctype() {
        let s = getCurrentNppFile();
        let p = Path::new(&s);
        if let Some(path) = cargoProject(p) {
            runCmd(Path::new(&path), f1.to_string());
        } else {
            runCmd(p.parent().unwrap(), f2(p));
        }
    }
}

fn getConfig() -> &'static str {
    unsafe {
        if CFG_STATE == 1 {
            "--release"
        } else {
            ""
        }
    }
}
//  exposed functions

pub extern "C" fn runProgram() {
    action(format!("cargo run {}", getConfig()).as_str(), |p: &Path| {
        let fname = p.file_name().unwrap();
        let fstem = p.file_stem().unwrap();
        format!(
            "(if exist {1}.exe (del {1}.exe)) & rustc {} & (if exist {1}.exe ({1}))",
            fname.to_str().unwrap(),
            fstem.to_str().unwrap()
        )
    })
}

pub extern "C" fn buildProgram() {
    action(
        format!("cargo build {}", getConfig()).as_str(),
        |p: &Path| {
            let fname = p.file_name().unwrap();
            format!("echo build... & rustc {}", fname.to_str().unwrap())
        },
    )
}

pub extern "C" fn fmtProgram() {
    action("echo format... & cargo fmt", |p: &Path| {
        let fname = p.file_name().unwrap();
        format!("echo format... & rustfmt {}", fname.to_str().unwrap())
    })
}

pub extern "C" fn runCargoClippy() {
    if check_doctype() {
        let s = getCurrentNppFile();
        let p = Path::new(&s);
        if let Some(path) = cargoProject(p) {
            runCmd(
                Path::new(&path),
                format!("echo clippy... & cargo clippy {}", getConfig()),
            );
        }
    }
}


pub extern "C" fn runCargoTest() {
    if check_doctype() {
        let s = getCurrentNppFile();
        let p = Path::new(&s);
        if let Some(path) = cargoProject(p) {
            runCmd(
                Path::new(&path),
                format!("echo test... & cargo test {}", getConfig()),
            );
        }
    }
}

pub extern "C" fn switchConfig() {
    unsafe {
        CFG_STATE = 1 - CFG_STATE;
        checkMenuItem(5 /* Switch Dev\\Release MenuItem */, CFG_STATE);
    };
}

pub extern "C" fn runAboutDlg() {
    unsafe {
        let text = format!(
            "{:>34}\n{:>38}\n{:>35}\n\n{:>35}\n\n{:>34}",
            "Authors:",
            "ნიკა <nika.begiashvili@gmail.com>",
            "Charsi <charsi2011@gmail.com>",
            "Version: 1.3.0",
            "License: GPL"
        );
        MessageBoxW(
            std::ptr::null_mut(),
            to_wide_chars(text.as_str()).as_ptr(),
            to_wide_chars("Rust plugin for Notepad++").as_ptr(),
            MB_OK,
        );
    }
}
