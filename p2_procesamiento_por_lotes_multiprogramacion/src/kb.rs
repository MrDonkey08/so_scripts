use crate::screen;

use std::io::{self, Read, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode; // Import IntoRawMode trait
use termion::terminal_size;

pub fn bind() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut keys = stdin.keys();

    if let Some(Ok(key)) = keys.next() {
        stdout.flush().unwrap();
        match key {
            Key::Char('p') => {
                pause();
            }
            Key::Char('c') => {
                continu();
            }
            Key::Char('w') => {
                error();
            }
            Key::Char('e') => {
                interruption();
            }
            _ => {}
        }
    }

    stdout.flush().unwrap();
}

fn pause() {
    let (term_width, term_height) = terminal_size().unwrap();
    let text = String::from("Paused");
    let x_top_right = term_width - text.len() as u16 - 1;
    screen::print_xy(x_top_right, 1, &text);
    bind();
}

fn continu() {
    //screen::print_xy(1, );
}

fn error() {
    println!("error");
}

fn interruption() {
    println!("interruption");
}
