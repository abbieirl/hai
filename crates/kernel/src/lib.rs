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
    firmware_vendor: *const c_void,
    firmware_revision: u32,
    uefi_revision: u32,
}

impl BootInfo {
    #[cfg(target_os = "uefi")]
    pub fn uefi() -> Self {
        use uefi::system::{firmware_revision, firmware_vendor, uefi_revision};

        let mut buf = ArrayString::<128>::new();
        firmware_vendor().as_str_in_buf(&mut buf).unwrap();

        BootInfo {
            firmware_vendor: buf.as_ptr().cast(),
            firmware_revision: firmware_revision(),
            uefi_revision: uefi_revision().0,
        }
    }
}
