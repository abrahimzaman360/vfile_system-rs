pub fn calculator(left: i32, operator: char, right: i32) -> Result<f32, String> {
    // match looks like switch -> I liked it:
    match operator {
        '+' => Ok((left + right) as f32),
        '-' => Ok((left - right) as f32),
        '*' => Ok((left * right) as f32),
        '/' => {
            if right == 0 {
                return Err("Cannot divide by 0".to_string());
            }
            Ok((left / right) as f32)
        }
        _ => Err("Awesome Error".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn machine_test() {
        let num1 = 10;
        let op = '-';
        let num2 = 5;

        let some_val = calculator(num1, op, num2).unwrap();

        assert_eq!(some_val, 5., "Must be 5 from {}", some_val);
    }
}
