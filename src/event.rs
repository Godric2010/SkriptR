pub struct Event<T, TOut> {
	callbacks: Vec<Box<dyn Fn(T) -> TOut>>,
}

impl<T, TOut> Event<T, TOut> {
	pub fn new() -> Self {
		Self {
			callbacks: Vec::new()
		}
	}

	pub fn add_listener(&mut self, listener: impl Fn(T) -> TOut + 'static) {
		self.callbacks.push(Box::new(listener));
	}

	pub fn execute(&self, value: T) -> TOut{
		self.callbacks[0](value)
	}
}