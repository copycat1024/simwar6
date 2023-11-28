
use std::{
    ffi::{c_char, c_void, CString},
    mem::transmute_copy,
};

pub fn load<FnLoad, FnRet>(loader: &FnLoad, name: &str) -> FnRet
where
    FnLoad: Fn(*const c_char) -> *mut c_void,
{
    let c_name = CString::new(name).unwrap();
    let p = match loader(c_name.as_ptr()) as usize {
        0 | 1 | 2 | 3 | usize::MAX => panic!("{}", name),
        p => p as *mut c_void,
    };
    unsafe { transmute_copy(&p) }
}
