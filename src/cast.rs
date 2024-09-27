use super::as_any::AsAny;

pub trait Cast: AsAny {
    fn cast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any_ref().downcast_ref()
    }

    fn cast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}
