use std::fmt::{Debug, Error};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::error::OsError;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct Window {
    name: String,
    logical_size: LogicalSize<u32>,
    pub physical_size: PhysicalSize<u32>,
    pub event_loop: EventLoop<()>,
    instance: winit::window::Window,
}

impl Window {
    pub fn new(name: &str, width: u32, height: u32) -> Option<Self> {
        let event_loop = EventLoop::new();

        let primary_monitor = match event_loop.primary_monitor(){
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
            Err(e) => {println!("{}", e); return None},
        };


        Some(Window {
            name: name.to_string(),
            logical_size,
            physical_size,
            event_loop,
            instance: window,
        })
    }

    pub fn run_window_loop(mut self){

        let mut should_configure_swapchain = true;

        self.event_loop.run(move |event, _, control_flow|{

            match event{
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::ExitWithCode(0),
                    WindowEvent::Resized(dims) => {
                        // Resize surface here!


                        self.physical_size = PhysicalSize::new(dims.width, dims.height);

                        should_configure_swapchain = true;
                    }
                    WindowEvent::ScaleFactorChanged {new_inner_size,..} =>{
                        // set new surface scale here!

                        should_configure_swapchain = true;
                    }
                    _ => (),
                }
                Event::MainEventsCleared => self.instance.request_redraw(),
                Event::RedrawRequested(_) => {
                    // render_func();
                }
                _ => (),
            }
        })
    }
}