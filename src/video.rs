use core::marker::PhantomData;
use ptr_::Unique;
use sdl2_sys::*;

use ::Nul;
use ::libc::{c_int as int};

#[derive(Debug)]
pub struct Video<'a>(pub(crate) PhantomData<&'a ::Library>, pub(crate) [*mut (); 0]);

impl<'a> Video<'a> {
    #[inline]
    pub fn new_window(&self, title: &Nul<u8>, pos: (int, int), size: (int, int),
                      flags: WindowFlags) -> Result<Window, ::Error> { unsafe {
        Unique::new(SDL_CreateWindow(title.as_ptr() as _, pos.0, pos.1, size.0, size.1,
                                     flags.bits))
            .map(|w| Window(w, PhantomData)).ok_or(::Error::get())
    } }
}

pub struct Window<'a>(Unique<SDL_Window>, PhantomData<Video<'a>>);

impl<'a> Window<'a> {
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowFlags { bits: u32 }
