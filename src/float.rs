#[unsafe(export_name = "_fltused")]
static FLT_USED: u8 = 0;

#[inline]
pub fn init_x87_cw() {
    cfg_select! {
        all(target_arch = "x86", not(target_feature = "sse2")) => {
            // All exceptions masked, 53-bit (double) precision, round to nearest even.
            static CW: u16 = 0x027F;
            // SAFETY: `fldcw` reads exactly the two bytes pointed at by `cw`.
            unsafe { core::arch::asm!("fldcw [{}]", in(reg) &CW, options(nostack, readonly)) };
        }
        _ => {}
    }
}
