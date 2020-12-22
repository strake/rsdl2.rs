extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    ::bindgen::Builder::default()
        .use_core().ctypes_prefix("::libc")
        .rust_target(::bindgen::RustTarget::Nightly)
        .header("/usr/include/SDL2/SDL.h")
        .header("/usr/include/SDL2/SDL_events.h")
        .header("/usr/include/SDL2/SDL_video.h")
        .clang_arg("-DSDL_VIDEO_DRIVER_X11")
        .blacklist_item("FP_[[:alpha:]]*")
        .prepend_enum_name(false)
        .generate().ok()
        .and_then(|bs| bs.write_to_file(out_path.join("raw.rs")).ok())
        .unwrap();
}
