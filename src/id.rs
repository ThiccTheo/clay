pub trait Id {
    fn id() -> u8;

    fn m_id(&self) -> u8 {
        Self::id()
    }
}
