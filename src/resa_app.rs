use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use resa_ecs::entity::Entity;
use resa_ecs::world::World;
use resa_renderer::mesh::{create_primitive_quad, create_primitive_triangle, Mesh};
use resa_renderer::{RendererConfig, ResaRenderer};
use crate::rendering::mesh_renderer::MeshRenderer;

#[allow(dead_code)]
pub struct ResaApp {
	pub name: String,
	logical_size: LogicalSize<u32>,
	pub physical_size: PhysicalSize<u32>,
	pub event_loop: EventLoop<()>,
	pub window: Window,
	pub renderer: Rc<RefCell<ResaRenderer>>,
	pub world: Rc<RefCell<World>>,
}

impl ResaApp {
	pub fn new(name: &str, width: u32, height: u32) -> Option<Self> {
		let event_loop = EventLoop::new();

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

		let renderer = Rc::new(RefCell::new(ResaRenderer::new(&window, RendererConfig {
			extent: physical_size.clone(),
			vertex_shader_path: "./src/rendering/shaders/base.vert".to_string(),
			fragment_shader_path: "./src/rendering/shaders/base.frag".to_string(),
		})));

		let world = Rc::new(RefCell::new(World::new()));

		Some(ResaApp {
			name: name.to_string(),
			logical_size,
			physical_size,
			event_loop,
			window,
			renderer,
			world,
		})
	}

	pub fn load_mesh(&mut self, mesh: Mesh) -> MeshRenderer {
		let mesh_id = self.renderer.borrow_mut().register_mesh(mesh);
		MeshRenderer::new(mesh_id, 0)
	}

	#[allow(unused)]
	pub fn run_window_loop(mut self) {
		let mut should_configure_swapchain = true;

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
						should_configure_swapchain = true;
					}
					WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
						// set new surface scale here!

						should_configure_swapchain = true;
					}
					_ => (),
				}
				Event::MainEventsCleared => self.window.request_redraw(),
				Event::RedrawRequested(_) => {
					if should_configure_swapchain {
						// rendering_controller.reconfigure_swapchain(&self.physical_size, &mut world);
						self.renderer.borrow_mut().refresh();
						should_configure_swapchain = false;
					}
					/* for entity in &mut scene {
						 entity.update()
					 }*/
					let mut mesh_ids = Vec::new();
					for mesh_components in self.world.borrow_mut().get_all_components_of_type::<MeshRenderer>().unwrap() {
						mesh_ids.push(mesh_components.0.mesh_id);
					}


					self.renderer.borrow_mut().render(&mesh_ids);
					if loop_runs % 10 == 0 {
						loop_runs = 0;

						println!("{}", self.renderer.borrow_mut().get_fps());
					}
				}
				_ => (),
			}
		});
	}
}