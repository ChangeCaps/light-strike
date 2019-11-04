#[macro_use]
pub extern crate glium;

use glium::glutin::Event;
use glium::glutin::WindowEvent;
use glium::glutin::DeviceEvent;
use glium::glutin::VirtualKeyCode;

#[macro_use]
mod engine;
mod entities;

use engine::types::*;

fn main() {
    let mut ecs = engine::ecs::ECS::new();

    //
    // Objects
    //

    ecs.push(None, None, None, None, Some(vec![Vector2::new(-0.2, 0.2), Vector2::new(-0.2, 0.5), Vector2::new(0.2, 0.5), Vector2::new(0.2, 0.2)]));
    let poly = ecs.push(None, None, None, None, Some(vec![Vector2::new(-0.2, -0.2), Vector2::new(0.0, -0.5), Vector2::new(0.2, -0.2)]));


    // 
    // Game variables
    //

    let mut player_trail: Vec<(engine::ecs::ID, f32)> = Vec::new();
    
    let mut bullets = Vec::new();

    let mut time: f32 = 0.0;

    let mut player_movement = Vector2::new(0.0, 0.0);
    let mut player_position = Vector2::new(0.0, 0.0);

    let mut camera_size = 1.0;

    let mut mouse_position = Vector2::new(0.0, 0.0);
    

    let mut keys_down = Vec::new();

    let mut speed = 0.5;

    engine::renderer::main_loop(ecs, |ecs, events_loop, delta_time, resolution| {
        let mut running = true;

        // Event handeling

        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent {event, ..} => {
                    match event {
                        WindowEvent::CloseRequested => {
                            running = false;
                        },
                        WindowEvent::CursorMoved {position, ..} => {
                            mouse_position = Vector2::new(
                                position.x as f32 / resolution.0 * 2.0 - 1.0, 
                                -(position.y as f32 / resolution.1 * 2.0 - 1.0)
                            );
                        },
                        WindowEvent::MouseInput {button, state, ..} => {
                            if state == glium::glutin::ElementState::Pressed {
                                match button {
                                    glium::glutin::MouseButton::Left => {
                                        let bullet = ecs.push(
                                            Some(player_position),
                                            None,
                                            Some((0.01, 1.0, 1.0, 1.0)),
                                            Some(mouse_position.normalize().mul(2.0)),
                                            None
                                        );

                                        bullets.push(bullet);
                                    },
                                    glium::glutin::MouseButton::Right => {
                                        speed = 3.0;
                                    },
                                    _ => (),
                                }
                            } else {
                                match button {
                                    glium::glutin::MouseButton::Right => {
                                        speed = 0.5;
                                    },
                                    _ => (),
                                }
                            }
                        }, _ => (),
                    }
                },
                Event::DeviceEvent {event, ..} => {
                    match event {
                        DeviceEvent::Key(glium::glutin::KeyboardInput{virtual_keycode, state, ..} ) => {
                            if let Some(key) = virtual_keycode {
                                if state == glium::glutin::ElementState::Pressed {
                                    if !keys_down.contains(&key) {
                                        keys_down.push(key);
                                    }

                                    // 
                                    // Key Pressed
                                    //

                                    match key {
                                        VirtualKeyCode::Space => {
                                            
                                        },
                                        _ => (),
                                    }
                                } else {
                                    keys_down = keys_down.clone().into_iter().filter(|x| *x != key).collect();

                                    //
                                    // Key Released
                                    //

                                }

                                
                            }
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        });

        for key in &keys_down {
            match key {
                _ => (),
            }
        }

        if !keys_down.contains(&VirtualKeyCode::Space) {
            player_movement = mouse_position;
        }

        for i in &mut player_trail {
            i.1 -= delta_time;

            if speed >= 3.0 {
                *ecs.light(i.0).unwrap() = (i.1 * 4.0, 0.0, 0.01, 0.0);
            } else {
                *ecs.light(i.0).unwrap() = (i.1, 0.0, 0.0, 0.01);
            }
        }

        player_trail = player_trail.clone().into_iter().filter(|(id, t)| {
            if *t < 0.0 {
                // Ignore the error for now
                let _ = ecs.remove(*id);
                return false;
            }

            true
        }).collect();

        if player_movement.len() != 0.0 {
            player_trail.push((
                ecs.push(
                    Some(player_position),
                    None,
                    Some((1.0, 0.0, 0.0, 0.01)),
                    None,
                    None
                ),
                0.5
            ));
        }

        player_position += player_movement.mul(delta_time).mul(speed);

        //print!("\r{:?}", ecs.gen);

        // Game logic

        bullets = bullets.clone().into_iter().filter(|x| {
            if (*ecs.position(*x).unwrap() - player_position).len() > 10.0 {
                ecs.remove(*x);

                return false;
            }

            true
        }).collect();

        time += delta_time;

        (running, player_position, camera_size)
    });
}
