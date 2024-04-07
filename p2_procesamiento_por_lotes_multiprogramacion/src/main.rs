mod process;
mod screen;

use std::io:: {self, Write};
use std::time:: {Instant};
use std:: {thread, time}; // For system sleep

const BATCH: usize = 4;

fn main() {
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

    h_line_1();
    println!("Bacthes: {batches}\n");
    h_line_1();

    let mut processes: Vec<process::Process> = Vec::new();

    for n in 0..num_process {
        processes.push(process::Process::new());
    }

    screen::pause();
    screen::clear();

    let mut execution: Vec<process::Process> = Vec::new();
    let mut finished: Vec<process::Process> = Vec::new();

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

        // Process printing
        for (j, exe) in execution.iter().enumerate() {
            screen::clear();
            // Batch in execution
            batch_in_exe(i, batches, time_left - time_elapsed);
            time_elapsed += execution[j].get_exe_time();
            finished.push(execution[j].clone());

            // Process of batch in execution
            for k in j..n {
                process_in_exe(execution[k].get_id(), execution[k].get_exe_time());
            }

            // Process in execution
            h_line_1();
            println!("Process in execution:");
            h_line_1();
            current_process(exe, time_elapsed, time_left - time_elapsed);

            // Process executed/finished
            h_line_1();
            println!("Processes finished");
            for (k, fin) in finished.iter().enumerate() {
                if (k) % 4 == 0 {
                    h_line_1();
                    finished_batch((k) / 4, batches);
                    h_line_2();
                }
                finished_processes(fin);
            }

            h_line_1();
            screen::pause(); 
        }

        execution.clear(); // empty the vector
    }

    let duration = start.elapsed();
    h_line_1();
    println!("Processes Execution Time: {} s", process::Process::get_p_exe_time());
    println!("Program Execution Time: {:?}", duration);
}

fn h_line_1(){
    println!("------------------------------");
}

fn h_line_2(){
    println!("----------------");
}

fn batch_in_exe(i: usize, batches: usize, time: u8) {
    h_line_1();
    println!("Batch in execution: {} of {}", i+1, batches);
    println!("Estimated execution time: {} s", time);
    h_line_1();
}

fn process_in_exe(id: &u16, time: &u8) {
    println!("Program (ID): {}", id);
    println!("Estimated execution time: {} s\n", time);
}

fn finished_batch(i: usize, batches: usize) {
    println!("Batch: {} of {}", i+1, batches);
}

fn finished_processes(p: &process::Process) {
    println!("Operation: {} = {}\n", p.get_math_exp(), p.get_result());
}

fn current_process(p: &process::Process, time_1: u8, time_2: u8) {
    println!("Program (ID): {}", p.get_id());
    println!("Operation: {}", p.get_math_exp());
    thread::sleep(time::Duration::from_secs(*p.get_exe_time() as u64));
    println!("Time elapsed: {} s", time_1);
    println!("Time left: {} s\n", time_2);
}
