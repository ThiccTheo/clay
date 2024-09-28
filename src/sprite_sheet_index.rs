use {
    super::{as_any::AsAny, id::Id, property::Property},
    std::any::Any,
};

#[derive(Clone)]
pub struct SpriteSheetIndex(pub usize);

impl AsAny for SpriteSheetIndex {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Id for SpriteSheetIndex {}

impl Property for SpriteSheetIndex {}
