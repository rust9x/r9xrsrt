//! Minimal implementations of mem functions.
//!
//! It's recommended to compile the stdlib with `-Zbuild-std-features=compiler-builtins-mem`, then
//! the compiler will use its optimized builtins instead of these functions.
//!
//! These here only exist as fallback.

use core::ffi::c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, len: usize) -> *mut c_void {
    unsafe { memcpy_inner(dest, src, len) };
    dest
}

#[inline(always)]
unsafe fn memcpy_inner(dest: *mut c_void, src: *const c_void, len: usize) {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "rep movsb",
            inout("rdi") dest => _,
            inout("rsi") src => _,
            inout("rcx") len => _,
            options(nostack, preserves_flags)
        );
        #[cfg(target_arch = "x86")]
        core::arch::asm!(
            "xchg esi, {src}",
            "rep movsb",
            "xchg esi, {src}",
            src = inout(reg) src => _, // LLVM doesn't allow using `inout("esi")`
            inout("edi") dest => _,
            inout("ecx") len => _,
            options(nostack, preserves_flags),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut c_void, src: *const c_void, len: usize) -> *mut c_void {
    unsafe {
        if (dest as usize) < (src as usize) {
            memcpy_inner(dest, src, len)
        } else {
            // dest above src: copy backward to handle overlap.
            #[cfg(target_arch = "x86_64")]
            core::arch::asm!(
                "std",
                "rep movsb",
                "cld",
                inout("rdi") dest.cast::<u8>().add(len).wrapping_sub(1) => _,
                inout("rsi") src.cast::<u8>().add(len).wrapping_sub(1) => _,
                inout("rcx") len => _,
                options(nostack, preserves_flags)
            );
            #[cfg(target_arch = "x86")]
            core::arch::asm!(
                "xchg esi, {src}",
                "std",
                "rep movsb",
                "cld",
                "xchg esi, {src}",
                src = inout(reg) src.cast::<u8>().add(len).wrapping_sub(1) => _,
                inout("edi") dest.cast::<u8>().add(len).wrapping_sub(1) => _,
                inout("ecx") len => _,
                options(nostack, preserves_flags),
            );
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut c_void, val: i32, len: usize) -> *mut c_void {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "rep stosb",
            inout("rdi") dest => _,
            in("al") val as u8,
            inout("rcx") len => _,
            options(nostack, preserves_flags)
        );
        #[cfg(target_arch = "x86")]
        core::arch::asm!(
            "rep stosb",
            inout("edi") dest => _,
            in("al") val as u8,
            inout("ecx") len => _,
            options(nostack, preserves_flags)
        );
    }
    dest
}
