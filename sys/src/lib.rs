#![no_std]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/raw.rs"));

#[link(name = "SDL2")]
extern {}
