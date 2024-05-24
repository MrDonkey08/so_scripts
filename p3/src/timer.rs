use rand::Rng;


#[derive(Default, Debug, Clone)]
pub struct Times { 
    estimated: u8,
    elapsed: u8,
    blocked: u8,
    arrive: u8,
    finished: u8,
    response: u8,
    returned: u8,
    waiting: u8,
    service: u8,
}

impl Times { 
    pub fn new() -> Self { 
        let mut rng = rand::thread_rng();
        Self { 
            estimated: rng.gen_range(5..=18),
            elapsed: 0,
            blocked: 0,
            arrive: 0,
            finished: 0,
            response: 0,
            returned: 0,
            waiting: 0,
            service: 0,
        }
    }

    // Getters
    pub fn get_estimated(&self) -> u8 { self.estimated }
    pub fn get_elapsed(&self) -> u8 { self.elapsed }
    pub fn get_blocked(&self) -> u8 { self.blocked }
    pub fn get_arrive(&self) -> u8 { self.arrive }
    pub fn get_finished(&self) -> u8 { self.finished }
    pub fn get_response(&self) -> u8 { self.response }
    pub fn get_returned(&self) -> u8 { self.returned }
    pub fn get_waiting(&self) -> u8 { self.waiting }
    pub fn get_service(&self) -> u8 { self.service }

    // Setters
    pub fn set_estimated(&mut self, value: u8) { self.estimated = value; }
    pub fn set_elapsed(&mut self, value: u8) { self.elapsed = value; }
    pub fn set_blocked(&mut self, value: u8) { self.blocked = value; }
    pub fn set_arrive(&mut self, value: u8) { self.arrive = value; }
    pub fn set_finished(&mut self, value: u8) { self.finished = value; }
    pub fn set_response(&mut self, value: u8) { self.response = value; }
    pub fn set_returned(&mut self, value: u8) { self.returned = value; }
    pub fn set_waiting(&mut self, value: u8) { self.waiting = value; }
    pub fn set_service(&mut self, value: u8) { self.service = value; }
}


