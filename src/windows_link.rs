//! Custom impl for windows_link from rust/library/windows_link, to allow for non raw-dylib linking.
#![allow(unused)]

macro_rules! link_raw_dylib {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[cfg_attr(not(target_arch = "x86"), link(name = $library, kind = "raw-dylib", modifiers = "+verbatim"))]
        #[cfg_attr(target_arch = "x86", link(name = $library, kind = "raw-dylib", modifiers = "+verbatim", import_name_type = "undecorated"))]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    );
}

macro_rules! link_dylib {
    ($library:literal $abi:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern $abi {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

#[cfg(feature = "windows_raw_dylib")]
macro_rules! link_ {
    ($($tt:tt)*) => {
        $crate::windows_link::link_raw_dylib!($($tt)*);
    }
}

#[cfg(not(feature = "windows_raw_dylib"))]
macro_rules! link_ {
    ($($tt:tt)*) => {
        $crate::windows_link::link_dylib!($($tt)*);
    }
}

pub(crate) use {link_ as link, link_dylib, link_raw_dylib};
