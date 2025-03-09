#![no_std]

pub mod alloc;
pub mod memory;

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
    pub revision: u32,
    pub framebuffer: *mut u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct BootTime {
    pub uefi: u32,
    pub init: u32,
}
