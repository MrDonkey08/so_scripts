import sys
import time
from typing import List

# MY files
from process import Process
import screen
import kb

BATCH: int = 4


def main():
    screen.clear()

    print("Welcome to the Process Simulator\n")
    num_process = get_num_processes()

    batches_res = num_process % BATCH
    batches_cos = num_process / BATCH
    batches = int(batches_cos + (1 if batches_res != 0 else 0))

    h_line(30)
    print(f"Batches: {batches}")
    h_line(30)

    processes = [Process() for _ in range(num_process)]
    execution: List[Process] = []
    finished: List[Process] = []

    screen.clear()

    s_start_time = time.time()

    for i in range(batches):
        p_beg = i * BATCH

        if i == batches - 1 and batches_res > 0:
            p_end = p_beg + batches_res
        else:
            p_end = p_beg + BATCH

        time_elapsed = 0
        time_left = 0
        n = 0

        for j in range(p_beg, p_end):
            execution.append(processes[j])
            time_left += execution[n].get_exe_time()
            n += 1

        j = 0

        # Process printing
        while j < n:
            key = 0
            screen.clear()
            # Batch in execution
            batch_in_exe(i, batches, time_left - time_elapsed)
            finished.append(execution[j])

            # Process of batch in execution
            print("ID\tEstimated Time\tElapsed Time")
            for k in range(j + 1, n):
                process_in_exe(execution[k])

            # Process in Execution
            h_line(70)
            print("Process in execution")
            h_line(70)
            print("ID\tOperation\tEstimated Time\tElapsed Time\tRemaining Time")
            current_process(execution[j])

            p_time = execution[j].get_exe_time()
            key, elapsed = kb.bind(p_time)

            # Time Update
            remaining = p_time - elapsed
            execution[j].add_elapsed_time(elapsed)
            execution[j].set_exe_time(remaining)
            time_elapsed += elapsed

            if key == "w":
                time_elapsed += remaining
                Process.add_p_exe_time(-remaining)
                remaining = 0
                execution[j].set_exe_time(0)
                finished[p_beg + j].set_result("Error")
                j += 1
            elif key == "e":
                execution.insert(n, execution[j])
                execution.pop(j)
                finished.pop(p_beg + j)
            else:
                j += 1

            current_process_times(elapsed, remaining)
            h_line(70)

            # Finiched / Executed Processes
            print("Finished Processes")
            for k, fin in enumerate(finished):
                if k % 4 == 0:
                    h_line(35)
                    finished_batch(k // 4, batches)
                    h_line(35)
                    print("ID\tOperation\tResult")
                finished_processes(fin)
            h_line(35)
            screen.pause()

        execution.clear()

    s_duration = time.time() - s_start_time
    p_duration = time.time() - p_start_time
    h_line(35)
    print(f"Processes Execution Time: {Process.get_p_exe_time()} s")
    print(f"Simulation Execution Time: {round(s_duration, 2)} s")
    print(f"Program Execution Time: {round(p_duration, 2)} s")
    h_line(35)


def get_num_processes() -> int:
    while True:
        try:
            num_process = int(input("Enter the number of processes to execute: "))
            return num_process
        except ValueError:
            print("Invalid input. Please enter a valid integer")
            screen.pause()
            screen.clear()


def h_line(times: int):
    print("-" * times)


def batch_in_exe(i: int, batches: int, time: int):
    h_line(40)
    print(f"Batch in execution: {i+1} of {batches}")
    print(f"Estimated execution time: {time} s")
    h_line(40)


def process_in_exe(p: Process):
    print(f"{p.get_id()}\t{p.get_exe_time()}\t\t{p.get_elapsed_time()}")


def finished_batch(i: int, batches: int):
    print(f"Batch: {i+1} of {batches}")


def finished_processes(p: Process):
    print(f"{p.get_id()}\t{p.get_math_exp()}\t\t{p.get_result()}")


def current_process(p: Process):
    print(f"{p.get_id()}\t{p.get_math_exp()}\t\t{p.get_exe_time()} s", end="")
    sys.stdout.flush()


def current_process_times(elapsed: int, remaining: int):
    print(f"\t\t{elapsed} s\t\t{remaining} s")


if __name__ == "__main__":
    p_start_time = time.time()
    main()
