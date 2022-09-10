mod rendering;
mod window;


fn main() {
    println!("Hello, world!");

    let mut window = match window::Window::new("SkriptR", 512, 512){
        Ok(window) => window,
        Err(e) => return,
    };

    window.run_window_loop();

}

fn temp(){
    println!("Renderer currently not implemented!");
}
