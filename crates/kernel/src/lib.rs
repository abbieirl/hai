#![no_std]

pub mod alloc;
pub mod memory;
pub mod boot;

use boot::BootInfo;

#[cfg(target_os = "uefi")]
#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: *const core::ffi::c_void,
) -> uefi::Status {
    use arrayvec::ArrayString;
    use uefi::boot::{MemoryType, exit_boot_services};
    use uefi::mem::memory_map::MemoryMap;
    use uefi::system::{firmware_revision, firmware_vendor, uefi_revision};

    unsafe { uefi::table::set_system_table(system_table.cast()) };
    unsafe { uefi::boot::set_image_handle(image_handle) };

    let (firmware_vendor, firmware_revision, uefi_revision) = {
        let mut buf = ArrayString::<128>::new();
        firmware_vendor().as_str_in_buf(&mut buf).unwrap();
        (buf.as_ptr().cast(), firmware_revision(), uefi_revision().0)
    };

    let mmap = unsafe { exit_boot_services(MemoryType::LOADER_DATA) };

    mmap.entries().for_each(|descriptor| match descriptor.ty {
        MemoryType::CONVENTIONAL => {
            let size = descriptor.page_count * 0x1000;
            let _end = descriptor.phys_start + size;
        }
        _ => (),
    });

    kernel_main(BootInfo {
        firmware_vendor,
        firmware_revision,
        uefi_revision,
    })
    .into()
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_boot_info: BootInfo) -> Status {
    cpu64::interrupt::enable();

    loop {
        core::hint::spin_loop();
    }
}

#[repr(C)]
pub enum Status {
    Success,
}

#[cfg(target_os = "uefi")]
impl From<Status> for uefi::Status {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => Self::SUCCESS,
        }
    }
}