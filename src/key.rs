use sys::*;

use self::SDL_Keymod::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sym {
    pub(crate) scan: ScanCode,
    pub(crate) sym: Code,
    pub(crate) mod_: Mod,
    pub(crate) x: Option<char>,
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
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScanCode {
    #[doc(hidden)]
    __Inexhaustive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Code {
    #[doc(hidden)]
    __Inexhaustive,
}
