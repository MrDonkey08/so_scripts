import select
import sys
import tty
import termios
import time

# My files
import screen


def kbhit(time_target, previous_time):
    def is_key_pressed():
        return select.select([sys.stdin], [], [], 0) == ([sys.stdin], [], [])

    # Set terminal to raw mode
    old_settings = termios.tcgetattr(sys.stdin)
    try:
        tty.setcbreak(sys.stdin.fileno())

        key = 0
        start = time.time()
        duration = 0

        while key == 0 and duration < time_target:
            duration = time.time() - start + previous_time
            if is_key_pressed():
                key = sys.stdin.read(1)

        # print(f"{duration}")

    finally:
        # Restore terminal settings
        termios.tcsetattr(sys.stdin, termios.TCSADRAIN, old_settings)

    return key, duration


def print_status(text):
    screen.print_top_right(text)

    if text != "Paused":
        spaces = " " * len(text)
        time.sleep(1)
        screen.print_top_right(spaces)


def bind(time_target):
    key = 0
    duration = 0
    previous_time = 0

    while duration < time_target:
        key, duration = kbhit(time_target, previous_time)

        if key == "p":
            print_status("Paused")
            previous_time = duration

            while True:
                key, _ = kbhit(time_target, previous_time)
                if key == "c":
                    print_status("Continue")
                    break

            continue
        elif key == "w":
            print_status("Error")
        elif key == "e":
            print_status("Interruption")
        else:
            continue

        break

    sys.stdout.flush()
    return key, int(duration)
