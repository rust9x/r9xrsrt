#[unsafe(link_section = ".CRT$XCA")]
#[used]
pub static CRT_STATIC_INIT_START: Option<unsafe extern "C" fn()> = None;

// rust (and cpp) static inits will be placed between here

#[unsafe(link_section = ".CRT$XCZ")]
#[used]
pub static CRT_STATIC_INIT_END: u8 = 0;

pub(crate) unsafe fn run_static_init() {
    unsafe {
        let mut current = (&raw const CRT_STATIC_INIT_START).offset(1);
        let end = (&raw const CRT_STATIC_INIT_END).cast();
        while current < end {
            if let Some(init_fn) = *current {
                init_fn();
            }
            current = current.add(1);
        }
    }
}
