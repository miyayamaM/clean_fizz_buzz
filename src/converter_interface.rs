use mockall::automock;
#[automock]
pub trait ConverterInterface {
    fn convert(&self, num: u32) -> String;
}
