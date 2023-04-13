pub trait Handler <T> {
    fn supports(&self, handleable: T) -> bool;
}