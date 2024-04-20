mod process;
mod screen;
mod kb;

use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread; // For system sleep

const BATCH: usize = 4;

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

    let batches_res: usize = num_process % BATCH;
    let batches_cos: usize = num_process / BATCH;
    let batches: usize = batches_cos + if batches_res != 0 { 1 } else { 0 };

    h_line(30);
    println!("Bacthes: {batches}\n");
    h_line(30);


    let mut processes: Vec<process::Process> = Vec::new();
    let mut execution: Vec<process::Process> = Vec::new();
    let mut finished: Vec<process::Process> = Vec::new();

    for _ in 0..num_process {
        processes.push(process::Process::new());
    }

    screen::pause();
    screen::clear();

    let mut p_end;

    for i in 0..batches {
        let p_beg = i * BATCH;

        if i == batches - 1 && batches_res > 0 {
            p_end = p_beg + batches_res;
        } else {
            p_end = p_beg + BATCH;
        }

        let mut n = 0; // Processes counter (lenght) in execution vector
        let mut time_left: u8 = 0; //
        let mut time_elapsed: u8 = 0;

        for j in p_beg..p_end {
            execution.push(processes[j].clone());
            time_left += execution[n].get_exe_time();
            n += 1;
        }

        let mut j = 0;
        let exe_len = execution.len();

        // Process printing
        while j < exe_len {
            screen::clear();
            // Batch in execution
            batch_in_exe(i, batches, time_left - time_elapsed);
            time_elapsed += execution[j].get_exe_time();
            finished.push(execution[j].clone());

            // Process of batch in execution
            println!("ID\tMax Estimated Time");
            for k in j..n {
                process_in_exe(execution[k].get_id(), execution[k].get_exe_time());
            }

            // Process in execution
            h_line(55);
            println!("Process in execution:");
            h_line(55);
            println!("ID\tOperation\tElapsed Time\tBath Time Left");
            current_process(&execution[j]);

            let key = kb::bind();

            match key {
                'w' => {
                    execution[j].set_result("Error");
                    finished[p_beg + j].set_result("Error");
                },
                'e' => {
                    time_elapsed -= execution[j].get_exe_time();

                    execution.splice(exe_len..exe_len , vec![execution[j].clone()]);
                    execution.remove(j);

                    finished.remove(j);
                    continue;
                },
                _ => thread::sleep(Duration::from_secs(*execution[j].get_exe_time() as u64)), // case 'c' too
            }

            j += 1;

            current_process_times(time_elapsed, time_left - time_elapsed);

            // Process executed/finished
            h_line(55);
            println!("Processes finished");
            for (k, fin) in finished.iter().enumerate() {
                if (k) % 4 == 0 {
                    h_line(35);
                    finished_batch((k) / 4, batches);
                    h_line(35);
                    println!("ID\tOperation\tResult");
                }
                finished_processes(fin);
            }

            h_line(35);
            screen::pause(); 
        }

        execution.clear(); // empty the vector
    }

    let duration = start.elapsed();
    h_line(35);
    println!("Processes Execution Time: {} s", unsafe { *process::Process::get_p_exe_time() });
    println!("Program Execution Time: {:?}", duration);
}

fn h_line(times: usize){
    for _ in 0..times {
        print!("-");
    }
    println!();
}

fn batch_in_exe(i: usize, batches: usize, time: u8) {
    h_line(30);
    println!("Batch in execution: {} of {}", i+1, batches);
    println!("Estimated execution time: {} s", time);
    h_line(30);
}

fn process_in_exe(id: &u16, time: &u8) {
    println!("{}\t{}", id, time);
}

fn finished_batch(i: usize, batches: usize) {
    println!("Batch: {} of {}", i+1, batches);
}

fn finished_processes(p: &process::Process) {
    println!("{}\t{}\t\t{}", p.get_id(), p.get_math_exp(), p.get_result());
}

fn current_process(p: &process::Process) {
    print!("{}\t{}", p.get_id(), p.get_math_exp());
    io::stdout().flush().expect("Failed to flush stdout");
}

fn current_process_times(time_1: u8, time_2: u8){
    println!("\t\t{} s\t\t{} s", time_1, time_2);
}
