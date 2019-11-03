#[macro_use]
pub extern crate glium;

use glium::glutin::Event;
use glium::glutin::WindowEvent;

#[macro_use]
mod engine;
mod entities;

use engine::types::*;

fn main() {
    let mut ecs = engine::ecs::ECS::new();

    ecs.push(None, None, None, None, Some(vec![Vector2::new(-0.2, 0.2), Vector2::new(-0.2, 0.5), Vector2::new(0.2, 0.5), Vector2::new(0.2, 0.2)]));
    let poly = ecs.push(None, None, None, None, Some(vec![Vector2::new(-0.2, -0.2), Vector2::new(0.0, -0.5), Vector2::new(0.2, -0.2)]));
    let light = ecs.push(Some(Vector2::new(0.0, 0.0)), None, Some((1.5, 1.0, 0.0, 1.0)), None, None);
    let light2 = ecs.push(Some(Vector2::new(0.0, 0.0)), None, Some((1.0, 0.0, 1.0, 0.0)), None, None);

    let mut time: f32 = 0.0;

    engine::renderer::main_loop(ecs, |ecs, events_loop, delta_time| {
        let mut running = true;

        // Event handeling

        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent {event, ..} => {
                    match event {
                        WindowEvent::CloseRequested => {
                            running = false;
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        });



        // Game logic

        position!(ecs, light).x = time.sin();
        position!(ecs, light2).x = (time).sin();
        position!(ecs, light2).y = time.cos();


        time += delta_time;

        running
    });
}
