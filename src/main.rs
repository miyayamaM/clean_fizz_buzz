use clean_fizz_buss::create_printer;

fn main() {
    let printer = create_printer();
    printer.print_range(0, 100)
}
