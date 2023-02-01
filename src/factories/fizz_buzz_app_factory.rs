use clean_fizz_buzz::number_converter::NumberConverter;
use clean_fizz_buzz::outputs::console_output::ConsoleOutput;
use clean_fizz_buzz::printer::FizzBuzzSequencePrinter;
use clean_fizz_buzz::replace_interface::ReplaceInterface;
use clean_fizz_buzz::rules::cyclic_number_rule::CyclicNumberRule;
use clean_fizz_buzz::rules::pass_through_rule::PassThroughRule;

pub struct FizzBuzzAppFactory {}

impl FizzBuzzAppFactory {
    pub fn create(&self) -> FizzBuzzSequencePrinter<NumberConverter, ConsoleOutput> {
        FizzBuzzSequencePrinter {
            fizzbuzz: self.create_fizz_buzz(),
            output: self.create_output(),
        }
    }

    fn create_fizz_buzz(&self) -> NumberConverter {
        let rules: Vec<Box<dyn ReplaceInterface>> = vec![
            Box::new(self.create_fizz_rule()),
            Box::new(self.create_buzz_rule()),
            Box::new(self.create_pass_through_rule()),
        ];
        NumberConverter { rules }
    }

    fn create_fizz_rule(&self) -> CyclicNumberRule {
        CyclicNumberRule {
            base: 3,
            replacement: "Fizz".to_string(),
        }
    }

    fn create_buzz_rule(&self) -> CyclicNumberRule {
        CyclicNumberRule {
            base: 5,
            replacement: "Buzz".to_string(),
        }
    }

    fn create_pass_through_rule(&self) -> PassThroughRule {
        PassThroughRule {}
    }

    fn create_output(&self) -> ConsoleOutput {
        ConsoleOutput {}
    }
}
