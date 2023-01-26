use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait OutputInterface {
    fn write(&self, data: String);
}
