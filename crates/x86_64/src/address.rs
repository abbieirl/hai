#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VirtualAddress(u64);

impl VirtualAddress {
    #[inline]
    pub const fn new(address: u64) -> Self {
        Self(address)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PhysicalAddress(u64);

impl PhysicalAddress {
    #[inline]
    pub const fn new(address: u64) -> Self {
        Self(address)
    }
}