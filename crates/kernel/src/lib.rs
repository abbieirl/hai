#![no_std]

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
