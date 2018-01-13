use core::marker::PhantomData;
use core::{mem, slice};
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

    #[inline]
    pub fn new_rgb_surface(&self, size: [int; 2], depth: int,
                           mask: [u32; 4]) -> Result<Ptr<'a, Surface>, ::Error> { unsafe {
        Ptr::new(SDL_CreateRGBSurface(0,       size[0], size[1], depth,
                                      mask[0], mask[1], mask[2], mask[3]) as _)
            .ok_or(::Error::get())
    } }

    #[inline]
    pub fn new_rgb_surface_from(&self, data: PixelsMut<'a>,
                                mask: [u32; 4]) -> Result<Ptr<'a, Surface>, ::Error> { unsafe {
        Ptr::new(SDL_CreateRGBSurfaceFrom(data.ptr as *mut _ as _, data.size[0], data.size[1],
                                          data.depth, data.pitch,
                                          mask[0], mask[1], mask[2], mask[3]) as _)
            .ok_or(::Error::get())
    } }
}

extern { pub type Pixels; }

pub struct PixelsMut<'a> {
    ptr: &'a mut Pixels,
    size: [int; 2],
    depth: int,
    pitch: int,
}

impl<'a> PixelsMut<'a> {
    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut (), size: [int; 2],
                                 depth: int, pitch: int) -> Self { Self {
        ptr: mem::transmute(ptr), size, depth, pitch,
    } }

    #[inline]
    pub fn rows(&mut self) -> slice::ChunksMut<u8> {
        let p = self.pitch as usize;
        self.raw_bytes().chunks_mut(p)
    }

    #[inline]
    pub fn raw_bytes(&mut self) -> &mut [u8] { unsafe {
        slice::from_raw_parts_mut(self.ptr as *mut _ as *mut u8,
                                  (self.pitch * self.size[1]) as _)
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

impl Renderer {
    #[inline]
    pub fn clear(&self) -> Result<(), ::Error> { unsafe {
        if SDL_RenderClear(&self.0 as *const _ as _) < 0 { Err(::Error::get()) } else { Ok(()) }
    } }

    #[inline]
    pub fn present(&self) { unsafe { SDL_RenderPresent(&self.0 as *const _ as _) } }

    #[inline]
    pub fn copy_from(&self, tex: &Texture,
                     src: Option<Rect>, dst: Option<Rect>) -> Result<(), ::Error> { unsafe {
        if SDL_RenderCopy(&self.0 as *const _ as _, tex as *const _ as *mut _,
                          to_ptr(&src) as _, to_ptr(&dst) as _) < 0 { Err(::Error::get()) }
        else { Ok(()) }
    } }

    #[inline]
    pub fn new_texture_from_surface(&self, surf: &Surface) -> Result<Ptr<Texture>, ::Error> { unsafe {
        Ptr::new(SDL_CreateTextureFromSurface(&self.0 as *const _ as _, surf as *const _ as *mut _) as _)
            .ok_or(::Error::get())
    } }
}

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

#[repr(C)]
pub struct Texture(SDL_Texture);

impl ::ptr::private::Sealed for Texture {}
impl ::ptr::DropPtr for Texture {
    #[inline]
    fn drop_ptr(ptr: *mut Self) { unsafe { SDL_DestroyTexture(ptr as _) } }
}

#[repr(C)]
pub struct Surface(SDL_Surface);

impl Surface {
    #[inline]
    pub fn size(&self) -> [int; 2] { [self.0.w, self.0.h] }

    #[inline]
    pub fn pixels_mut(&mut self) -> PixelsMut { unsafe { PixelsMut {
        ptr: mem::transmute(self.0.pixels),
        size: [self.0.w, self.0.h],
        depth: (*self.0.format).BitsPerPixel as _,
        pitch: self.0.pitch,
    } } }

    #[inline]
    pub fn raw(&self) -> &SDL_Surface { &self.0 }

    #[inline]
    pub unsafe fn raw_mut(&mut self) -> &mut SDL_Surface { &mut self.0 }
}

impl ::ptr::private::Sealed for Surface {}
impl ::ptr::DropPtr for Surface {
    #[inline]
    fn drop_ptr(ptr: *mut Self) { unsafe { SDL_FreeSurface(ptr as _) } }
}

#[repr(C)]
pub struct Rect { pub pos: [int; 2], pub size: [int; 2] }

pub use ::sys::SDL_Color as Color;

fn to_ptr<A>(a: &Option<A>) -> *const A { a.as_ref().map_or(::core::ptr::null(), |p| p as _) }
