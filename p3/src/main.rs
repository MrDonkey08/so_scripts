mod process;
mod screen;
mod kb;
mod timer;

use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread; // For system sleep

const READY_MAX: usize = 4;

fn main() {
    screen::clear();

    let start = Instant::now();
    let num_process: usize;

    loop {
        println!("Welcome to the Process Simulator\n");
        print!("Enter the number of processes to execute: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut n_process = String::new();
        io::stdin()
            .read_line(&mut n_process)
            .expect("Failed to read number");

        num_process = match n_process.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please, enter a valid integer");
                screen::pause();
                screen::clear();
                continue; // Continue to the next iteration of the loop
            }
        };

        break;
    }

    let mut new: Vec<process::Process> = Vec::new();
    let mut blocked: Vec<process::Process> = Vec::new();
    let mut finished: Vec<process::Process> = Vec::new();

    for _ in 0..num_process {
        new.push(process::Process::new());
    }

    let mut i = 0;
    let mut ready_end = READY_MAX + 1;

    screen::pause();
    screen::clear();

    let mut interrupted = false;

    while i < num_process {
        screen::clear();
        //let mut time_left: u8 = 0; //
        //let mut time_elapsed: u8 = 0;

        if num_process - i < READY_MAX + 1 {
            ready_end = num_process - i;
        }

        ready_end -= if interrupted {1} else {0};

        let f = i + ready_end;

        // Process printing
        //time_elapsed += execution[j].get_exe_time();
        h_line(50);
        println!("Ready Processes");
        h_line(50);
        println!("ID\tMax Estimated Time\tTime Elapsed");

        for j in i+1..f {
            process_ready(&new[j]);
        }

        h_line(55);
        println!("Process in execution:");
        h_line(55);
        println!("ID\tOperation\tElapsed Time\tBath Time Left");
        current_process(&new[i]);

        interrupted = false;

        let start_p = Instant::now();
        let key = kb::bind();

        if key == 'x' {
            let key = kb::bind();
            println!("{key}");
            match key {
                'w' => new[i].set_result("Error"),
                'e' => {
                    interrupted = true;
                    //new.splice(f..f, vec![new[i].clone()]);
                    blocked.push(new[i].clone());
                    new.remove(i);
                },
                _ => {}//thread::sleep(Duration::from_secs(*new[i].get_exe_time() as u64)), // case 'c' too
            }
        }

        let duration_p = start.elapsed().as_secs();

        if !interrupted {
            finished.push(new[i].clone());
            i += 1;
        }

        //current_process_times(time_elapsed, time_left - time_elapsed);

        println!();
        h_line(55);
        println!("Finished processes");
        h_line(35);
        println!("ID\tOperation\tResult");

        for fin in finished.iter() {
            finished_process(fin);
        }

        h_line(35);

        // Blocked Processes
        h_line(35);
        println!("Blocked Processes");
        h_line(35);
        println!("ID\tTime Elapsed");

        for block in blocked.iter() {
            blocked_process(block);
        }

        if !blocked.is_empty() {
            for j in 0..blocked.len() {
                if blocked[j].time.get_blocked() > blocked[j].time.get_estimated() {
                    new.splice(f-1..f-1, vec![blocked[0].clone()]);
                    blocked.remove(0);
                    ready_end += 1
                }
            }

        }

        screen::pause(); 
    }

    let duration = start.elapsed().as_secs();
    h_line(35);
    //println!("Processes Execution Time: {} s", unsafe { *process::Process::get_p_exe_time() });
    println!("Program Execution Time: {} s", duration);
}

fn h_line(times: usize){
    for _ in 0..times {
        print!("-");
    }
    println!();
}

fn process_ready(p: &process::Process) {
    println!("{}\t{}", p.get_id(), p.time.get_estimated());
}

fn finished_process(p: &process::Process) {
    println!("{}\t{}\t\t{}", p.get_id(), p.get_math_exp(), p.get_result());
}

fn blocked_process(p: &process::Process) {
    println!("{}", p.get_id());
}

fn current_process(p: &process::Process) {
    print!("{}\t{}", p.get_id(), p.get_math_exp());
    io::stdout().flush().expect("Failed to flush stdout");
}

fn current_process_times(time_1: u8, time_2: u8){
    println!("\t\t{} s\t\t{} s", time_1, time_2);
}
