#![no_std]

#![allow(clippy::approx_constant)]
#![allow(improper_ctypes)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/raw.rs"));

#[link(name = "SDL2")]
extern {}
