#![crate_id = "termbox#0.1.0"]
#![crate_type = "lib" ]
#![feature(link_args)]
#![feature(struct_variant)] 


extern crate std;
use std::libc::types::os::arch::c95::{ c_int, c_uint};
use std::task;

/*
 *
 * A lightweight curses alternative wrapping the termbox library.
 *
 * # SYNOPSIS
 *
 * A hello world for the terminal:
 *
 *     use std;
 *     use termbox;
 *
 *     import tb = termbox;
 *
 *     fn main() {
 *         tb::init();
 *         tb::print(1, 1, tb::bold, tb::White, tb::Black, "Hello, world!");
 *         tb::present();
 *         std::timer::sleep(std::uv_global_loop::get(), 1000);
 *         tb::shutdown();
 *     }
 *
 * # DESCRIPTION
 *
 * Output is double-buffered.
 *
 * TODO
 *
 * # EXAMPLES
 *
 * TODO
 *
 */

// Exported functions
// export init, shutdown
//      , width, height
//      , clear, present
//      , set_cursor
//      , print, print_ch
//      , poll_event, peek_event
//      , event;

// Exported types
// export color, Style
//      , event;



/*
 * The event type matches struct tb_event from termbox.h
 */
pub struct RawEvent {
    etype: u8,
    emod: u8,
    key: u16,
    ch: u32,
    w: i32,
    h: i32,
}



/*
 * Foreign functions from termbox.
 */
mod c {
    use std::libc::types::os::arch::c95::{ c_int, c_uint};

    #[link(name = "termbox")]
    extern {

        pub fn tb_init() -> c_int;
        pub fn tb_shutdown();

        pub fn tb_width() -> c_uint;
        pub fn tb_height() -> c_uint;

        pub fn tb_clear();
        pub fn tb_present();

        pub fn tb_set_cursor(cx: c_int, cy: c_int);

        pub fn tb_change_cell(x: c_uint, y: c_uint, ch: u32, fg: u16, bg: u16);

        pub fn tb_select_input_mode(mode: c_int) -> c_int;
        pub fn tb_set_clear_attributes(fg: u16, bg: u16);

        pub fn tb_peek_event(ev: *::RawEvent, timeout: c_uint) -> c_int;
        pub fn tb_poll_event(ev: *::RawEvent) -> c_int;
    }
}

pub fn init() -> int { 
    unsafe { c::tb_init() as int }
}

pub fn shutdown() { 
    unsafe { c::tb_shutdown(); }
}

pub fn width() -> uint { 
    unsafe { 
        return  c::tb_width() as uint; 
    }
}

pub fn height() -> uint { 
    unsafe {
        return  c::tb_height() as uint; 
    }
}

/**
 * Clear buffer.
 */
pub fn clear() { 
    unsafe {
        c::tb_clear(); 
    }
}

// /**
//  * Write buffer to terminal.
//  */
pub fn present() { 
    unsafe {
        c::tb_present(); 
    }
}

pub fn set_cursor(cx: int, cy: int) { 
    unsafe {
        c::tb_set_cursor(cx as c_int, cy as c_int); 
    }
}

// low-level wrapper
pub fn change_cell(x: uint, y: uint, ch: u32, fg: u16, bg: u16) { 
    unsafe {
        c::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg); 
    }
}

/// Convert from enums to u16
pub fn convert_color(c: Color) -> u16 {
    match c {
        Black   => 0x00,
        Red     => 0x01,
        Green   => 0x02,
        Yellow  => 0x03,
        Blue    => 0x04,
        Magenta => 0x05,
        Cyan    => 0x06,
        White   => 0x07,
    }
}

pub fn convert_style(sty: Style) -> u16 {
    match sty {
        Normal         => 0x00,
        Bold           => 0x10,
        Underline      => 0x20,
        BoldUnderline => 0x30,
    }
}

/**
 * Print a string to the buffer.  Leftmost charater is at (x, y).
 */
pub fn print(x: uint, y: uint, sty: Style, fg: Color, bg: Color, s: ~str) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    for (i, ch) in s.chars().enumerate() {
        unsafe {
            c::tb_change_cell((x + i) as c_uint, y as c_uint, ch as u32, fg, bg);
        }
    }
}

// /**
//  * Print a charater to the buffer.
//  */
pub fn print_ch(x: uint, y: uint, sty: Style, fg: Color, bg: Color, ch: char) {
    unsafe {
        let fg: u16 = convert_color(fg) | convert_style(sty);
        let bg: u16 = convert_color(bg);
        c::tb_change_cell(x as c_uint, y as c_uint, ch as u32, fg, bg);
    }
}

pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

pub enum Style {
    Normal,
    Bold,
    Underline,
    BoldUnderline
}

//Convenience functions
pub fn with_term(f: proc:Send()) {
    init();
    let res = task::try(f);
    shutdown();
    match res {
        Err(_) => {            
            fail!("with_term: An error occured.");
        }
        _ => {}
    }
}

pub fn nil_raw_event() -> RawEvent { 
    RawEvent{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

pub enum Event {
    KeyEvent(u8, u16, u32),
    ResizeEvent(i32, i32),
    NoEvent
}

/**
 * Get an event if within timeout milliseconds, otherwise return urn no_event.
 */

pub fn peek_event(timeout: uint) -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = c::tb_peek_event(&ev as *RawEvent, timeout as c_uint);
        return unpack_event(rc, &ev);
    }
}

// /**
//  * Blocking function to return urn next event.
//  */
pub fn poll_event() -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = c::tb_poll_event(&ev as *RawEvent);
        return unpack_event(rc, &ev);
    }
}

// /* helper pub fn
//  *
//  * ev_type
//  *   0 -> no event
//  *   1 -> key
//  *   2 -> resize
//  *   -1 -> error
//  */
pub fn unpack_event(ev_type: c_int, ev: &RawEvent) -> Event {
    match ev_type {
        0 => NoEvent,
        1 => {
            return KeyEvent(ev.emod, ev.key, ev.ch);
        },
        2 => {
            return ResizeEvent(ev.w, ev.h);
        },
        _ => { fail!("asdf"); }
    }
}

