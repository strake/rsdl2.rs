use core::mem;
use sys::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Window(::window::Id, ::window::Event),
    Keyboard { wid: ::window::Id, state: bool, repeat: bool, sym: ::key::Sym },
    Quit,
    #[doc(hidden)]
    __Inexhaustive,
}

impl From<SDL_Event> for Event {
    #[inline]
    fn from(raw: SDL_Event) -> Self { unsafe { use self::Event::*; use SDL_EventType::*; match mem::transmute(raw.type_) {
        SDL_QUIT => Quit,
        SDL_WINDOWEVENT => Window(raw.window.windowID, ::window::Event::__Inexhaustive),
        SDL_KEYDOWN | SDL_KEYUP => Keyboard {
            wid: raw.key.windowID, state: SDL_KEYDOWN as u32 == raw.type_,
            repeat: 0 != raw.key.repeat, sym: ::key::Sym {
                scan: ::key::ScanCode::__Inexhaustive,
                sym: ::key::Code::__Inexhaustive,
                mod_: ::key::Mod::from_bits_truncate(raw.key.keysym.mod_),
                x: None,
            }
        },
        _ => __Inexhaustive,
    } } }
}

impl<'a> ::Video<'a> {
    #[inline]
    pub fn events<'b>(&'b self, wait: bool) -> Iter<'b, 'a> { Iter(wait, self) }
}

pub struct Iter<'a, 'b: 'a>(bool, &'a ::Video<'b>);

impl<'a, 'b> Iterator for Iter<'a, 'b> {
    type Item = (Event, ::time::Stamp);
    #[inline]
    fn next(&mut self) -> Option<(Event, ::time::Stamp)> { unsafe {
        let mut ev = mem::uninitialized();
        if if self.0 { SDL_WaitEvent } else { SDL_PollEvent } (&mut ev) > 0 {
            Some((Event::from(ev), ev.common.timestamp))
        } else { None }
    } }
}
