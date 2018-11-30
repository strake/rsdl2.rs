use null_terminated::Nul;
use sys::*;

use self::SDL_Keymod::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sym {
    pub scan: ScanCode,
    pub sym: Code,
    pub mod_: Mod,
    pub x: Option<char>,
}

bitflags! {
    pub struct Mod: u16 {
        const LShift = KMOD_LSHIFT as _;
        const RShift = KMOD_RSHIFT as _;
        const LCtrl  = KMOD_LCTRL  as _;
        const RCtrl  = KMOD_RCTRL  as _;
        const LAlt   = KMOD_LALT   as _;
        const RAlt   = KMOD_RALT   as _;
        const LGUI   = KMOD_LGUI   as _;
        const RGUI   = KMOD_RGUI   as _;
        const Num    = KMOD_NUM    as _;
        const Caps   = KMOD_CAPS   as _;
        const Mode   = KMOD_MODE   as _;
        const Shift  = KMOD_LSHIFT as u16 | KMOD_RSHIFT as u16;
        const Ctrl   = KMOD_LCTRL  as u16 | KMOD_RCTRL  as u16;
        const Alt    = KMOD_LALT   as u16 | KMOD_RALT   as u16;
        const GUI    = KMOD_LGUI   as u16 | KMOD_RGUI   as u16;
    }
}

pub use ::sys::SDL_Scancode as ScanCode;
pub use ::sys::SDL_Keycode as Code;

#[inline]
pub fn code_from_name(name: &Nul<u8>) -> Code { unsafe {
    SDL_GetKeyFromName(name.as_ptr() as _)
} }
