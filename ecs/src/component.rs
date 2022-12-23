use std::any::{Any, TypeId};

pub(crate) trait ComponentInstanceCollection: Any{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn new_empty_column(&self) -> Box<dyn ComponentInstanceCollection>;
}

impl<T: 'static> ComponentInstanceCollection for Vec<T> {
    fn as_any(&self) -> &dyn Any {
       self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
       self
    }

    fn new_empty_column(&self) -> Box<dyn ComponentInstanceCollection> {
        Box::new(Vec::<T>::new())
    }
}