use crate::bindings::PIMAGE_TLS_CALLBACK;

static mut TLS_INDEX: u32 = 0;

#[unsafe(link_section = ".tls")]
#[unsafe(export_name = "_tls_start")]
pub static mut TLS_START: u8 = 0;
#[unsafe(link_section = ".tls$ZZZ")]
#[unsafe(export_name = "_tls_end")]
pub static mut TLS_END: u8 = 0;

#[unsafe(link_section = ".CRT$XLA")]
pub static mut CRT_TLS_CALLBACK_START: PIMAGE_TLS_CALLBACK = None;

// callbacks will be placed in between here

// nullptr so that the loader knows when to stop calling callbacks
#[unsafe(link_section = ".CRT$XLZ")]
#[used]
pub static mut CRT_TLS_CALLBACK_END: PIMAGE_TLS_CALLBACK = None;

cfg_select! {
    target_arch = "x86" => {
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct UnsafePtr(*mut ());
        unsafe impl Send for UnsafePtr {}
        unsafe impl Sync for UnsafePtr {}

        #[allow(non_camel_case_types, non_snake_case)]
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct IMAGE_TLS_DIRECTORY32 {
            pub StartAddressOfRawData: UnsafePtr,
            pub EndAddressOfRawData: UnsafePtr,
            pub AddressOfIndex: UnsafePtr,
            pub AddressOfCallBacks: UnsafePtr,
            pub SizeOfZeroFill: u32,
            pub Characteristics: u32,
        }

        #[unsafe(link_section = ".rdata$T")]
        #[unsafe(export_name = "_tls_used")]
        static TLS_USED: IMAGE_TLS_DIRECTORY32 = unsafe {
            IMAGE_TLS_DIRECTORY32 {
                StartAddressOfRawData: UnsafePtr((&raw mut TLS_START).cast()),
                EndAddressOfRawData: UnsafePtr((&raw mut TLS_END).cast()),
                AddressOfIndex: UnsafePtr((&raw mut TLS_INDEX).cast()),
                AddressOfCallBacks: UnsafePtr((&raw mut CRT_TLS_CALLBACK_START).offset(1).cast()),
                SizeOfZeroFill: 0,
                Characteristics: 0,
            }
        };
    }
    target_arch = "x86_64" => {
        compiler_error!("x86_64 is not supported yet");
        // #[repr(C, packed(4))]
        // #[derive(Clone, Copy)]
        // pub struct IMAGE_TLS_DIRECTORY64 {
        //     pub StartAddressOfRawData: u64,
        //     pub EndAddressOfRawData: u64,
        //     pub AddressOfIndex: u64,
        //     pub AddressOfCallBacks: u64,
        //     pub SizeOfZeroFill: u32,
        //     pub Characteristics: u32,
        // }
    }
}
