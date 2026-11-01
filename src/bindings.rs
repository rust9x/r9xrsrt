#![allow(nonstandard_style, clippy::all)]

use super::windows_link;

windows_link::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : u32) -> !);
pub const DLL_PROCESS_ATTACH: i32 = 1;
pub const DLL_PROCESS_DETACH: i32 = 0;
pub type HANDLE = *mut core::ffi::c_void;
pub type PIMAGE_TLS_CALLBACK = Option<
    unsafe extern "system" fn(
        dllhandle: *mut core::ffi::c_void,
        reason: u32,
        reserved: *mut core::ffi::c_void,
    ),
>;
