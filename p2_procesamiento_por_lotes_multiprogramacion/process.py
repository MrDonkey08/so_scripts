import random
from typing import Optional

P_EXE_TIME: int = 0


class Process:
    def __init__(self):
        self.id: int = self.gen_id()
        self.num_1: int = random.randint(0, 100)
        self.num_2: int = random.randint(0, 100)
        self.operation: str = random.choice(["+", "-", "*", "/", "%"])
        self.math_exp: str = ""
        self.result: str = ""
        self.exe_time: int = random.randint(5, 18)
        self.elapsed_time: int = 0

        global P_EXE_TIME
        P_EXE_TIME += self.get_exe_time()
        self.gen_math_exp()
        self.gen_result()

    @staticmethod
    def gen_id() -> int:
        global ID_COUNTER
        ID_COUNTER += 1
        return ID_COUNTER

    def gen_number_2(self):
        self.num_2 = random.randint(0, 100)

    def gen_math_exp(self):
        self.math_exp = f"{self.num_1}{self.operation}{self.num_2}"

    def gen_result(self):
        while self.num_2 == 0 and (self.operation == "/" or self.operation == "%"):
            self.gen_number_2()
            self.gen_math_exp()

        try:
            if self.operation == "+":
                result = float(self.num_1 + self.num_2)
            elif self.operation == "-":
                result = float(self.num_1 - self.num_2)
            elif self.operation == "*":
                result = float(self.num_1 * self.num_2)
            elif self.operation == "/":
                result = float(self.num_1 / self.num_2)
            elif self.operation == "%":
                result = float(self.num_1 % self.num_2)
            else:
                raise ValueError("Invalid operation. Try again")
        except ZeroDivisionError:
            print("Error: Division by zero.")

        self.result = str(round(result, 5))

    def get_id(self) -> int:
        return self.id

    def get_math_exp(self) -> str:
        return self.math_exp

    def get_result(self) -> str:
        return self.result

    def get_exe_time(self) -> int:
        return self.exe_time

    def get_elapsed_time(self) -> int:
        return self.elapsed_time

    @staticmethod
    def get_p_exe_time() -> Optional[int]:
        global P_EXE_TIME
        return P_EXE_TIME

    def set_result(self, res: str):
        self.result = res

    def set_exe_time(self, var):
        self.exe_time = var

    def add_elapsed_time(self, var):
        self.elapsed_time += var

    def add_p_exe_time(var) -> Optional[int]:
        global P_EXE_TIME
        P_EXE_TIME += var


# Initializing the ID_COUNTER
ID_COUNTER: int = 0
