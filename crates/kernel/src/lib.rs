#![no_std]

pub mod alloc;
pub mod memory;

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
    pub revision: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FrameBuffer {}
