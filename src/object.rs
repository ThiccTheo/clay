use {
    super::world::World,
    ggez::{graphics::InstanceArray, Context},
    std::any::Any,
};

/// Cross between OOP and ECS world objects.
///
/// These objects are intended to interact with the world,
/// unlike things like GUI, for example, which interact with the mouse.
#[allow(dead_code)]
pub trait Object {
    /// What should this object do in the current frame?
    ///
    /// * `others` - All other objects in the state excluding `self`.
    /// * `ctx` - App context.
    fn update(&mut self, others: World, ctx: &mut Context);

    /// How should this object be rendered?
    ///
    /// * `batch` - List of verticies associated with an image.
    fn draw(&self, batch: &mut InstanceArray);

    /// Whether or not `self` is active.
    ///
    /// i.e., should `self` be despawned from the world at the end of the frame?
    fn is_active(&self) -> bool;

    /// Integer value associated with each type of `Object`.
    ///
    /// This value is also used to layer render batches on top of one another,
    /// where the highest id is on the very top and vice-versa with the lowest id.
    fn id(&self) -> u8;

    /// Helper function for casting `&dyn Object` to `&dyn Any`.
    /// 
    /// Prefer to use `cast<T>(&self)` for casting from `&dyn Object`
    /// to it's underlying type.
    fn as_any(&self) -> &dyn Any;

    /// Helper function for casting `&mut dyn Object` to `&mut dyn Any`.
    /// 
    /// Prefer to use `cast_mut<T>()` for casting from `&mut dyn Object`
    /// to it's underlying type.
    fn as_any_mut(&self) -> &mut dyn Any;
}

#[allow(dead_code)]
impl dyn Object {
    /// Cast from `&dyn Object` to `&T`.
    /// 
    /// Returns `Option<&T>`, which is `None` when T is not the underlying type.
    pub fn cast<T: 'static + Object>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    /// Cast from `&mut dyn Object` to `&mut T`.
    /// 
    /// Returns `Option<mut &T>`, which is `None` when T is not the underlying type.
    pub fn cast_mut<T: 'static + Object>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}
