pub trait HasData {
    fn encode(&self) -> u64;
    fn size(&self) -> u16;
}
