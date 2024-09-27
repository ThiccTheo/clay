use {
    super::object::Object,
    std::{iter::Chain, slice::IterMut},
};

/// Iterator of all `Object`s excluding the one being iterated on when this is built.
///
/// # Example
/// ```
/// for i in 0..self.objects.len() {
///     let (before, tmp) = self.objects.split_at_mut(i);
///     let (this, after) = tmp.split_first_mut().unwrap();
///     let others = before.iter_mut().chain(after.iter_mut());
///     
///     // "others" is the "world"
///     this.update(others, ctx);
/// }
/// ```
pub type World<'a> = Chain<IterMut<'a, Object>, IterMut<'a, Object>>;
