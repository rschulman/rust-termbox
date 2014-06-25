extern crate termbox;

use std::char;
use tb = termbox;

fn print(x: uint, y: uint, s: str) {
    tb::print(x, y, tb::Bold, tb::White, tb::Black, s);
}

fn main() {
    tb::init();
    print(1, 1, "Hello, world!");
    print(1, 3, "Press 'q' to quit.");
    tb::present();
    loop {
        match tb::poll_event() {
            tb::KeyEvent(_, _, ch) => {
                match char::from_u32(ch) {
                    Some('q') => { break; },
                    _ => {}
                }
            },
            _ => { }
        }
    }
    tb::shutdown();
}

