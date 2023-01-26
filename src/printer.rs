use crate::converter_interface::ConverterInterface;
use crate::output_interface::OutputInterface;

pub struct FizzBuzzSequencePrinter<T: ConverterInterface, S: OutputInterface> {
    pub fizzbuzz: T,
    pub output: S,
}

impl<T: ConverterInterface, S: OutputInterface> FizzBuzzSequencePrinter<T, S> {
    pub fn print_range(&self, start: u32, end: u32) {
        for i in start..=end {
            let text = self.fizzbuzz.convert(i);
            let formatted_text = format!("{0} {1}", i.to_string(), text);
            self.output.write(formatted_text)
        }
    }
}

#[cfg(test)]
mod tests {
    #[double]
    use crate::number_converter::NumberConverter;
    use crate::output_interface::MockOutputInterface;
    use mockall_double::double;

    use super::FizzBuzzSequencePrinter;

    #[test]
    fn test_print_1_to_3() {
        //converterのmock作成
        let mut mock_converter = NumberConverter::new();
        mock_converter
            .expect_convert()
            .times(3)
            .return_const("Fizz".to_string());
        //outputのmock作成
        let mut mock_output = MockOutputInterface::new();
        mock_output.expect_write().times(3).return_const(());
        // FizzBuzzPrinterインスタンスを作成
        let printer = FizzBuzzSequencePrinter {
            fizzbuzz: mock_converter,
            output: mock_output,
        };

        printer.print_range(1, 3);
    }
}
