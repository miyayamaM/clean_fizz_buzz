
#[test]
fn test_convert() {
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
    assert_eq!("1".to_string(), fizz_buzz.convert(1));
    assert_eq!("2".to_string(), fizz_buzz.convert(2));
    assert_eq!("Fizz".to_string(), fizz_buzz.convert(3));
    assert_eq!("4".to_string(), fizz_buzz.convert(4));
    assert_eq!("Buzz".to_string(), fizz_buzz.convert(5));
    assert_eq!("Fizz".to_string(), fizz_buzz.convert(6));
    assert_eq!("7".to_string(), fizz_buzz.convert(7));
    assert_eq!("8".to_string(), fizz_buzz.convert(8));
    assert_eq!("Fizz".to_string(), fizz_buzz.convert(9));
    assert_eq!("Buzz".to_string(), fizz_buzz.convert(10));
    assert_eq!("FizzBuzz".to_string(), fizz_buzz.convert(15));
    assert_eq!("FizzBuzz".to_string(), fizz_buzz.convert(30));
}
