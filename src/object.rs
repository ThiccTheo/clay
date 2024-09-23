use {
    super::{id::Id, world::World}, crate::action::Action, ggez::{
        graphics::{Rect, Transform},
        Context,
    }, std::any::Any
};

/// Cross between OOP and ECS world objects.
///
/// These objects are intended to interact with the world,
/// unlike things like GUI, for example, which interact with the mouse.
#[allow(dead_code)]
pub trait Object {
    /// Integer value associated with each type of `Object`.
    ///
    /// This value is also used to layer render batches on top of one another,
    /// where the highest id is on the very top and vice-versa with the lowest id.
    ///
    /// # Example
    /// ```
    /// pub struct Player;
    ///
    /// impl Player {
    ///     pub const ID: u8 = ...;
    /// }
    ///
    /// impl Object for Player {
    ///     ...
    ///     
    ///     fn id(&self) -> u8 {
    ///         Self::ID
    ///     }
    /// }
    /// ```
    fn id(&self) -> Id;

    /// What should this object do in the current frame?
    ///
    /// * `others` - All other objects in the state excluding `self`.
    /// * `ctx` - App context.
    fn tick(&mut self, others: World, ctx: &mut Context, action: &mut Option<Action>);

    /// Whether or not `self` is active.
    ///
    /// i.e., should `self` be despawned from the world at the end of the frame?
    ///
    /// # Example
    /// ```
    /// pub struct Player {
    ///     pub is_active: bool,
    /// }
    ///
    /// impl Object for Player {
    ///     ...
    ///
    ///     fn is_active(&self) -> bool {
    ///         self.is_active
    ///     }
    /// }
    /// ```
    fn is_active(&self) -> bool;

    /// Whether or not `self` is visible.
    ///
    /// i.e., should `self` be rendered on `draw()`?
    fn is_visible(&self) -> bool {
        true
    }

    /// The `Transform` i.e., position and orientation of `self`.
    ///
    /// `None` is `self` has no `Transform`;
    /// `Some(Transform)` if `self` has a `Transform`.
    ///
    /// This function is supposed to be used by the engine and for world interactions.
    fn transform(&self) -> Option<Transform> {
        None
    }

    /// The bounding box of `self`.
    ///
    /// Is `None` if `self` has no collider; is `Some(Rect)` if `self` has a collider.
    ///
    /// This is meant to be a rectangle centered on `self`'s transform, assuming
    /// `self` has one.
    ///
    /// For collision detection, use `Rect::overlaps(...)` or `Rect::contains(...)`.
    ///
    /// This function is supposed to be used by the engine and for world interactions.
    fn collider(&self) -> Option<Rect> {
        None
    }

    fn sprite_sheet_index(&self) -> Option<usize> {
        None
    }

    /// Helper function for casting `&dyn Object` to `&dyn Any`.
    ///
    /// Prefer to use `cast<T>(&self)` for casting from `&dyn Object`
    /// to it's underlying type.
    ///
    /// # Example
    /// ```
    /// pub struct Player;
    ///
    /// impl Object for Player {
    ///     ...
    ///
    ///     fn as_any(&self) -> &dyn Any {
    ///         self
    ///     }
    /// }
    /// ```
    fn as_any_ref(&self) -> &dyn Any;

    /// Helper function for casting `&mut dyn Object` to `&mut dyn Any`.
    ///
    /// Prefer to use `cast_mut<T>()` for casting from `&mut dyn Object`
    /// to it's underlying type.
    ///
    /// # Example
    /// ```
    /// pub struct Player;
    ///
    /// impl Object for Player {
    ///     ...
    ///
    ///     fn as_any_mut(&mut self) -> &mut dyn Any {
    ///         self
    ///     }
    /// }
    /// ```
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[allow(dead_code)]
impl dyn Object {
    /// Cast from `&dyn Object` to `&T`.
    ///
    /// Returns `Option<&T>`, which is `None` when T is not the underlying type.
    pub fn cast<T: 'static + Object>(&self) -> Option<&T> {
        self.as_any_ref().downcast_ref()
    }

    /// Cast from `&mut dyn Object` to `&mut T`.
    ///
    /// Returns `Option<mut &T>`, which is `None` when T is not the underlying type.
    pub fn cast_mut<T: 'static + Object>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}
