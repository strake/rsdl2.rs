extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    ::bindgen::Builder::default()
        .use_core().ctypes_prefix("::libc")
        .unstable_rust(true)
        .header("/usr/include/SDL2/SDL.h")
        .header("/usr/include/SDL2/SDL_events.h")
        .header("/usr/include/SDL2/SDL_video.h")
        .clang_arg("-DSDL_VIDEO_DRIVER_X11")
        .hide_type("FP_[[:alpha:]]*")
        .raw_line("impl Clone for SDL_Event { #[inline] fn clone(&self) -> Self { *self } }")
        .raw_line("impl Copy for SDL_Event {}")
        .generate().ok()
        .and_then(|bs| bs.write_to_file(out_path.join("raw.rs")).ok())
        .unwrap();
}
