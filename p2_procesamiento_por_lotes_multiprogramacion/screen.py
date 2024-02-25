import os

def clear():
    os.system("cls" if os.name == "nt" else "clear")

def pause():
    print("Press <enter> to continue...")
    input()
