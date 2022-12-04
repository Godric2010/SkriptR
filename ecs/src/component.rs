use std::any::Any;
use std::cell::RefCell;

pub(crate) trait ComponentVec{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&mut self);
    fn set_none(&mut self, entity: usize);

}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn Any {
       self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }

    fn set_none(&mut self, entity: usize) {
        self.get_mut()[entity] = None
    }
}