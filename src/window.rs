pub type Id = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Shown, Hidden, Exposed,
    Moved([i32; 2]),
    Resized([i32; 2]),
    SizeChanged([i32; 2]),
    Minimized, Maximized, Restored,
    PointerFocus(bool), KeyboardFocus(bool),
    Close, TakeFocus, HitTest,
    #[doc(hidden)]
    __Inexhaustive,
}
