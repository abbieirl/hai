#![no_std]
#![no_main]

use kernel::BootInfo;

#[uefi::entry]
#[cfg(target_os = "uefi")]
fn efi_main() -> uefi::Status {
    use uefi::boot::{MemoryType, exit_boot_services};
    use uefi::boot::{get_handle_for_protocol, open_protocol_exclusive};
    use uefi::mem::memory_map::MemoryMap;
    use uefi::proto::console::gop::GraphicsOutput;
    use uefi::proto::console::text::Output;
    use uefi::system::{firmware_revision, firmware_vendor};
    use core::fmt::Write;

    let _vendor = firmware_vendor();
    let revision = firmware_revision();

    {
        let handle = get_handle_for_protocol::<Output>().unwrap();
        let mut console = open_protocol_exclusive::<Output>(handle).unwrap();
        writeln!(console, "HaiOS UEFI Boot v{}", env!("CARGO_PKG_VERSION")).unwrap();
    }

    let framebuffer = {
        let handle = get_handle_for_protocol::<GraphicsOutput>().unwrap();
        let mut gop = open_protocol_exclusive::<GraphicsOutput>(handle).unwrap();
        let _mode_info = gop.current_mode_info();
        gop.frame_buffer().as_mut_ptr()
    };

    let mmap = unsafe { exit_boot_services(MemoryType::LOADER_DATA) };

    mmap.entries().for_each(|descriptor| match descriptor.ty {
        MemoryType::CONVENTIONAL => {
            let size = descriptor.page_count * 0x1000;
            let _end = descriptor.phys_start + size;
        }
        _ => (),
    });

    kernel_main(BootInfo { revision, framebuffer });

    uefi::Status::SUCCESS
}

#[unsafe(no_mangle)]
extern "C" fn kernel_main(_boot_info: BootInfo) {
    cpu64::interrupt::enable();

    #[allow(clippy::empty_loop)]
    loop {}
}
