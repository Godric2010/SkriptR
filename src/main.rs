use crate::rendering::{create_renderer };

mod rendering;
mod window;


fn main() {
    println!("Hello, world!");

    let window = match window::Window::new("SkriptR", 512, 512){
        Some(window) => window,
        None => return,
    };

    let renderer = create_renderer(&window);
    // if renderer.is_none(){
    //     return;
    // }

    window.run_window_loop();


}

