#![no_std]

pub mod alloc;
pub mod memory;

#[repr(C)]
pub struct BootInfo {
    pub revision: u32,
}

#[repr(C)]
pub struct FrameBuffer {}
