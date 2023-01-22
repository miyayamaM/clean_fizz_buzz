pub trait ReplaceInterface {
    fn apply(&self, carry: String, num: u32) -> String;
    fn matching(&self, carry: String, num: u32) -> bool;
}
