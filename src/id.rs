use std::any::TypeId;

pub trait Id
where
    Self: 'static,
{
    fn id() -> TypeId {
        TypeId::of::<Self>()
    }

    fn m_id(&self) -> TypeId {
        Self::id()
    }
}
