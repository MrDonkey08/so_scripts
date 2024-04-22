import sys
import time
from typing import List

# MY files
from process import Process
import screen
import kb

READY_MAX: int = 4


def main():
    screen.clear()

    print("Welcome to the Process Simulator\n")
    num_process = get_num_processes()

    batches_res = num_process % READY_MAX
    batches_cos = num_process / READY_MAX
    batches = int(batches_cos + (1 if batches_res != 0 else 0))

    h_line(30)
    print(f"Batches: {batches}")
    h_line(30)

    new = [Process() for _ in range(num_process)]
    blocked: List[Process] = []
    finished: List[Process] = []

    print(f"{new[0].time.get_elapsed()}")

    i = 0
    ready_end = READY_MAX + 1
    interrupted = False

    screen.pause()
    screen.clear()

    s_start_time = time.time()

    while i < num_process:
        screen.clear()
        # time_elapsed = 0
        # time_left = 0

        if num_process - i < READY_MAX + 1:
            ready_end = num_process - 1

        ready_end -= 1 if interrupted else 0

        f = i + ready_end

        # time_left += execution[n].time.get_estimated()

        # Process printing
        key = 0

        h_line(50)
        print("Ready Processes")
        h_line(50)
        print("ID\tEstimated Time\tElapsed Time")

        for j in range(i + 1, f):
            process_ready(new[j])

        # Process in Execution
        h_line(70)
        print("Process in execution")
        h_line(70)
        print("ID\tOperation\tEstimated Time\tElapsed Time\tRemaining Time")
        current_process(new[i])

        key, elapsed = kb.bind(new[i].time.get_estimated())
        """p_time = execution[j].time.get_estimated()

        # Time Update
        remaining = p_time - elapsed
        execution[j].add_elapsed_time(elapsed)
        execution[j].set_exe_time(remaining)
        time_elapsed += elapsed"""

        if key == "w":
            """time_elapsed += remaining
            Process.add_p_exe_time(-remaining)
            remaining = 0
            execution[j].set_exe_time(0)"""
            new[i].set_result("Error")
        elif key == "e":
            interrupted = True
            # execution.insert(n, execution[j])
            blocked.append(new[i])
            new.pop(j)
            # finished.pop(p_beg + j)

        if not interrupted:
            finished.push(new[i])
            i += 1

        # current_process_times(elapsed, remaining)
        h_line(70)

        # Finished / Executed Processes
        h_line(70)
        print("Finished Processes")
        h_line(70)
        print("ID\tOperation\tResult")

        for fin in finished:
            finished_process(fin)

        h_line(35)
        print("Blocked Processes")
        h_line(35)
        print("ID\tTime Elapsed")

        for block in blocked:
            blocked_process(block)

        if not blocked.is_empty():
            for j in range(blocked.len()):
                if blocked[j].time.get_blocked() > blocked.time.get_estimated():
                    new.insert(f - 1, blocked[0])
                    blocked.pop(0)

        h_line(35)
        screen.pause()

    s_duration = time.time() - s_start_time
    p_duration = time.time() - p_start_time
    h_line(35)
    # print(f"Processes Execution Time: {Process.get_p_exe_time()} s")
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


def process_ready(p: Process):
    print(f"{p.get_id()}\t{p.time.get_estimated()}\t\t{p.time.get_elapsed()}")


def finished_batch(i: int, batches: int):
    print(f"Batch: {i+1} of {batches}")


def finished_process(p: Process):
    print(f"{p.get_id()}\t{p.get_math_exp()}\t\t{p.get_result()}")


def blocked_process(p: Process):
    print(p.get_id())


def current_process(p: Process):
    print(f"{p.get_id()}\t{p.get_math_exp()}\t\t{p.time.get_estimated()} s", end="")
    sys.stdout.flush()


def current_process_times(elapsed: int, remaining: int):
    print(f"\t\t{elapsed} s\t\t{remaining} s")


if __name__ == "__main__":
    p_start_time = time.time()
    main()
