pub mod number_converter;

use crate::number_converter::NumberConverter;

fn main() {
    let fizz_buzz = NumberConverter {};
    for i in 1..100 {
        let converted_string = fizz_buzz.convert(i);
        println!("{}", converted_string)
    }
}
