#![no_std]

use core::ffi::c_char;

pub mod alloc;
pub mod memory;

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
    pub firmware_vendor: *const c_char,
    pub firmware_revision: u32,
    pub uefi_revision: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BootTime {
    pub uefi: u32,
    pub init: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Framebuffer {
    pub ptr: *mut u8,
}

impl Framebuffer {
    fn read() {}
    fn write() {}
}
