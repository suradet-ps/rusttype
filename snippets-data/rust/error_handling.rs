use std::num::ParseIntError;

fn parse_and_double(input: &str) -> Result<i64, ParseIntError> {
    let value: i64 = input.trim().parse()?;
    Ok(value * 2)
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

fn process_values(inputs: &[&str]) -> Vec<Result<i64, ParseIntError>> {
    inputs.iter().map(|s| parse_and_double(s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_input() {
        assert_eq!(parse_and_double("5").unwrap(), 10);
    }

    #[test]
    fn parse_invalid_input() {
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn divide_normal() {
        let result = divide(10.0, 2.0).unwrap();
        assert!((result - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn process_multiple_values() {
        let results = process_values(&["1", "2", "bad", "4"]);
        assert_eq!(results.len(), 4);
        assert!(results[2].is_err());
    }
}
