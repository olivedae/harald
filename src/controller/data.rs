pub trait HasData {
    fn encode(&self) -> Vec<u8>;
}
