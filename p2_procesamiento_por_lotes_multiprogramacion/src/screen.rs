use std::process::Command;
use std::io::{self, stdout, Write};
use termion::{cursor::DetectCursorPos, cursor::Goto, clear::CurrentLine};


pub fn clear() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

pub fn pause() {
    println!("Press Enter to continue...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

pub fn print_xy(x: u16, y: u16, text: &str) {
    // Get stdout
    let mut stdout = stdout();

    // Save the current cursor position
    let (saved_x, saved_y) = stdout.cursor_pos().unwrap();

    // Move cursor to the specified position and print the text
    write!(stdout, "{}{}", Goto(x, y), text).unwrap();

    // Clear the rest of the line after the printed text
    write!(stdout, "{}{}", Goto(0, y + 1), CurrentLine).unwrap();

    // Restore cursor to the saved position
    write!(stdout, "{}{}", Goto(saved_x, saved_y), termion::cursor::Show).unwrap();

    // Flush the output
    stdout.flush().unwrap();
}
