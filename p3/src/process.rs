use rand::Rng;
use std::ptr::addr_of;

use crate::timer;

static mut P_EXE_TIME: u8 = 0;

#[derive(Default, Debug, Clone)]
pub struct Process {
    id: u16,
    num_1: u32,
    num_2: u32,
    operation: char,
    math_exp: String,
    result: String,
    pub time: timer::Times,
}

impl Process {
    pub fn new() -> Self {

        let mut rng = rand::thread_rng();

        let mut process = Process {
            id: Process::gen_id(),
            num_1: rng.gen_range(0..=100),
            num_2: rng.gen_range(0..=100),
            operation: match rng.gen_range(0..=4) {
                0 => '+',
                1 => '-',
                2 => '*',
                3 => '/',
                4 => '%',
                _ => unreachable!(),
            },
            math_exp: String::new(),
            result: String::new(),
            time: timer::Times::new(),
        };

        //unsafe { P_EXE_TIME += process.get_exe_time(); }
        process.gen_math_exp();
        process.gen_result();
        process
    }

    // Generators

    fn gen_id() -> u16 {
        static mut ID_COUNTER : u16 = 0;
        unsafe {
            ID_COUNTER  += 1;
            ID_COUNTER 
        }
    }

    fn gen_number_2(&mut self) {
        let mut rng = rand::thread_rng();
        self.num_2 = rng.gen_range(0..=100);
    }

    fn gen_math_exp(&mut self) {
        self.math_exp = format!("{}{}{}", self.num_1, self.operation, self.num_2);
    }

    fn gen_result(&mut self) {
        while self.num_2 == 0 && (self.operation == '/' || self.operation == '%') {
            self.gen_number_2();
            self.gen_math_exp();
        }

        self.result = match self.operation {
            '+' => ((self.num_1 + self.num_2) as f32).to_string(),
            '-' => (self.num_1 as f32 - self.num_2 as f32).to_string(),
            '*' => ((self.num_1 * self.num_2) as f32).to_string(),
            '/' => (self.num_1 as f32 / self.num_2 as f32).to_string(),
            '%' => (self.num_1 as f32 % self.num_2 as f32).to_string(),
            _ => {
                println!("Invalid operation. Try again");
                (f32::NAN).to_string()
            }
        };
    }

    // Getters

    pub fn get_id(&self) -> &u16 { &self.id }
    pub fn _get_num_1(&self) -> &u32 { &self.num_1 }
    pub fn _get_num_2(&self) -> &u32 { &self.num_2 }
    pub fn _get_operation(&self) -> &char { &self.operation }
    pub fn get_math_exp(&self) -> &String { &self.math_exp }
    pub fn get_result(&self) -> &String { &self.result }
    pub fn get_p_exe_time() -> *const u8 { unsafe { addr_of!(P_EXE_TIME) } }

    // Setters

    pub fn set_result(&mut self, res: &str) {
        self.result = res.to_string();
    }
}
