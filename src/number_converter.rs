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
}
