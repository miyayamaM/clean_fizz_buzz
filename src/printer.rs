use crate::number_converter::NumberConverter;
use crate::output_interface::OutputInterface;

pub struct FizzBuzzSequencePrinter<T: OutputInterface> {
    pub fizzbuzz: NumberConverter,
    pub output: T,
}

impl<T: OutputInterface> FizzBuzzSequencePrinter<T> {
    pub fn print_range(&self, start: u32, end: u32) {
        for i in start..end {
            let text = self.fizzbuzz.convert(i);
            let formatted_text = format!("{0} {1}", i.to_string(), text);
            self.output.write(formatted_text)
        }
    }
}
