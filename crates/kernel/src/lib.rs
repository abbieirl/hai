#![no_std]

use core::ffi::c_void;

use arrayvec::ArrayString;

pub mod alloc;
pub mod memory;

#[repr(C)]
pub enum Status {
    Success,
}

#[cfg(target_os = "uefi")]
impl From<Status> for uefi::Status {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => uefi::Status::SUCCESS,
        }
    }
}

#[repr(C)]
pub struct BootInfo {
    pub firmware_vendor: *const c_void,
    pub firmware_revision: u32,
    pub uefi_revision: u32,
}
