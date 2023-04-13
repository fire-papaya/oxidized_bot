pub trait Handler <T> {
    fn supports(handleable: T) -> bool;
}