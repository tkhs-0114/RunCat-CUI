extern crate termion;
mod dot;
mod time;

use termion::{color,style};
use termion::cursor::{Goto, DetectCursorPos};
use termion::raw::IntoRawMode;

use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let pattern = [[
        "--------------0-0-",
        "--------------000-",
        "-------0--0000-0--",
        "--------000000000-",
        "-0--------000000--",
        "-0000----0------0-"
        ],[
        "-------------0-0--",
        "-------------000--",
        "---------0000-0---",
        "-----00000000000--",
        "---------000000---",
        "----0-----0--0----"
        ],[
        "---------0-0------",
        "---------000------",
        "-----0000-0-------",
        "--0000000000------",
        "--0--000000-------",
        "-----0----0-------"
        ],[
        "------------0-0---",
        "------------000---",
        "-----0--0000-0----",
        "-----0000000000---",
        "-00-----000000----",
        "-000--00------0---"
        ]];
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (x, y) = stdout.cursor_pos().unwrap();
    for cnt in 1..50 {
        let (w, h) = termion::terminal_size().unwrap();
        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::Green),
            color::Fg(color::Blue),
            time::draw(20, 1)
        ).unwrap();

        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::Green),
            color::Fg(color::Blue),
            dot::create(&pattern[cnt%4], 1, 1)
        ).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    };
    print!("{}{}", Goto(x, y), style::Reset);
}

