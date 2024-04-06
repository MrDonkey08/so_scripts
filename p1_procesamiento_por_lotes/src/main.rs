mod process;
mod screen;
mod reg_cal;

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
                screen::pause();
                screen::clear();
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
    let mut p_time : u64 = 0;

    for n in 0..int_num_process {
        processes.push(process::Process::new(String::from(""), String::from(""), String::from(""), String::from(""), 3));

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
                        screen::pause();
                        screen::clear();
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
                screen::pause();
                screen::clear();
                continue; // Continue to the next iteration of the loop
            }

            //processes[n].set_exe_time(rand::thread_rng().gen_range(2..=5));
            break;
        }*/

        p_time += processes[n].get_exe_time()
    }

    screen::pause();
    screen::clear();

    let mut execution : Vec<process::Process> = Vec::new();
    let mut finished : Vec<process::Process> = Vec::new();

    let mut p_end;

    for i in 0..batches {
        let p_beg = i * LOTE;

        if i == batches - 1 && batches_res > 0 {
            p_end = p_beg + batches_res;
        } else {
            p_end = p_beg + LOTE;
        }

        let mut n = 0; // Processes counter (lenght) in execution vector
        let mut time_left : u64 = 0; //
        let mut time_elapsed : u64 = 0;

        for j in p_beg..p_end {
            execution.push(processes[j].clone());
            time_left += execution[n].get_exe_time();
            n += 1;
        }

        // Process printing
        for (j, exe) in execution.iter().enumerate() {
            screen::clear();
            // Batch in execution
            h_line_1();
            batch_in_exe(i, batches, time_left - time_elapsed);
            time_elapsed += execution[j].get_exe_time();
            finished.push(execution[j].clone());

            // Process of batch in execution
            for k in j..n {
                process_in_exe(execution[k].get_id(), execution[k].get_exe_time());
            }

            h_line_1();
            // Process in execution
            current_process(exe, time_elapsed, time_left - time_elapsed);

            // Process executed/finished
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
    println!("Processes Execution Time: {} s", p_time);
    println!("Program Execution Time: {:?}", duration);
}

fn h_line_1(){
    println!("------------------------------");
}

fn h_line_2(){
    println!("----------------");
}

fn batch_in_exe(i : usize, batches : usize, time : u64) {
    println!("Batch in execution: {} de {}\n", i+1, batches);
    println!("Estimated execution time {} s\n", time);
}

fn process_in_exe(id : &str, time : u64) {
    println!("Program (ID): {}", id);
    println!("Estimated execution time {} s\n", time);
}

fn finished_batch(i : usize, batches : usize) {
    println!("Batch: {} de {}\n", i+1, batches);
}

fn finished_processes(p : &process::Process) {
    println!("Nombre: {}", p.get_username());
    println!("Operation: {} = {}\n", p.get_math_exp(), p.get_ans_exp());
}

fn current_process(p : &process::Process, mut time_1 : u64, mut time_2 : u64) {
    println!("Program (ID): {}", p.get_id());
    println!("Nombre: {}", p.get_username());
    println!("Operation: {} = {}", p.get_math_exp(), p.get_ans_exp());
    thread::sleep(time::Duration::from_secs(p.get_exe_time()));
    println!("Time elapsed: {} s", time_1);
    println!("Time left: {} s\n", time_2);
}
