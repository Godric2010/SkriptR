use std::any::{Any};
use std::ops::Index;

pub(crate) trait ComponentInstanceCollection: Any{
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn new_empty_column(&self) -> Box<dyn ComponentInstanceCollection>;
    fn remove_at(&mut self, index: usize);
    fn migrate(&mut self, index: usize, other: &mut dyn ComponentInstanceCollection);
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

    fn remove_at(&mut self, index: usize){
       self.remove(index);
    }

    fn migrate(&mut self, index: usize, other: &mut dyn ComponentInstanceCollection) {
        let data = self.remove(index);
        other.as_any_mut().downcast_mut::<Vec<T>>().unwrap().push(data);
    }
}