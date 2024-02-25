use std::process::Command;
use std::io::{self, Write};

pub fn sys_clear() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

pub fn sys_pause() {
    println!("Press Enter to continue...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}
