#![cfg(target_arch = "x86")] // not needed on x86_64 of course

//! 64-bit integer division and remainder for x86
//!
//! These use a non-standard ABI: The functions follow `stdcall`, but the symbol names carry no `@N`
//! suffix.
//!
//! `no_mangle`/`export_name` doesn't suppress stdcall decoration, so `extern "stdcall"` alone can't
//! produce the symbol the linker looks for. The ABI it generates *is* correct, so a naked shim
//! under the undecorated name only has to jump to it.

use core::hint::unreachable_unchecked;

/// `unsigned __int64 _aulldiv(unsigned __int64 dividend, unsigned __int64 divisor)`
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "custom" fn _aulldiv() {
    // Same ABI, only a different symbol name; fallthrough may be unreliable, so jump.
    core::arch::naked_asm!("jmp {}", sym aulldiv_stdcall);
}

/// 64-by-64-bit unsigned division, using only 32-bit operations.
extern "stdcall" fn aulldiv_stdcall(a: u64, b: u64) -> u64 {
    let (a_hi, b_lo, b_hi) = ((a >> 32) as u32, b as u32, (b >> 32) as u32);

    if b_hi == 0 {
        // Single-word divisor: two chained 64-by-32-bit divisions. `a_hi < b_lo` holds for the
        // second one because the first one reduced it to a remainder.
        let (q_hi, rem) = unsafe { divmod_64_32(0, a_hi, b_lo) };
        let (q_lo, _) = unsafe { divmod_64_32(rem, a as u32, b_lo) };
        return ((q_hi as u64) << 32) | q_lo as u64;
    }

    // SAFETY: high word == 0 is checked above.
    unsafe { divmod_wide(a, b).0 as u64 }
}

/// `unsigned __int64 _aullrem(unsigned __int64 dividend, unsigned __int64 divisor)`
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "custom" fn _aullrem() {
    core::arch::naked_asm!("jmp {}", sym aullrem_stdcall);
}

/// 64-by-64-bit unsigned remainder. See [`_aulldiv_stdcall`].
extern "stdcall" fn aullrem_stdcall(a: u64, b: u64) -> u64 {
    let (a_hi, b_lo, b_hi) = ((a >> 32) as u32, b as u32, (b >> 32) as u32);

    if b_hi == 0 {
        // Only the remainder of the second division matters; the first one exists to feed it the
        // high word's remainder.
        let (_, rem) = unsafe { divmod_64_32(0, a_hi, b_lo) };
        let (_, rem) = unsafe { divmod_64_32(rem, a as u32, b_lo) };
        return rem as u64;
    }

    // `a - q * b`, where the product is exact once the estimate has been corrected.
    // SAFETY: high word == 0 is checked above.
    unsafe { a - divmod_wide(a, b).1 }
}

/// 64-by-64-bit unsigned division for a divisor that needs both words, returning the quotient
/// (which always fits in 32 bits here) and the exact product `quotient * divisor`.
///
/// # Safety
///
/// The high word of `b` must be non-zero.
unsafe fn divmod_wide(a: u64, b: u64) -> (u32, u64) {
    let (a_lo, a_hi) = (a as u32, (a >> 32) as u32);
    let (b_lo, b_hi) = (b as u32, (b >> 32) as u32);

    if b_hi == 0 {
        // SAFETY: guaranteed by the caller
        unsafe { unreachable_unchecked() }
    }

    // Normalize by shifting both operands right until the divisor is a single word, then divide;
    // that yields the quotient, possibly one too large because of the bits that were shifted out.
    let shift = 32 - b_hi.leading_zeros();
    let a_norm = a >> shift;
    let b_norm = (b >> shift) as u32;

    let (mut q, _) = unsafe { divmod_64_32((a_norm >> 32) as u32, a_norm as u32, b_norm) };

    // Correct the estimate: if `q * b` (truncated to 64 bits, with the carry out of the high word
    // tracked separately) exceeds the dividend, `q` was one too large.
    let carry_in = (q as u64 * b_hi as u64) as u32;
    let prod = q as u64 * b_lo as u64;
    let (prod_hi, overflow) = ((prod >> 32) as u32).overflowing_add(carry_in);
    let mut prod = ((prod_hi as u64) << 32) | prod as u32 as u64;

    if overflow || prod_hi > a_hi || (prod_hi == a_hi && prod as u32 > a_lo) {
        q -= 1;
        prod = prod.wrapping_sub(b);
    }

    (q, prod)
}

/// Divides `hi:lo` by `d`, returning `(quotient, remainder)`.
///
/// # Safety
///
/// `d` must be non-zero and `hi < d`, otherwise the quotient doesn't fit in 32 bits and the `div`
/// raises `#DE`.
#[inline]
unsafe fn divmod_64_32(hi: u32, lo: u32, d: u32) -> (u32, u32) {
    let (quot, rem);
    unsafe {
        core::arch::asm!(
            "div {d:e}",
            d = in(reg) d,
            inout("eax") lo => quot,
            inout("edx") hi => rem,
            options(nomem, nostack),
        );
    }
    (quot, rem)
}
