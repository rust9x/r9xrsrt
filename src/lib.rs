#![feature(abi_custom)]
#![no_std]
#![no_main]
#![no_builtins]
#![allow(clippy::missing_safety_doc)]

mod aull_x86;
mod bindings;
mod chkstk;
mod entry;
mod float;
mod mem;
mod static_init;
mod tls;
mod windows_link;

#[link(name = "kernel32")]
unsafe extern "system" {}
