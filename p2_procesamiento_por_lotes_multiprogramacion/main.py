import random
import time
import os
from typing import List

import process
import screen

LOTE = 4

def main():
    start = time.time()

    while True:
        print("Welcome to the Process Simulator\n")
        num_process_str = input("Enter the number of processes to execute: ")
        try:
            int_num_process = int(num_process_str)
            break
        except ValueError:
            print("Invalid input. Please enter a valid integer")
            screen.pause()
            screen.clear()
            continue

    batches_res = int_num_process % LOTE
    batches_cos = int_num_process // LOTE
    batches = batches_cos + (1 if batches_res != 0 else 0)

    print(f"Batch: {batches}")

    processes: List[process.Process] = []

    for n in range(int_num_process):
        processes.append(process.Process('', '', '', '', '', '', '', '', ''))
        used_id = False
        seted_exe_time = False

        while True:
            print(f"Process: {n + 1} of {num_process_str}")

            if processes[n].get_id() == "":
                print("ID: ", end='')
                processes[n].set_id()
                for m in range(n):
                    if n > 0 and processes[n].get_id() == processes[m].get_id():
                        print("ID already used. Try again")
                        used_id = True
                        processes[n].set_id_empty()
                        break

            if not seted_exe_time:
                try:
                    int_aux = int(input("Estimated execution time: "))
                except ValueError:
                    print("Invalid input. Please enter a valid integer")
                    screen.pause()
                    screen.clear()
                    continue
                processes[n].set_exe_time(int_aux)
                seted_exe_time = True

            if processes[n].get_username() == "":
                print("Username: ", end='')
                processes[n].set_username()

            if processes[n].get_result() == "": 
                print("Número 1: ", end='')
                processes[n].set_num_1()
                print("Número 2: ", end='')
                processes[n].set_num_2()

                if processes[n].num_1 != "" and processes[n].num_2 != "":
                    print("Operation\n")
                    print("1. Suma")
                    print("2. Resta")
                    print("3. Multiplicación")
                    print("4. División")
                    print("5. Módulo\n")
                    print("Opción: ", end='')
                    print()

                    processes[n].set_operation()
                    processes[n].set_result()

            if processes[n].input_empty():
                print("\nEmpty field detected. Please try again.")
                screen.pause()
                screen.clear()
                continue

            print()
            processes[n].set_exe_time(random.randint(2, 5))
            break

    screen.pause()
    screen.clear()

    screen.clear()

    k = 0
    l = 0
    time_list = []

    for i in range(batches):
        # Calculation of Batch Times
        time_list.append(0)

        if l < int_num_process - batches_res:
            for j in range(l, LOTE+k):
                time_list[i] += processes[j].get_exe_time()
                l = j + 1
        elif batches_res > 0:
            for j in range(l, int_num_process):
                time_list[i] += processes[j].get_exe_time()
                l = j + 1

        # Current Batch
        working_batch(time_list, batches, i)

        # Processes in Execution
        if k < int_num_process - batches_res:
            for j in range(k, LOTE+k):
                working_processes(processes, time_list, i, j)
                k = j + 1
        elif batches_res > 0:
            for j in range(k, int_num_process):
                working_processes(processes, time_list, i, j)
                k = j + 1

        h_line()

    screen.pause()
    screen.clear()

    k = 0

    print("Finished batches")

    for i in range(batches):
        finished_batch(batches, i)

        # Processes in Execution
        if k < int_num_process - batches_res:
            for j in range(k, LOTE+k):
                finished_processes(processes, j)
                k = j + 1
        elif batches_res > 0:
            for j in range(k, int_num_process):
                finished_processes(processes, j)
                k = j + 1

        print("-----------------------------------------------------------------------")

    duration = time.time() - start

    print("Program Execution Time:", duration)


def h_line():
    print("---------------------")

def working_batch(times, batches, i):
    print(f"Batch in execution: {i + 1} of {batches}\n")
    print(f"Estimated execution time: {times[i]}s\n\n")

def batch_in_exe(arr, j):
    print(f"Program (ID): {arr[j].get_id()}")
    print(f"Estimated execution time: {arr[j].get_exe_time()}s")

def finished_batch(batches, i):
    print(f"Batch: {i + 1} of {batches}\n")

def finished_processes(arr, j):
    print(f"Name: {arr[j].get_username()}")
    print(f"Operation: {arr[j].get_math_exp()} = {arr[j].get_result()}\n\n")

def working_processes(arr, times, i, j):
    start = time.time()
    arr[j].set_result()
    print(f"Program (ID): {arr[j].get_id()}")
    print(f"Name: {arr[j].get_username()}")
    arr[j].set_math_exp()
    print(f"Operation: {arr[j].get_math_exp()} = {arr[j].get_result()}")
    time.sleep(arr[j].get_exe_time() - 1)
    duration = time.time() - start
    times[i] -= arr[j].get_exe_time()
    print(f"Time elapsed: {duration}")
    print(f"Time left: {times[i]}s\n\n")

if __name__ == "__main__":
    main()
