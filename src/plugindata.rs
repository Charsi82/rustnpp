extern crate libc;
extern crate winapi;

use def::NppData;

pub static mut NPP_DATA: Option<NppData> = None;

pub fn getNppHandle() -> &'static mut NppData {
    unsafe {
        match NPP_DATA {
            Some(ref mut x) => &mut *x,
            None => panic!(),
        }
    }
}
