mod rendering;
mod window;


fn main() {
    println!("Hello, world!");

    let window = match window::Window::new("SkriptR", 512, 512){
        Some(window) => window,
        None => return,
    };

    window.run_window_loop();

}

