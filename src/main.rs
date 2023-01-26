pub mod converter_interface;
pub mod factories;
pub mod number_converter;
pub mod output_interface;
pub mod outputs;
pub mod printer;
pub mod replace_interface;
pub mod rules;

use crate::factories::fizz_buzz_app_factory::FizzBuzzAppFactory;

fn main() {
    let factory = FizzBuzzAppFactory {};
    let printer = factory.create();
    printer.print_range(1, 100)
}
