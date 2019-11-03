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
    where F: FnMut(&mut ecs::ECS, &mut glium::glutin::EventsLoop, f32) -> bool
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


    // Polygon getter
    let (ecs_polygon_sender, reciever) = mpsc::channel::<Vec<Arc<Mutex<Option<Vec<Vector2>>>>>>();
    let (polygon_position_sender, polygon_position_reciever) = mpsc::channel();
    let (polygon_lengths_sender, polygon_lengths_reciever) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let ecs_polygons = reciever.recv().unwrap();
            let mut polygons = Vec::new();
            let mut polygon_lengths = Vec::new();

            for i in ecs_polygons {
                let i = i.lock().unwrap();

                if let Some(i) = &*i {
                    polygon_lengths.push(i.len() as i32);                   

                    for vert in i {
                        polygons.push(<[f32; 2]>::from(vert.clone()));                
                    }
                }
            }

            polygon_position_sender.send(polygons).unwrap();
            polygon_lengths_sender.send(polygon_lengths).unwrap();
        }
    });


    // Light getter
    let (ecs_light_position_sender, light_position_reciever) = mpsc::channel::<Vec<Arc<Mutex<Option<Vector2>>>>>();
    let (ecs_light_sender, light_receiver) = mpsc::channel::<Vec<Arc<Mutex<Option<(f32, f32, f32, f32)>>>>>();
    let (light_positions_sender, light_positions_reciever) = mpsc::channel();
    let (lights_sender, lights_reciever) = mpsc::channel();
    thread::spawn(move || {
        loop {
            let ecs_light_positions = light_position_reciever.recv().unwrap();
            let ecs_lights = light_receiver.recv().unwrap();

            let mut light_positions = Vec::new();
            let mut lights = Vec::new();

            for i in 0..ecs_light_positions.len() {
                let light = ecs_lights[i].lock().unwrap();
                let position = ecs_light_positions[i].lock().unwrap();

                if let Some(light) = &*light {
                    if let Some(position) = &*position {
                        light_positions.push([position.x, position.y]);
                        lights.push([light.0, light.1, light.2, light.3]);
                    }
                }
            }

            light_positions_sender.send(light_positions).unwrap();
            lights_sender.send(lights).unwrap();
        }
    });

    

    let mut running = true; 

    let mut time = None;

    // Main Loop
    while running {
        let mut delta_time = 0.0;

        if let Some(time) = time {
            delta_time = Instant::now().duration_since(time).as_micros() as f32 / 1_000_000.0;

            print!("\r{:?}             ", Instant::now().duration_since(time));
        }

        running = each_frame(&mut ecs, &mut events_loop, delta_time);

        time = Some(Instant::now());

        let mut frame = display.draw();

        let res = display.get_framebuffer_dimensions();

        ecs_polygon_sender.send(ecs.polygon.clone()).unwrap();
        
        ecs_light_position_sender.send(ecs.position.clone()).unwrap();
        ecs_light_sender.send(ecs.light.clone()).unwrap();

        let polygons = polygon_position_reciever.recv().unwrap();
        let polygon_lengths = polygon_lengths_reciever.recv().unwrap();
        let light_positions = light_positions_reciever.recv().unwrap();
        let lights = lights_reciever.recv().unwrap();

        //println!("{:?}\n{:?}\n", polygons, polygon_lengths);
        //println!("{:?}", lights);

        let object_lengths = glium::buffer::Buffer::<[i32]>::new(&display, &polygon_lengths, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();
        let object_positions = glium::buffer::Buffer::<[[f32; 2]]>::new(&display, &polygons, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();
        let light_positions = glium::buffer::Buffer::<[[f32; 2]]>::new(&display, &light_positions, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();
        let lights = glium::buffer::Buffer::<[[f32; 4]]>::new(&display, &lights, glium::buffer::BufferType::ArrayBuffer, glium::buffer::BufferMode::Default).unwrap();


        let uniforms = uniform!{
            resolution: [res.0 as f32, res.1 as f32],
            object_lengths: &object_lengths,
            object_positions: &object_positions,
            light_positions: &light_positions,
            light_strengths: &lights,
        };

        frame.draw(&vertex_buffer, &index_buffer, 
            &program, &uniforms, &Default::default()).expect("Failed to draw!");

        frame.finish().unwrap();
    }
}
