use crate::replace_interface::ReplaceInterface;

pub struct CyclicNumberRule {
    pub base: u32,
    pub replacement: String,
}

impl ReplaceInterface for CyclicNumberRule {
    fn apply(&self, carry: String, _num: u32) -> String {
        carry.to_string() + &self.replacement
    }

    fn matching(&self, _carry: String, num: u32) -> bool {
        num % self.base == 0
    }
}

#[cfg(test)]
mod tests {
    use super::CyclicNumberRule;
    use crate::replace_interface::ReplaceInterface;

    #[test]
    fn test_apply() {
        let rule = CyclicNumberRule {
            base: 3,
            replacement: "Buzz".to_string(),
        };
        assert_eq!(rule.apply("".to_string(), 1), "Buzz".to_string());
        assert_eq!(rule.apply("Fizz".to_string(), 3), "FizzBuzz".to_string());
        assert_eq!(rule.apply("".to_string(), 6), "Buzz".to_string());
    }

    #[test]
    fn test_matching() {
        let rule = CyclicNumberRule {
            base: 3,
            replacement: "Buzz".to_string(),
        };
        assert_eq!(rule.matching("".to_string(), 1), false);
        assert_eq!(rule.matching("Fizz".to_string(), 3), true);
        assert_eq!(rule.matching("".to_string(), 6), true);
    }
}
