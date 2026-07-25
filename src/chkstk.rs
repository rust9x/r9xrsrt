// Adapted from rust/library/compiler-builtins/compiler-builtins/src/x86.rs and tweaked to prefer
// _chkstk

cfg_select! {
    target_arch = "x86" => {
        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        pub unsafe extern "custom" fn alloca() {
            core::arch::naked_asm!(
                "jmp {}", // Jump to __chkstk since fallthrough may be unreliable"
                sym _chkstk,
            );
        }

        #[unsafe(naked)]
        #[unsafe(no_mangle)]
        pub unsafe extern "custom" fn _chkstk() {
            // __chkstk and _alloca are the same function
            core::arch::naked_asm!(
                "push   ecx",
                "cmp    eax, 0x1000",
                "lea    ecx, [esp + 8]", // esp before calling this routine -> ecx
                "jb     3f",
                "2:",
                "sub    ecx, 0x1000",
                "test   [ecx], ecx",
                "sub    eax, 0x1000",
                "cmp    eax, 0x1000",
                "ja     2b",
                "3:",
                "sub    ecx, eax",
                "test   [ecx], ecx",
                "lea    eax, [esp + 4]", // load pointer to the return address into eax
                "mov    esp, ecx",       // install the new top of stack pointer into esp
                "mov    ecx, [eax - 4]", // restore ecx
                "push   [eax]",          // push return address onto the stack
                "sub    eax, esp",       // restore the original value in eax
                "ret",
            );
        }
    }
    target_arch = "x86_64" => {
        compiler_error!("x86_64 is not supported yet");
    }
}
