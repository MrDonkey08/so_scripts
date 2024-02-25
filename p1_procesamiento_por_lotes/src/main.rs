mod process;
mod screen;
mod reg_cal;

use rand::Rng;
use std::io:: {self, Write};
use std::time:: {Instant};
use std:: {thread, time}; // For system sleep

const LOTE: usize = 4;

fn main() {
    let start = Instant::now();

    let mut num_process;
    let int_num_process: usize;

    loop {
        println!("Welcome to the Process Simulator\n");
        print!("Enter the number of processes to execute: ");
        io::stdout().flush().expect("Failed to flush stdout");

        num_process = String::new();
        io::stdin()
            .read_line(&mut num_process)
            .expect("Failed to read number");

        int_num_process = match num_process.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please, enter a valid integer");
                screen::sys_pause();
                screen::sys_clear();
                continue; // Continue to the next iteration of the loop
            }
        };
        break;
    }

    let batches_res : usize = int_num_process % LOTE;
    let batches_cos : usize = int_num_process / LOTE;
    let batches : usize = batches_cos + if batches_res != 0 { 1 } else { 0 };

    println!("Bacth: {batches}");

    let mut processes : Vec<process::Process> = Vec::new();

    for n in 0..int_num_process {
        processes.push(process::Process::new(String::from(""), String::from(""), String::from(""), String::from(""), 0));

        let mut used_id = false;
        let mut seted_exe_time = false;

        loop {
            println!("Process: {} of {}", n+1, num_process);

            if processes[n].get_id().is_empty() {
                print!("ID: ");
                io::stdout().flush().expect("Failed to flush stdout");
                processes[n].set_id();

                for m in 0..n {
                    if n > 0 && processes[n].get_id() == processes[m].get_id() {
                        println!("ID already used. Try again");
                        used_id = true;
                        processes[n].set_id_empty();
                        break;
                    }
                }
            }

            if  !seted_exe_time {
                let int_aux;

                print!("Estimated execution time: ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut aux = String::new();

                io::stdin()
                    .read_line(&mut aux)
                    .expect("Failed to read number");
        
                int_aux = match aux.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please, enter a valid integer");
                        screen::sys_pause();
                        screen::sys_clear();
                        continue;
                    }
                };
                processes[n].set_exe_time(int_aux);
                seted_exe_time = true;
            }

            if processes[n].get_username().is_empty() {
                print!("Programmer name: ");
                io::stdout().flush().expect("Failed to flush stdout");
                processes[n].set_username();
            }

            if processes[n].get_math_exp().is_empty() {
                print!("Math Expression: ");
                io::stdout().flush().expect("Failed to flush stdout");
                processes[n].set_math_exp();
                processes[n].calculate_ans_exp();

                if processes[n].get_ans_exp().is_empty() {
                    processes[n].set_math_exp_empty();
                }
            }

            if processes[n].input_empty() {
                println!("Empty field detected. Please try again.");
                screen::sys_pause();
                screen::sys_clear();
                continue; // Continue to the next iteration of the loop
            }

            processes[n].set_exe_time(rand::thread_rng().gen_range(2..=5));
            break;
        }
    }

    screen::sys_pause();
    screen::sys_clear();

    let mut time: Vec<u64> = Vec::new();
    let mut j = 0; // int_num_process index
    let mut k = 0; // 
    let mut l = 0; // index for run batches
    let mut m = batches-1; // descending index
    let mut n = LOTE-1;
    let mut lote = LOTE;

    for i in 0..int_num_process {
        time.push(0);
    }


    for i in 0..batches {
        
        if m == 0 {
            lote = batches_res;
        }

        for j in k..lote+k {
            time[i] += processes[j].get_exe_time();
            working_batch(&time, batches, i);
            for k in (n..=0).rev() {
                batch_in_exe(&mut processes, k);
            }
            h_line();

            working_processes(&mut processes, &mut time, i, j);
            h_line();

            for l in 0..i {
                if i % 4 == 0 {
                    finished_batch(batches, i);
                }
                finished_processes(&processes, j);
            }

            h_line();
            screen::sys_pause();
            screen::sys_clear();

        }

        if m > 0 {
            m -= 1;
        }

        if n > 0 {
            n -= 1;
        }


    }

    let duration = start.elapsed();
    h_line();
    println!("Program Execution Time: {:?}", duration);
}

fn h_line(){
    println!("---------------------");
}

fn working_batch(times: &Vec<u64>, batches : usize, i : usize) {
    println!("Batch in execution: {} de {}\n", i+1, batches);
    println!("Estimated execution time {}s\n\n", times[i]);
}

fn batch_in_exe(arr: &Vec<process::Process>, j : usize){
    println!("Program (ID): {}", arr[j].get_id());
    println!("Estimated execution time {}s", (arr[j].get_exe_time()));
}

fn finished_batch(batches : usize, i : usize) {
    println!("Batch: {} de {}\n", i+1, batches);
}

fn finished_processes(arr : &Vec<process::Process>, j : usize) {
    println!("Nombre: {}", arr[j].get_username());
    println!("Operation: {} = {}\n\n", arr[j].get_math_exp(), arr[j].get_ans_exp());
}

fn working_processes(arr : &mut Vec<process::Process>, times : &mut Vec<u64>, i : usize, j : usize) {
    let start = Instant::now();
    arr[j].calculate_ans_exp();
    println!("Program (ID): {}", arr[j].get_id());
    println!("Nombre: {}", arr[j].get_username());
    println!("Operation: {} = {}", arr[j].get_math_exp(), arr[j].get_ans_exp());
    thread::sleep(time::Duration::from_secs(arr[j].get_exe_time()-1));
    let duration = start.elapsed();
    times[i] -= arr[j].get_exe_time();
    println!("Time elapsed: {:?}", duration);
    println!("Time left: {}s\n\n", times[i]);
}
