use crate::output_interface::OutputInterface;

pub struct ConsoleOutput {}

impl OutputInterface for ConsoleOutput {
    fn write(&self, data: String) {
        println!("{}", data)
    }
}
