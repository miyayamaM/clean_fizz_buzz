pub mod factories;

use crate::factories::fizz_buzz_app_factory::FizzBuzzAppFactory;
use clean_fizz_buzz::number_converter::NumberConverter;
use clean_fizz_buzz::outputs::console_output::ConsoleOutput;
use clean_fizz_buzz::printer::FizzBuzzSequencePrinter;

pub fn create_printer() -> FizzBuzzSequencePrinter<NumberConverter, ConsoleOutput> {
    let factory = FizzBuzzAppFactory {};
    factory.create()
}

fn main() {
    let printer = create_printer();
    printer.print_range(0, 100)
}
