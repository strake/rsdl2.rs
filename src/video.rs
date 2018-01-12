use core::marker::PhantomData;
use ptr_::Unique;
use sdl2_sys::*;

use ::Nul;
use ::libc::{c_int as int};

#[derive(Debug)]
pub struct Video<'a>(pub(crate) PhantomData<&'a ::Library>, pub(crate) [*mut (); 0]);

impl<'a> Video<'a> {
    #[inline]
    pub fn new_window(&self, title: &Nul<u8>, pos: [WindowPos; 2], size: [int; 2],
                      flags: WindowFlags) -> Result<Window, ::Error> { unsafe {
        Unique::new(SDL_CreateWindow(title.as_ptr() as _, pos[0].to_int(), pos[1].to_int(),
                                     size[0], size[1], flags.bits))
            .map(|w| Window(w, PhantomData)).ok_or(::Error::get())
    } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WindowPos {
    Centered,
    Undefined,
    At(int),
}

impl WindowPos {
    fn to_int(self) -> int { use self::WindowPos::*; match self {
        Centered => SDL_WINDOWPOS_CENTERED_MASK as _,
        Undefined => SDL_WINDOWPOS_UNDEFINED_MASK as _,
        At(n) => n,
    } }
}

pub struct Window<'a>(Unique<SDL_Window>, PhantomData<Video<'a>>);

impl<'a> Window<'a> {
}

impl<'a> Drop for Window<'a> {
    #[inline]
    fn drop(&mut self) { unsafe { SDL_DestroyWindow(self.0.as_ptr()) } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowFlags { bits: u32 }
