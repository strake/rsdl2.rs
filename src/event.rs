use core::mem;
use sys::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Window(::window::Id, ::window::Event),
    Keyboard { wid: ::window::Id, state: bool, repeat: bool, sym: ::key::Sym },
    Pointer { wid: ::window::Id, state: bool, pos: [i32; 2], button: Button },
    Text { wid: ::window::Id, text: [u8; 32] },
    Quit,
    #[doc(hidden)]
    __Inexhaustive(SDL_EventType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Button(u8);

impl Button {
    pub const Left  : Self = Button(SDL_BUTTON_LEFT   as _);
    pub const Middle: Self = Button(SDL_BUTTON_MIDDLE as _);
    pub const Right : Self = Button(SDL_BUTTON_RIGHT  as _);

    #[inline]
    pub const fn raw(self) -> u8 { self.0 }
}

impl From<SDL_Event> for Event {
    #[inline]
    fn from(raw: SDL_Event) -> Self {
        use self::Event::*;
        use ::window::Event::*;
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
            SDL_MOUSEBUTTONDOWN | SDL_MOUSEBUTTONUP => Pointer {
                wid: raw.button.windowID, state: SDL_MOUSEBUTTONDOWN as u32 == raw.type_,
                pos: [raw.button.x, raw.button.y],
                button: Button(raw.button.button as _),
            },
            SDL_TEXTINPUT => Text { wid: raw.text.windowID, text: mem::transmute(raw.text.text) },
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
