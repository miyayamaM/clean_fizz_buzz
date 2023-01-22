pub mod number_converter;
pub mod replace_interface;
pub mod rules;

use crate::number_converter::NumberConverter;
use crate::replace_interface::ReplaceInterface;
use crate::rules::cyclic_number_rule::CyclicNumberRule;
use crate::rules::pass_through_rule::PassThroughRule;

fn main() {
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

    for i in 1..100 {
        println!("{}", fizz_buzz.convert(i));
    }
}
