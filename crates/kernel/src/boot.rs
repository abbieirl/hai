use core::ffi::c_void;

#[repr(C)]
pub struct BootInfo {
    pub firmware_vendor: *const c_void,
    pub firmware_revision: u32,
    pub uefi_revision: u32,
}
