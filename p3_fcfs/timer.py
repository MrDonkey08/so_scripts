import random


class Times:
    def __init__(self):
        self.estimated: int = random.randint(5, 18)
        self.elapsed: int = 0
        self.blocked: int = 0
        self.arrive: int = 0
        self.finished: int = 0
        self.response: int = 0
        self.returned: int = 0
        self.waiting: int = 0
        self.service: int = 0

    # Getters
    def get_estimated(self) -> int:
        self.estimated

    def get_elapsed(self) -> int:
        self.elapsed

    def get_blocked(self) -> int:
        self.blocked

    def get_arrive(self) -> int:
        self.arrive

    def get_finished(self) -> int:
        self.finished

    def get_response(self) -> int:
        self.response

    def get_returned(self) -> int:
        self.returned

    def get_waiting(self) -> int:
        self.waiting

    def get_service(self) -> int:
        self.service

    # Setters
    def set_estimated(self, value: int):
        self.estimated = value

    def set_elapsed(self, value: int):
        self.elapsed = value

    def set_blocked(self, value: int):
        self.blocked = value

    def set_arrive(self, value: int):
        self.arrive = value

    def set_finished(self, value: int):
        self.finished = value

    def set_response(self, value: int):
        self.response = value

    def set_returned(self, value: int):
        self.returned = value

    def set_waiting(self, value: int):
        self.waiting = value

    def set_service(self, value: int):
        self.service = value
