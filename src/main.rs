pub extern crate glium;

mod engine;
mod entities;


fn main() {
    engine::renderer::main_loop(|event| {
        match event {
            _ => (),
        }
        
        true
    });
}
