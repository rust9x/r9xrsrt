use crate::{
    bindings::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, ExitProcess, HANDLE},
    static_init::run_static_init,
};
use core::{
    ffi::{c_int, c_void},
    ptr,
};
use windows_sys::core::BOOL;

unsafe extern "C" {
    pub unsafe fn main(_argc: isize, _argv: *const *const u8, _envp: *const *const u8) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainCRTStartup() -> c_int {
    unsafe {
        run_static_init();
        // rust's main doesn't use the arguments on windows
        let ret = main(0, ptr::null(), ptr::null());
        ExitProcess(ret as u32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMainCRTStartup(
    _dll_handle: HANDLE,
    reason: u32,
    _reserved: *mut c_void,
) -> BOOL {
    unsafe {
        match reason {
            DLL_PROCESS_ATTACH => {
                run_static_init();
            }
            DLL_PROCESS_DETACH => {}
            _ => {}
        }
    }

    true as BOOL
}
