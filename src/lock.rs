use core::sync::atomic::{ATOMIC_BOOL_INIT, AtomicBool, Ordering as Memord};

use self::Memord::*;

pub static lock: Lock = Lock(ATOMIC_BOOL_INIT);

#[derive(Debug)]
pub struct Lock(AtomicBool);

impl Lock {
    #[inline]
    pub fn lock(&self) -> Option<Guard> {
        self.0.compare_exchange(false, true, Acquire, Relaxed).ok().map(|_| Guard(self))
    }
}

#[derive(Debug)]
pub struct Guard<'a>(&'a Lock);

impl<'a> Drop for Guard<'a> {
    #[inline]
    fn drop(&mut self) { (self.0).0.store(false, Release); }
}
