use std::os::raw::c_int;

extern "C" {
    fn cv_imshow() -> bool;
    fn cv_waitKey() -> c_int;
}

pub fn imshow() -> bool {
    unsafe { cv_imshow() }
}

pub fn wait_key() -> Option<u8> {
    let code = unsafe { cv_waitKey() };
    if code >= 0 { Some(code as _) } else { None }
}
