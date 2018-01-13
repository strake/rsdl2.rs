use core::marker::PhantomData;
use ptr::Ptr;
use sys::*;

use ::Nul;
use ::libc::{c_int as int};

#[derive(Debug)]
pub struct Video<'a>(pub(crate) PhantomData<&'a ::Library>, pub(crate) [*mut (); 0]);

impl<'a> Video<'a> {
    #[inline]
    pub fn new_window(&self, title: &Nul<u8>, pos: [WindowPos; 2], size: [int; 2],
                      flags: WindowFlags) -> Result<Ptr<'a, Window>, ::Error> { unsafe {
        Ptr::new(SDL_CreateWindow(title.as_ptr() as _, pos[0].to_int(), pos[1].to_int(),
                                  size[0], size[1], flags.bits) as _).ok_or(::Error::get())
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

#[repr(C)]
pub struct Window(SDL_Window);

impl Window {
    #[inline]
    pub fn new_renderer(&mut self, ix: Option<int>,
                        flags: RendererFlags) -> Result<Ptr<Renderer>, ::Error> { unsafe {
        Ptr::new(SDL_CreateRenderer(&mut self.0, ix.unwrap_or(-1), flags.bits) as _)
            .ok_or(::Error::get())
    } }
}

impl ::ptr::private::Sealed for Window {}
impl ::ptr::DropPtr for Window {
    #[inline]
    fn drop_ptr(ptr: *mut Self) { unsafe { SDL_DestroyWindow(ptr as _); } }
}

bitflags! {
    pub struct WindowFlags: u32 {
        const Fullscreen        = SDL_WindowFlags::SDL_WINDOW_FULLSCREEN         as _;
        const FullscreenDesktop = SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP as _;
        const OpenGL            = SDL_WindowFlags::SDL_WINDOW_OPENGL             as _;
        const Hidden            = SDL_WindowFlags::SDL_WINDOW_HIDDEN             as _;
        const Borderless        = SDL_WindowFlags::SDL_WINDOW_BORDERLESS         as _;
        const Resizable         = SDL_WindowFlags::SDL_WINDOW_RESIZABLE          as _;
        const Minimized         = SDL_WindowFlags::SDL_WINDOW_MINIMIZED          as _;
        const Maximized         = SDL_WindowFlags::SDL_WINDOW_MAXIMIZED          as _;
        const InputGrabbed      = SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED      as _;
        const AllowHighDPI      = SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI      as _;
    }
}

#[repr(C)]
pub struct Renderer(SDL_Renderer);

impl ::ptr::private::Sealed for Renderer {}
impl ::ptr::DropPtr for Renderer {
    #[inline]
    fn drop_ptr(ptr: *mut Self) { unsafe { SDL_DestroyRenderer(ptr as _); } }
}

bitflags! {
    pub struct RendererFlags: u32 {
        const Software      = SDL_RendererFlags::SDL_RENDERER_SOFTWARE      as _;
        const Accelerated   = SDL_RendererFlags::SDL_RENDERER_ACCELERATED   as _;
        const PresentVSync  = SDL_RendererFlags::SDL_RENDERER_PRESENTVSYNC  as _;
        const TargetTexture = SDL_RendererFlags::SDL_RENDERER_TARGETTEXTURE as _;
    }
}
