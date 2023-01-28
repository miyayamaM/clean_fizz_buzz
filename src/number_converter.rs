use crate::converter_interface::ConverterInterface;
use crate::replace_interface::ReplaceInterface;
use mockall::automock;

pub struct NumberConverter {
    pub rules: Vec<Box<dyn ReplaceInterface>>,
}

#[automock]
impl ConverterInterface for NumberConverter {
    fn convert(&self, num: u32) -> String {
        self.rules.iter().fold("".to_string(), |carry, rule| {
            if rule.matching(carry.clone(), num) {
                rule.apply(carry, num)
            } else {
                carry
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::converter_interface::ConverterInterface;
    use crate::number_converter::NumberConverter;
    use crate::replace_interface::MockReplaceInterface;
    use crate::replace_interface::ReplaceInterface;
    use crate::rules::cyclic_number_rule::CyclicNumberRule;
    use crate::rules::pass_through_rule::PassThroughRule;

    #[test]
    fn test_convert_with_empty_rules() {
        let rules = vec![];
        let converter = NumberConverter { rules };
        assert_eq!("".to_string(), converter.convert(1));
    }

    #[test]
    fn test_convert_with_single_rule() {
        let rules: Vec<Box<dyn ReplaceInterface>> = vec![Box::new(create_mock_rule(
            "".to_string(),
            1,
            "Replaced".to_string(),
            true,
        ))];
        let converter = NumberConverter { rules };
        assert_eq!("Replaced".to_string(), converter.convert(1));
    }

    #[test]
    fn test_convert_with_multiple_rules() {
        let rules: Vec<Box<dyn ReplaceInterface>> = vec![
            Box::new(create_mock_rule(
                "".to_string(),
                1,
                "Fizz".to_string(),
                true,
            )),
            Box::new(create_mock_rule(
                "Fizz".to_string(),
                1,
                "FizzBuzz".to_string(),
                true,
            )),
        ];
        let converter = NumberConverter { rules };
        assert_eq!("FizzBuzz".to_string(), converter.convert(1));
    }

    #[test]
    fn test_convert_skipping_unmatched_rules() {
        let rules: Vec<Box<dyn ReplaceInterface>> = vec![
            Box::new(create_mock_rule(
                "".to_string(),
                1,
                "Fizz".to_string(),
                false,
            )),
            Box::new(create_mock_rule(
                "".to_string(),
                1,
                "Buzz".to_string(),
                false,
            )),
            Box::new(create_mock_rule("".to_string(), 1, "1".to_string(), true)),
        ];
        let converter = NumberConverter { rules };
        assert_eq!("1".to_string(), converter.convert(1));
    }

    fn create_mock_rule(
        expected_carry: String,
        expected_num: u32,
        replacement: String,
        matching_result: bool,
    ) -> MockReplaceInterface {
        let mut mock_rule = MockReplaceInterface::new();
        mock_rule
            .expect_apply()
            .with(eq(expected_carry.clone()), eq(expected_num))
            .return_const(replacement);

        mock_rule
            .expect_matching()
            .with(eq(expected_carry.clone()), eq(expected_num))
            .return_const(matching_result);

        mock_rule
    }

    #[test]
    fn test_convert() {
        let rules: Vec<Box<dyn ReplaceInterface>> = vec![
            Box::new(CyclicNumberRule {
                base: 3,
                replacement: "Fizz".to_string(),
            }),
            Box::new(CyclicNumberRule {
                base: 5,
                replacement: "Buzz".to_string(),
            }),
            Box::new(PassThroughRule {}),
        ];
        let fizz_buzz = NumberConverter { rules };
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
