use core::mem;
use sys::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Window(::window::Id, ::window::Event),
    Keyboard { wid: ::window::Id, state: bool, repeat: bool, sym: ::key::Sym },
    Quit,
    #[doc(hidden)]
    __Inexhaustive(SDL_EventType),
}

impl From<SDL_Event> for Event {
    #[inline]
    fn from(raw: SDL_Event) -> Self {
        use self::Event::*;
        use ::window::Event::*;
        use SDL_EventType::*;
        use SDL_WindowEventID::*;
        unsafe { match mem::transmute(raw.type_) {
            SDL_QUIT => Quit,
            SDL_WINDOWEVENT => Window(raw.window.windowID,
                                      match mem::transmute(raw.window.event as u32) {
                SDL_WINDOWEVENT_SHOWN => Shown,
                SDL_WINDOWEVENT_HIDDEN => Hidden,
                SDL_WINDOWEVENT_EXPOSED => Exposed,
                SDL_WINDOWEVENT_MOVED => Moved([raw.window.data1, raw.window.data2]),
                SDL_WINDOWEVENT_RESIZED => Resized([raw.window.data1, raw.window.data2]),
                SDL_WINDOWEVENT_SIZE_CHANGED => SizeChanged([raw.window.data1,
                                                             raw.window.data2]),
                SDL_WINDOWEVENT_MINIMIZED => Minimized,
                SDL_WINDOWEVENT_MAXIMIZED => Maximized,
                SDL_WINDOWEVENT_RESTORED => Restored,
                SDL_WINDOWEVENT_ENTER => PointerFocus(true),
                SDL_WINDOWEVENT_LEAVE => PointerFocus(false),
                SDL_WINDOWEVENT_FOCUS_GAINED => KeyboardFocus(false),
                SDL_WINDOWEVENT_FOCUS_LOST => KeyboardFocus(false),
                SDL_WINDOWEVENT_CLOSE => Close,
                SDL_WINDOWEVENT_TAKE_FOCUS => TakeFocus,
                SDL_WINDOWEVENT_HIT_TEST => HitTest,
                _ => ::window::Event::__Inexhaustive,
            }),
            SDL_KEYDOWN | SDL_KEYUP => Keyboard {
                wid: raw.key.windowID, state: SDL_KEYDOWN as u32 == raw.type_,
                repeat: 0 != raw.key.repeat, sym: ::key::Sym {
                    scan: raw.key.keysym.scancode,
                    sym: raw.key.keysym.sym,
                    mod_: ::key::Mod::from_bits_truncate(raw.key.keysym.mod_),
                    x: None,
                }
            },
            t => self::Event::__Inexhaustive(t),
        }
    } }
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
