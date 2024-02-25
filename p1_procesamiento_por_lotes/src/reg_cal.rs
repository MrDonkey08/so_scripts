use regex::Regex;

pub fn math_operation(expression: Regex, mut operation: String, operator: &str) -> Result<String, &'static str> {
    loop {
        let caps = expression.captures(operation.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let caps_expression = caps.get(0).unwrap().as_str();
        let caps_num1 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let caps_num2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let result = match operator {
            "+" => caps_num1 + caps_num2,
            "-" => caps_num1 - caps_num2,
            "*" => caps_num1 * caps_num2,
            "/" => {
                if caps_num2 != 0 {
                    caps_num1 / caps_num2
                } else {
                    return Err("Division by zero is not allowed");
                }
            },
            "%" => {
                if caps_num2 != 0 {
                    caps_num1 % caps_num2
                } else {
                    return Err("Modulus by zero is not allowed");
                }
            },
            _ => 0,
        };

        operation = operation.replace(caps_expression, &result.to_string());
    }
    Ok(operation)
}

