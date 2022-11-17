use std::any::Any;

pub trait Component<'a>{
    fn enable(&mut self);
    fn disable(&mut self);

    fn update(&self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}