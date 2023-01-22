#[warn(dead_code)]
pub struct NumberConverter {}

#[warn(dead_code)]
impl NumberConverter {
    pub fn convert(&self, num: u32) -> String {
        if num % 3 == 0 && num % 5 == 0 {
            "FizzBuzz".to_string()
        } else if num % 3 == 0 {
            "Fizz".to_string()
        } else if num % 5 == 0 {
            "Buzz".to_string()
        } else {
            num.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::number_converter::NumberConverter;

    #[test]
    fn test_convert() {
        let fizz_buzz = NumberConverter {};
        assert_eq!("1".to_string(), fizz_buzz.convert(1));
        assert_eq!("2".to_string(), fizz_buzz.convert(2));
        assert_eq!("Fizz".to_string(), fizz_buzz.convert(3));
        assert_eq!("4".to_string(), fizz_buzz.convert(4));
        assert_eq!("Buzz".to_string(), fizz_buzz.convert(5));
        assert_eq!("Fizz".to_string(), fizz_buzz.convert(6));
        assert_eq!("7".to_string(), fizz_buzz.convert(7));
        assert_eq!("8".to_string(), fizz_buzz.convert(8));
        assert_eq!("Fizz".to_string(), fizz_buzz.convert(9));
        assert_eq!("Buzz".to_string(), fizz_buzz.convert(10));
        assert_eq!("FizzBuzz".to_string(), fizz_buzz.convert(15));
        assert_eq!("FizzBuzz".to_string(), fizz_buzz.convert(30));
    }
}
