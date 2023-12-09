pub trait NuType {
    fn into_inner(self) -> u64;
    fn new(value: u64) -> Self;
}
