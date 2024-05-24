use crate::screen;

use std::io::{self, Write};
use std::time::Duration;
use std::thread; // For system sleep
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode; // Import IntoRawMode trait
use termion::terminal_size;

pub fn bind() -> char {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();
    let mut key_char = '\0';
    let mut paused = false;

    loop {
        if let Some(Ok(key)) = keys.next() {
            stdout.flush().unwrap();

            if paused {
                if let Key::Char('c') = key {
                    print_status("Continue");
                    key_char = 'c';
                    break;
                } else { continue; }
            } else {
                match key {
                    Key::Char('p') => {
                        print_status("Paused");
                        key_char = 'p';
                        paused = true;
                        continue;
                    }
                    Key::Char('w') => {
                        print_status("Error");
                        key_char = 'w';
                    }
                    Key::Char('e') => {
                        print_status("Interruption");
                        key_char = 'e';
                    }
                    Key::Char('x') => {}
                    _ => { continue; }
                }
            }

            break;
        }

    }
    stdout.flush().unwrap();
    key_char
}

fn print_status(text: &str) {
    let (term_width, _term_height) = terminal_size().unwrap();
    let x_top_right = term_width - text.len() as u16 - 1;
    screen::print_xy(x_top_right, 1, text);

    if text != "Paused" {
        let spaces: String = " ".repeat(text.len());
        thread::sleep(Duration::from_secs(1));
        screen::print_xy(x_top_right, 1, &spaces);
    }
}
