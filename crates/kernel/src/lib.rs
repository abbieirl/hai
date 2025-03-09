#![no_std]

pub mod alloc;
pub mod memory;

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo<'a> {
    pub revision: u32,
    pub framebuffer: &'a Framebuffer,
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
