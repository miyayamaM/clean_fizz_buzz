pub mod factories;

use crate::factories::fizz_buzz_app_factory::FizzBuzzAppFactory;
fn main() {
    let factory = FizzBuzzAppFactory {};
    let printer = factory.create();
    printer.print_range(0, 100)
}
