#![crate_name = "nspireio"]
#![crate_type = "rlib"]

#![feature(core, libc, collections, no_std)]
#![no_std]

extern crate core;
extern crate libc;
extern crate collections;

use libc::{c_char, c_int};
use core::num::ToPrimitive;
use core::result::Result;

mod ffi {
    use libc::{c_char, c_uchar, c_int};
    
    pub struct nio_console;

    // Shim functions
    extern "C" {
        pub fn alloc_console() -> *mut nio_console;
        pub fn dealloc_console(csl: *mut nio_console);
    }
    
    extern "C" {
        pub fn nio_init(csl: *mut nio_console, size_x: c_int, size_y: c_int,
                        offset_x: c_int, offset_y: c_int,
                        bkg_color: c_uchar, frg_color: c_uchar,
                        drawing_enabled: u8);

        pub fn nio_fputs(text: *const c_uchar, csl: *mut nio_console);
    }
}

pub mod consts {
    use libc::c_int;
    
    pub const NIO_MAX_COLS: c_int = 53;
    pub const NIO_MAX_ROWS: c_int = 30;

    #[repr(u8)]
    pub enum Color {
        Black = 0,
        Red,
        Green,
        Yellow,
        Blue,
        Magenta,
        Cyan,
        Gray,
        LightBlack,
        LightRed,
        LightGreen,
        LightYellow,
        LightBlue,
        LightMagenta,
        LightCyan,
        White
    }
}

pub struct NIOConsole {
    ptr: *mut ffi::nio_console,
}

impl NIOConsole {
    pub fn new(size_x: c_int, size_y: c_int, offset_x: c_int, offset_y: c_int,
           bkg_color: consts::Color, frg_color: consts::Color,
           drawing_enabled: bool) -> NIOConsole {
        let csl = NIOConsole {
            ptr: unsafe { ffi::alloc_console() },
        };

        unsafe {
            ffi::nio_init(csl.ptr, size_x, size_y, offset_x, offset_y,
                          (bkg_color as u8),
                          (frg_color as u8),
                          if drawing_enabled { 1 } else { 0 });
        }
        
        csl
    }
}

impl core::fmt::Write for NIOConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            ffi::nio_fputs(s.as_ptr(), self.ptr);
        }
        
        Result::Ok(())
    }
}

impl core::ops::Drop for NIOConsole {
    fn drop(&mut self) {
        unsafe {
            ffi::dealloc_console(self.ptr)
        }
    }
}
