use {
    super::object::Object,
    std::{iter::Chain, slice::IterMut},
};

pub type World<'a> = Chain<IterMut<'a, Object>, IterMut<'a, Object>>;
