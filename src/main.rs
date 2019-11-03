#[macro_use]
pub extern crate glium;

#[macro_use]
mod engine;
mod entities;

use engine::types::*;

fn main() {
    let mut ecs = engine::ecs::ECS::new();

    ecs.push(None, None, None, None, Some(vec![Vector2::new(-0.2, 0.2), Vector2::new(0.0, 0.5), Vector2::new(0.2, 0.2)]));
    ecs.push(None, None, None, None, Some(vec![Vector2::new(-0.2, -0.2), Vector2::new(0.0, -0.5), Vector2::new(0.2, -0.2)]));
    let light = ecs.push(Some(Vector2::new(0.0, 0.0)), None, Some(1.0), None, None);

    let time = 0.0;

    engine::renderer::main_loop(ecs, |ecs, delta_time| {
        

        time += delta_time;
    });
}
