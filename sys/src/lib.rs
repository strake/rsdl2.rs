#![no_std]

#![allow(clippy::approx_constant)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/raw.rs"));

#[link(name = "SDL2")]
extern {}
