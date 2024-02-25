use crate::reg_cal;

use regex::Regex;
use std::io::{self}; // Importing io and Write trait

#[derive(Default, Debug)]
pub struct Process {
    id: String,
    username: String,
    math_exp: String,
    ans_exp: String,
    exe_time: u64,
}

impl Process {
    // Method to create a new instance of Process
    pub fn new(id: String, username: String, math_exp: String, ans_exp: String, exe_time: u64) -> Self {
        Process {
            id,
            username,
            math_exp,
            ans_exp,
            exe_time,
        }
    }

    pub fn input_empty(&self) -> bool {
        let is_empty: bool = if self.id.is_empty() || self.username.is_empty() || self.math_exp.is_empty() {
            true
        } else { false };

        return is_empty;
    }

    pub fn get_id(&self) -> &str {
        return &self.id;
    }

    pub fn set_id(&mut self) {
        io::stdin()
        .read_line(&mut self.id)
        .expect("Failed to read line");
        self.id = self.id.trim().to_string();
    }

    pub fn set_id_empty(&mut self) {
        self.id.clear()
    }

    pub fn get_username(&self) -> &str {
        return &self.username;
    }

    pub fn set_username(&mut self) {
        io::stdin()
            .read_line(&mut self.username)
            .expect("Failed to read line");
        self.username = self.username.trim().to_string();
    }

    pub fn get_math_exp(&self) -> &str {
        return &self.math_exp;
    }

    pub fn set_math_exp(&mut self) {
        io::stdin()
            .read_line(&mut self.math_exp)
            .expect("Failed to read line");
        self.math_exp = self.math_exp.trim().to_string();
    }

    pub fn set_math_exp_empty(&mut self) {
        self.math_exp.clear();
    }

    pub fn get_ans_exp(&self) -> &str {
        return &self.ans_exp;
    }

    pub fn _set_ans_exp(&mut self) {
        io::stdin()
            .read_line(&mut self.ans_exp)
            .expect("Failed to read line");
        self.ans_exp = self.ans_exp.trim().to_string();
    }

    pub fn calculate_ans_exp(&mut self) {
        // Regex
        let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
        let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
        let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
        let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();
        let re_mod = Regex::new(r"(\d+)\s?\%\s?(\d+)").unwrap();

        let mut input = self.math_exp.clone();

        // Apply each operation in the specified order
        if let Ok(result) = reg_cal::math_operation(re_mul, input.clone(), "*") {
            input = result;
        } else {
            println!("Error occurred while performing multiplication");
            return;
        }

        if let Ok(result) = reg_cal::math_operation(re_div, input.clone(), "/") {
            input = result;
        } else {
            println!("Indeterminacy error: not possible to divide by 0");
            return;
        }

        if let Ok(result) = reg_cal::math_operation(re_mod, input.clone(), "%") {
            input = result;
        } else {
            println!("Indeterminacy error: not possible to divide by 0");
            return;
        }

        if let Ok(result) = reg_cal::math_operation(re_add, input.clone(), "+") {
            input = result;
        } else {
            println!("Error occurred while performing addition");
            return;
        }

        if let Ok(result) = reg_cal::math_operation(re_sub, input.clone(), "-") {
            input = result;
        } else {
            println!("Error occurred while performing subtraction");
            return;
        }

        self.ans_exp = input;
    }

    pub fn get_exe_time(&self) -> u64 {
        return self.exe_time;
    }

    pub fn set_exe_time(&mut self, value: u64) {
        self.exe_time = value;
    }
}
