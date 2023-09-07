use std::ops::{Deref, DerefMut};
use std::ptr;

/// Wrapper to zero out memory when dropped.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Memzero<T: AsMut<[u8]>> {
    mem: T,
}

impl<T: AsMut<[u8]>> From<T> for Memzero<T> {
    fn from(mem: T) -> Memzero<T> {
        Memzero { mem }
    }
}

impl<T: AsMut<[u8]>> Drop for Memzero<T> {
    fn drop(&mut self) {
        unsafe {
            for byte_ref in self.mem.as_mut() {
                ptr::write_volatile(byte_ref, 0)
            }
        }
    }
}

impl<T: AsMut<[u8]>> Deref for Memzero<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}

impl<T: AsMut<[u8]>> DerefMut for Memzero<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mem
    }
}
