pub mod converter_interface;
pub mod factories;
pub mod number_converter;
pub mod output_interface;
pub mod outputs;
pub mod printer;
pub mod replace_interface;
pub mod rules;

use number_converter::NumberConverter;
use outputs::console_output::ConsoleOutput;
use printer::FizzBuzzSequencePrinter;

use crate::factories::fizz_buzz_app_factory::FizzBuzzAppFactory;

pub fn create_printer() -> FizzBuzzSequencePrinter<NumberConverter, ConsoleOutput> {
    let factory = FizzBuzzAppFactory {};
    factory.create()
}
