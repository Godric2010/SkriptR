use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use resa_ecs::world::World;
use crate::rendering::mesh::create_primitive_quad;
use crate::rendering::RenderingController;

#[allow(dead_code)]
pub struct Window {
    pub name: String,
    logical_size: LogicalSize<u32>,
    pub physical_size: PhysicalSize<u32>,
    pub event_loop: EventLoop<()>,
    pub instance: winit::window::Window,
    rendering_controller: Option<RenderingController>,
}

impl Window {
    pub fn new(name: &str, width: u32, height: u32) -> Option<Self> {
        let event_loop = EventLoop::new();

        let primary_monitor = match event_loop.primary_monitor() {
            Some(monitor) => monitor,
            None => return None,
        };

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
                return None
            },
        };


        Some(Window {
            name: name.to_string(),
            logical_size,
            physical_size,
            event_loop,
            instance: window,
            rendering_controller: None,
        })
    }

    pub fn set_renderer_instance(&mut self, renderer: RenderingController) {
        self.rendering_controller = Some(renderer);
    }

    #[allow(unused)]
    pub fn run_window_loop(mut self, world_instance: World) {
        let mut should_configure_swapchain = true;
        let mut rendering_controller = match self.rendering_controller {
            Some(rendering_controller) => rendering_controller,
            None => {
                println!("No renderer found! End application!");
                return;
            }
        };

        let mut world= world_instance;

        rendering_controller.add_mesh_to_renderer(&create_primitive_quad());

        let start_time = std::time::Instant::now();
        let mut anim = 0.0;
        self.event_loop.run(move |event, _, control_flow| {
            anim = start_time.elapsed().as_secs_f32().sin() * 0.5 + 0.5;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("Requested shutdown!");
                        *control_flow = ControlFlow::ExitWithCode(0);
                    },
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
                Event::MainEventsCleared => self.instance.request_redraw(),
                Event::RedrawRequested(_) => {
                    if should_configure_swapchain{
                        rendering_controller.reconfigure_swapchain(&self.physical_size);
                        should_configure_swapchain = false;
                    }
                   /* for entity in &mut scene {
                        entity.update()
                    }*/
                    rendering_controller.render(&mut world);
                }
                _ => (),
            }
        })
    }
}