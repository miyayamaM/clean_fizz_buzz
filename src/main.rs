pub mod number_converter;

use crate::number_converter::NumberConverter;

fn main() {
    let fizz_buzz = NumberConverter {};
    fizz_buzz.convert(1);
}
