#[repr(C)]
#[derive(Debug)]
pub struct MemoryMap {
    entries: &'static mut [MemoryRegion],
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryRegion {
    start: u64,
    end: u64,
}
