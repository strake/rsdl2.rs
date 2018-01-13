#![no_std]

#![feature(extern_types)]

#[macro_use]
extern crate bitflags;
extern crate libc;
extern crate null_terminated;
pub extern crate sdl2_sys as sys;

use core::{cmp, fmt};
use core::marker::PhantomData;
use core::ops::Deref;
use null_terminated::Nul;
use sys::*;

mod lock;

pub mod ptr;
pub use ptr::Ptr;

#[derive(Debug)]
pub struct Library(lock::Guard<'static>, [*mut (); 0]);

impl Library {
    #[inline]
    pub fn new() -> Result<Self, Error> {
        if let Some(g) = lock::lock.lock() {
            if unsafe { SDL_Init(0) } < 0 { Err(Error::get()) } else { Ok(Library(g, [])) }
        } else { Err(Error::from_str("SDL already in use")) }
    }

    #[inline]
    pub fn video(&self) -> Result<Video, Error> {
        if unsafe { SDL_Init(SDL_INIT_VIDEO) } < 0 { Err(Error::get()) }
        else { Ok(Video(PhantomData, [])) }
    }
}

impl Drop for Library {
    #[inline]
    fn drop(&mut self) { unsafe { SDL_Quit(); } }
}

pub mod video;
pub use video::Video;

#[derive(Clone, Copy)]
pub struct Error([u8; MAX_ERR_LENGTH]);

impl PartialEq for Error {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.0[..] == other.0[..] }
}

impl Eq for Error {}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::core::str::from_utf8(&self.0).map_err(|_| fmt::Error).and_then(|s| f.write_str(s))
    }
}

impl Deref for Error {
    type Target = Nul<u8>;
    #[inline]
    fn deref(&self) -> &Nul<u8> { unsafe { Nul::new_unchecked(&self.0[0]) } }
}

impl Error {
    #[inline]
    fn from_bytes(s: &[u8]) -> Self {
        let mut e = Error([0; MAX_ERR_LENGTH]);
        let l = cmp::min(s.len(), MAX_ERR_LENGTH - 1);
        e.0[0..l].copy_from_slice(&s[0..l]);
        e
    }

    #[inline]
    fn from_str(s: &str) -> Self { Self::from_bytes(s.as_bytes()) }

    #[inline]
    fn get() -> Self {
        Self::from_bytes(&unsafe { Nul::new_unchecked(SDL_GetError() as *const u8) }[..])
    }
}

const MAX_ERR_LENGTH: usize = 1 << 10;
