pub trait Component{
    fn enable(&mut self);
    fn disable(&mut self);

    fn update(&self);
}