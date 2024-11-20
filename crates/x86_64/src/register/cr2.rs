use crate::address::VirtualAddress;
use core::arch::asm;

#[derive(Debug)]
pub struct CR2;

impl CR2 {
    #[inline]
    pub fn read() -> VirtualAddress {
        let address: u64;
        unsafe { asm!("mov {}, cr2", out(reg) address, options(nomem, nostack, preserves_flags)) };
        VirtualAddress::new(address)
    }
}
