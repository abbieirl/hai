use crate::address::Virtual;
use core::arch::asm;

use super::Read;

#[derive(Debug)]
pub struct CR2;

impl Read for CR2 {
    type Output = Virtual;

    #[inline]
    fn read() -> Self::Output {
        let address: u64;
        unsafe { asm!("mov {}, cr2", out(reg) address, options(nomem, nostack, preserves_flags)) };
        Virtual::new(address)
    }
}
