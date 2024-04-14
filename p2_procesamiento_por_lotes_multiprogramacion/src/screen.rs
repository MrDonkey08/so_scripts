use std::process::Command;
use std::io::{self, stdout, Write};
use termion::cursor::{Goto, Hide, Show};
use termion::raw::IntoRawMode;

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
    // Get stdout and enter raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Hide the cursor
    write!(stdout, "{}", Hide).unwrap();

    // Save current cursor position
    write!(stdout, "{}", Goto(x, y)).unwrap();

    // Print the text at the specified position
    write!(stdout, "{}", text).unwrap();

    // Restore cursor to previous position
    write!(stdout, "{}", Goto(x, y + 1)).unwrap(); // Move cursor one line down
    write!(stdout, "{}", Show).unwrap(); // Show the cursor again

    // Flush the output
    stdout.flush().unwrap();
}
