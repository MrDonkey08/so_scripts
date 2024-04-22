import os
import sys
import shutil


def clear():
    if sys.platform.startswith("win"):
        os.system("cls")
    else:
        os.system("clear")


def pause():
    input("Press <enter> to continue.")


def gotoxy(x, y):
    # ANSI escape codes for cursor posing
    print(f"\033[{y};{x}H", end="")


def save_cursor_pos():
    print("\033[s", end="")


def restore_cursor_pos():
    print("\033[u", end="")


def print_top_right(text):
    term_width = shutil.get_terminal_size().columns
    x_top_right = term_width - len(text) + 1

    save_cursor_pos()
    gotoxy(x_top_right, 1)
    print(text)
    restore_cursor_pos()
