use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Instant};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use resa_ecs::entity::Entity;
use resa_ecs::world::World;
use resa_renderer::RendererConfig;
use resa_ui::ResaUserInterface;
use crate::rendering::RenderingSystem;
use crate::resources::resource_loader::ResourceLoader;
use crate::resources::ResourceManager;
use crate::test_anim::{change_color, rotate_entity};

#[allow(dead_code)]
pub struct ResaApp {
	pub name: String,
	logical_size: LogicalSize<u32>,
	pub physical_size: PhysicalSize<u32>,
	pub event_loop: EventLoop<()>,
	pub window: Window,
	pub rendering: RenderingSystem,
	pub ui: ResaUserInterface,
	pub world: Rc<RefCell<World>>,
	pub resource_loader: ResourceManager,
}

impl ResaApp {
	pub fn new(name: &str, width: u32, height: u32) -> Option<Self> {
		let event_loop = EventLoop::new();
		let mut resource_manager = ResourceManager::new()?;

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

		let shaders = resource_manager.get_shaders();
		let mut renderer = RenderingSystem::new(&window, RendererConfig {
			extent: physical_size.clone(),
			shaders,
		});
		renderer.load_materials(&resource_manager.get_materials());

		let ui_system = ResaUserInterface::new(resource_manager.get_fonts());

		let world = Rc::new(RefCell::new(World::new()));

		Some(ResaApp {
			name: name.to_string(),
			logical_size,
			physical_size,
			event_loop,
			window,
			rendering: renderer,
			ui: ui_system,
			world,
			resource_loader: resource_manager,
		})
	}

	#[allow(unused)]
	pub fn run_window_loop(mut self) {
		let system_time = Instant::now();
		let mut last_time = 0.0;
		self.event_loop.run(move |event, _, control_flow| {
			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						println!("Requested shutdown!");
						*control_flow = ControlFlow::ExitWithCode(0);
					}
					WindowEvent::Resized(dims) => {
						self.physical_size = PhysicalSize::new(dims.width, dims.height);
						self.rendering.set_dirty();
					}
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						// set new surface scale here!

						self.rendering.set_dirty();
					}
					_ => (),
				}
				Event::MainEventsCleared => self.window.request_redraw(),
				Event::RedrawRequested(_) => {
					let current_time = system_time.elapsed().as_secs_f64();
					let delta_time = (current_time - last_time);
					last_time = current_time;
					let entity: Entity = Entity(4);
					rotate_entity(&Rc::clone(&self.world), &entity, &delta_time );

					let entity = Entity(2);
					change_color(&Rc::clone(&self.world), &entity, &delta_time);

					self.rendering.render(&Rc::clone(&self.world));
				}
				_ => (),
			}
		});
	}
}