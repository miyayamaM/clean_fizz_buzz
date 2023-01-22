use crate::replace_interface::ReplaceInterface;

pub struct PassThroughRule {}

impl ReplaceInterface for PassThroughRule {
    fn apply(&self, _carry: String, num: u32) -> String {
        num.to_string()
    }

    fn matching(&self, carry: String, _num: u32) -> bool {
        carry == ""
    }
}

#[cfg(test)]
mod tests {
    use super::PassThroughRule;
    use crate::replace_interface::ReplaceInterface;

    #[test]
    fn test_apply() {
        let rule = PassThroughRule {};
        assert_eq!(rule.apply("".to_string(), 1), "1".to_string());
        assert_eq!(rule.apply("Fizz".to_string(), 3), "3".to_string());
        assert_eq!(rule.apply("".to_string(), 6), "6".to_string());
    }

    #[test]
    fn test_matching() {
        let rule = PassThroughRule {};
        assert_eq!(rule.matching("".to_string(), 0), true);
        assert_eq!(rule.matching("Fizz".to_string(), 0), false);
    }
}
