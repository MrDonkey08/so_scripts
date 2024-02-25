class Process:
    def __init__(self, id, username, num_1, num_2, operation, result, math_exp, operator, exe_time):
        self.id = id
        self.username = username
        self.num_1 = num_1
        self.num_2 = num_2
        self.operation = operation
        self.result = operation
        self.math_exp = math_exp
        self.exe_time = exe_time

    def input_empty(self):
        return self.id == '' or self.username == '' or self.operation == '' or self.result == '' or self.exe_time == ''

    def get_id(self):
        return self.id

    def set_id(self):
        self.id = input().strip()

    def set_id_empty(self):
        self.id = ''

    def get_username(self):
        return self.username

    def set_username(self):
        self.username = input().strip()

    def get_num_1(self):
        return self.num_1

    def set_num_1(self):
        self.num_1 = input()
        if self.num_1 != "":
            self.num_1 = int(self.num_1)
    
    def get_num_2(self):
        return self.num_2
    
    def set_num_2(self):
        self.num_2 = input()
        if self.num_2 != "":
            self.num_2 = int(self.num_2)

    def get_operation(self):
        return self.operation

    def set_operation(self):
        self.operation = input()
        if self.operation != "":
            self.operation = int(self.operation)
    
    def get_result(self):
        return self.result

    def set_result(self):
        match self.operation:
            case 1:
                self.result = self.num_1 + self.num_2
                self.operator = "+"

            case 2:
                self.result = self.num_1 - self.num_2
                self.operator = "-"

            case 3:
                self.result = self.num_1 * self.num_2
                self.operator = "*"

            case 4 | 5:
                if self.num_2 != 0:
                    match self.operation:

                        case 4:
                            self.result = float(self.num_1) / float(self.num_2)
                            self.operator = "/"

                        case 5:
                            self.result = float(self.num_1) % float(self.num_2)
                            self.operator = "%"
                else:
                    print("Indeterminacy error: not possible to divide by 0")

            case _:
                print("Invalid Operation. Try Again.")

    def set_math_exp(self):
        self.math_exp = str(self.num_1) + self.operator + str(self.num_2)

    def get_math_exp(self):
        return self.math_exp

    def get_exe_time(self):
        return self.exe_time

    def set_exe_time(self, value):
        self.exe_time = value
