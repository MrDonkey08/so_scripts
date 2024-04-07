use rand::Rng;

static mut P_EXE_TIME: u8 = 0;

#[derive(Default, Debug, Clone)]
pub struct Process {
    id: u16,
    num_1: u32,
    num_2: u32,
    operation: char,
    math_exp: String,
    result: f32,
    exe_time: u8,
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
            result: 0.0,
            exe_time: rng.gen_range(1..=2),
        };

        unsafe { P_EXE_TIME += process.get_exe_time(); }
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
        let rng = rand::thread_rng();
        self.num_2 = rand::thread_rng().gen_range(0..=100);
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
            '+' => (self.num_1 + self.num_2) as f32,
            '-' => self.num_1 as f32 - self.num_2 as f32,
            '*' => (self.num_1 * self.num_2) as f32,
            '/' =>  self.num_1 as f32 / self.num_2 as f32,
            '%' =>  self.num_1 as f32 % self.num_2 as f32,
            _ => {
                println!("Invalid operation. Try again");
                f32::NAN
            }
        };
    }

    // Getters

    pub fn get_id(&self) -> &u16 { &self.id }
    pub fn get_num_1(&self) -> &u32 { &self.num_1 }
    pub fn get_num_2(&self) -> &u32 { &self.num_2 }
    pub fn get_operation(&self) -> &char { &self.operation }
    pub fn get_math_exp(&self) -> &String { &self.math_exp }
    pub fn get_result(&self) -> &f32 { &self.result }
    pub fn get_exe_time(&self) -> &u8 { &self.exe_time }
    pub fn get_p_exe_time() -> &'static u8 { unsafe { &P_EXE_TIME } }
}
