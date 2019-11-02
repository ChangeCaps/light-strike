use glium;
use glium::Surface;
use super::*;

#[derive(Copy, Clone)]
struct Vert {
    position: [f32; 2],
}

glium::implement_vertex!(Vert, position);


pub fn main_loop(each_frame: fn (event: glium::glutin::Event) -> bool) {
    let mut events_loop = glium::glutin::EventsLoop::new();

    let wb = glium::glutin::WindowBuilder::new()
        .with_dimensions(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Light Strike");

    let cb = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &events_loop).unwrap(); 



    let index_buffer = glium::index::NoIndices(
        glium::index::PrimitiveType::TrianglesList); 


    let vertex_buffer = glium::VertexBuffer::new(&display, &[
        // Top right
        Vert { position: [1.0, 1.0] },
        Vert { position: [-1.0, 1.0] },
        Vert { position: [1.0, -1.0] },

        // Bottom left
        Vert { position: [-1.0, -1.0] },
        Vert { position: [-1.0, 1.0] },
        Vert { position: [1.0, -1.0] },
    ]).expect("Failed to create vertex buffer!");

    let program  = glium::program::Program::from_source(
        &display, 
        include_str!("../vertex_shader.glsl"),
        include_str!("../fragment_shader.glsl"),
        None).expect("Failed to create program");

    let mut ecs = ecs::ECS::new();

    let mut running = true; 

    // Main Loop
    while running {
        let mut frame = display.draw();

        let uniforms = glium::uniform!{
            
        };

        frame.draw(&vertex_buffer, &index_buffer, 
            &program, &uniforms, &Default::default()).expect("Failed to draw!");

        frame.finish().unwrap();

        events_loop.poll_events(|event| {
            running = each_frame(event);
        });
    }
}
