#![no_std]
#![no_main]

use kernel::Status;

#[cfg(target_os = "uefi")]
#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: *const core::ffi::c_void,
) -> uefi::Status {
    use uefi::boot::{MemoryType, exit_boot_services};
    use uefi::mem::memory_map::MemoryMap;

    unsafe { uefi::table::set_system_table(system_table.cast()) };
    unsafe { uefi::boot::set_image_handle(image_handle) };

    let mmap = unsafe { exit_boot_services(MemoryType::LOADER_DATA) };

    mmap.entries().for_each(|descriptor| match descriptor.ty {
        MemoryType::CONVENTIONAL => {
            let size = descriptor.page_count * 0x1000;
            let _end = descriptor.phys_start + size;
        }
        _ => (),
    });

    kernel_main().into()
}

#[unsafe(no_mangle)]
extern "C" fn kernel_main() -> Status {
    cpu64::interrupt::enable();

    #[allow(clippy::empty_loop)]
    loop {}
}
