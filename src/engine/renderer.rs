#[macro_use]
use glium;
use glium::Surface;
use super::*;
use types::*;
use std::thread;
use std::sync::{
    Arc, Mutex
};
use std::sync::mpsc::{
    self,
};
use std::time::Instant;

#[derive(Copy, Clone)]
struct Vert {
    position: [f32; 3],
}

glium::implement_vertex!(Vert, position);


pub fn main_loop<F>(mut ecs: ecs::ECS, mut each_frame: F) 
    where F: FnMut(&mut ecs::ECS, &mut glium::glutin::EventsLoop, f32, (f32, f32)) -> (bool, Vector2, f32)
{
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
        Vert { position: [1.0, 1.0, -1.0] },
        Vert { position: [-1.0, 1.0, -1.0] },
        Vert { position: [1.0, -1.0, -1.0] },

        // Bottom left
        Vert { position: [-1.0, -1.0, -1.0] },
        Vert { position: [-1.0, 1.0, -1.0] },
        Vert { position: [1.0, -1.0, -1.0] },
    ]).expect("Failed to create vertex buffer!");

    let program  = glium::program::Program::from_source(
        &display, 
        include_str!("../vertex_shader.glsl"),
        include_str!("../fragment_shader.glsl"),
        None).expect("Failed to create program");
 
    let mut running = true; 

    let mut time = None;

    let time_per_frame = std::time::Duration::from_millis(16);

    // Main Loop
    while running {
        let mut delta_time = 0.0;
        
        if let Some(time) = time {
            
            let t = time_per_frame.checked_sub(Instant::now().duration_since(time));

            if let Some(wait) = t {
                thread::sleep(wait);
            }

            delta_time = Instant::now().duration_since(time).as_micros() as f32 / 1_000_000.0;
        }

        let res = display.get_framebuffer_dimensions();

        let output = each_frame(&mut ecs, &mut events_loop, delta_time, (res.0 as f32, res.1 as f32));

        running = output.0;

        let camera_position = output.1;
        let camera_size = output.2;

        time = Some(Instant::now());

        let mut frame = display.draw();


        //
        // Updates on components
        //

        for i in 0..ecs.len() {
            if let Some(pos) = &mut ecs.position[i] {
                if let Some(vel) = ecs.velocity[i] {
                    *pos += vel.mul(delta_time);
                }
            }
        }

        //
        // Get entities for rendering
        //

        let mut polygons = Vec::new();
        let mut polygon_lengths = Vec::new();

        for i in &ecs.polygon {
            if let Some(polygon) = i {
                for i in polygon {
                    polygons.push(i.arr());
                }

                polygon_lengths.push(polygon.len() as i32);
            }
        }

        let mut light_positions = Vec::new();
        let mut lights = Vec::new();

        for i in 0..ecs.len() {
            if let Some(light_position) = ecs.position[i] {
                if let Some(light) = ecs.light[i] {
                    light_positions.push(light_position.arr());
                    lights.push([light.0, light.1, light.2, light.3]);
                }
            }
        }

        //println!("{:?}\n{:?}\n", polygons, polygon_lengths);
        //println!("{:?}", lights);

        let object_lengths = glium::buffer::Buffer::<[i32]>::new(&display, &polygon_lengths, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();
        let object_positions = glium::buffer::Buffer::<[[f32; 2]]>::new(&display, &polygons, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();
        let light_positions = glium::buffer::Buffer::<[[f32; 2]]>::new(&display, &light_positions, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();
        let lights = glium::buffer::Buffer::<[[f32; 4]]>::new(&display, &lights, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();


        let uniforms = uniform!{
            resolution: [res.0 as f32, res.1 as f32],
            camera_position: <[f32; 2]>::from(camera_position),
            camera_size: camera_size,
            object_length_buffer: &object_lengths,
            object_position_buffer: &object_positions,
            light_position_buffer: &light_positions,
            light_buffer: &lights,
        };

        frame.draw(&vertex_buffer, &index_buffer, 
            &program, &uniforms, &Default::default()).expect("Failed to draw!");

        frame.finish().unwrap();
    }
}
