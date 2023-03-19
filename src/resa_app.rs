use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use resa_ecs::world::World;
use resa_renderer::RendererConfig;
use crate::rendering::RenderingSystem;
use crate::resource_loader::ResourceLoader;

#[allow(dead_code)]
pub struct ResaApp {
	pub name: String,
	logical_size: LogicalSize<u32>,
	pub physical_size: PhysicalSize<u32>,
	pub event_loop: EventLoop<()>,
	pub window: Window,
	pub rendering: Rc<RefCell<RenderingSystem>>,
	pub world: Rc<RefCell<World>>,
	pub resource_loader: ResourceLoader,
}

impl ResaApp {
	pub fn new(name: &str, width: u32, height: u32) -> Option<Self> {
		let event_loop = EventLoop::new();
		let mut resource_loader = ResourceLoader::new()?;

		let primary_monitor = event_loop.primary_monitor()?;

		let dpi = primary_monitor.scale_factor();
		let logical_size = LogicalSize::new(width, height);
		let physical_size = logical_size.to_physical(dpi);

		let window = match WindowBuilder::new()
			.with_title(name)
			.with_inner_size(logical_size)
			.with_always_on_top(true)
			.build(&event_loop) {
			Ok(win) => win,
			Err(e) => {
				println!("{}", e);
				return None;
			}
		};

		let shader_refs = resource_loader.load_all_shaders()?;
		let renderer = Rc::new(RefCell::new(RenderingSystem::new(&window, RendererConfig {
			extent: physical_size.clone(),
			shaders: shader_refs,
		})));


		let renderer_binding = renderer.clone();
		resource_loader.set_image_cb(move |data|renderer_binding.borrow_mut().register_texture(data));

		let world = Rc::new(RefCell::new(World::new()));

		Some(ResaApp {
			name: name.to_string(),
			logical_size,
			physical_size,
			event_loop,
			window,
			rendering: renderer,
			world,
			resource_loader,
		})
	}

	// pub fn load_mesh(&mut self, mesh: Mesh) -> MeshRenderer {
	// 	let mesh_id = self.rendering.borrow_mut().register_mesh(mesh);
	// 	MeshRenderer::new(mesh_id, 0)
	// }

	#[allow(unused)]
	pub fn run_window_loop(mut self) {
		let start_time = std::time::Instant::now();
		let mut anim = 0.0;
		let mut loop_runs = 0;
		self.event_loop.run(move |event, _, control_flow| {
			loop_runs += 1;
			anim = start_time.elapsed().as_secs_f32().sin() * 0.5 + 0.5;
			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						println!("Requested shutdown!");
						*control_flow = ControlFlow::ExitWithCode(0);
					}
					WindowEvent::Resized(dims) => {
						self.physical_size = PhysicalSize::new(dims.width, dims.height);
						self.rendering.borrow_mut().set_dirty();
					}
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						// set new surface scale here!

						self.rendering.borrow_mut().set_dirty();
					}
					_ => (),
				}
				Event::MainEventsCleared => self.window.request_redraw(),
				Event::RedrawRequested(_) => {
					self.rendering.borrow_mut().render(&Rc::clone(&self.world));
				}
				_ => (),
			}
		});
	}
}