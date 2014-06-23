extern crate termbox;

use tb = termbox;

fn main() {
    tb::init();
    tb::print(1, 1, tb::Bold, tb::White, tb::Black, ~"Hello, world!");
    tb::present();

    std::io::timer::sleep(1000);

    tb::shutdown();
}
