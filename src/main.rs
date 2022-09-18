use crate::rendering::RenderingController;

mod rendering;
mod window;


fn main() {
    println!("Hello, world!");

    let mut window = match window::Window::new("SkriptR", 512, 512){
        Some(window) => window,
        None => return,
    };

    let renderer = RenderingController::new(&window);
/*    if renderer.is_none(){
        println!("Creating renderer failed!");
        return;
    }*/

    window.set_renderer_instance(renderer);

    window.run_window_loop();


}

