use crate::screen;

use std::io::{self, Read,Write};
use std::{thread, time}; // For system sleep
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode; // Import IntoRawMode trait
use termion::terminal_size;

pub fn bind() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();

    let mut paused = false;

    loop {
        if let Some(Ok(key)) = keys.next() {
            stdout.flush().unwrap();

            if paused {
                if let Key::Char('c') = key {
                    print_status("Continue");
                } else {
                    continue;
                }
            } else {
                match key {
                    Key::Char('p') => {
                        print_status("Paused");
                        paused = true;
                        continue;
                    }
                    Key::Char('w') => {
                        print_status("Error");
                    }
                    Key::Char('e') => {
                        print_status("Interruption");
                    }
                    _ => { continue; }
                }
            }

            break;
        }

    }
    stdout.flush().unwrap();
}

fn print_status(text: &str) {
    let (term_width, _term_height) = terminal_size().unwrap();
    let x_top_right = term_width - text.len() as u16 - 1;
    screen::print_xy(x_top_right, 1, text);

    if text != "Paused" {
        let spaces: String = " ".repeat(text.len());
        thread::sleep(time::Duration::from_secs(1));
        screen::print_xy(x_top_right, 1, &spaces);
    }
}
