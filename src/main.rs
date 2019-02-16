extern crate aesthetics;
extern crate nalgebra_glm;
extern crate specs;

use aesthetics::{
    components::{mesh, ActiveCamera, Camera, Transform},
    systems::renderer,
    window_utils,
};

use nalgebra_glm as glm;
use specs::{world::Builder, DispatcherBuilder, World};

fn init_display() -> (glium::glutin::EventsLoop, glium::Display) {
    let window_builder = glium::glutin::WindowBuilder::new()
        .with_fullscreen(Some(window_utils::get_primary_monitor()))
        .with_title("Hello world");
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(true);
    window_utils::new_window(window_builder, context_builder)
}

fn main() {
    let mut world = World::new();
    let (mut ev_loop, display) = init_display();
    let renderer = renderer::Renderer::new(&mut world, display);

    let mut dispatcher = DispatcherBuilder::new().with_thread_local(renderer).build();

    let _camera = world
        .create_entity()
        .with(Camera::Persp {
            fov: 45.0,
            aspect: 1920.0 / 1080.0,
        })
        .with(Transform {
            pos: glm::vec3(0.0, std::f32::consts::SQRT_2, std::f32::consts::SQRT_2),
            // pos: glm::vec3(0.0, 0.0, 2.0),
            rot: glm::quat_look_at(&glm::vec3(0.0, 1.0, -1.0), &glm::vec3(0.0, 1.0, 0.0)),
            // rot: glm::quat_angle_axis(-std::f32::consts::PI / 4.0, &glm::vec3(1.0, 0.0, 0.0)),
            // rot: glm::Quat::identity(),
        })
        .build();

    let _camera = world
        .create_entity()
        .with(Camera::Persp {
            fov: 45.0,
            aspect: 1920.0 / 1080.0,
        })
        .with(Transform {
            // pos: glm::vec3(0.0, std::f32::consts::SQRT_2, std::f32::consts::SQRT_2),
            pos: glm::vec3(0.0, 0.0, 2.0),
            // rot: glm::quat_look_at(&glm::vec3(0.0, 0.5, 0.0), &glm::vec3(0.0, 1.0, 0.0)),
            // rot: glm::quat_angle_axis(-std::f32::consts::PI / 4.0, &glm::vec3(1.0, 0.0, 0.0)),
            rot: glm::Quat::identity(),
        })
        .build();

    *world.write_resource::<ActiveCamera>() = ActiveCamera(Some(_camera));

    let _obj = world
        .create_entity()
        .with(mesh::cube((1.0, 1.0, 1.0)))
        .with(Transform {
            pos: glm::vec3(0.0, 0.0, 0.0),
            // rot: glm::quat_angle_axis(std::f32::consts::PI / 8.0, &glm::vec3(0.0, 1.0, 0.0)),
            rot: glm::Quat::identity(),
        })
        .build();

    'main: loop {
        for ev in window_utils::get_events(&mut ev_loop).iter() {
            match ev {
                glium::glutin::Event::WindowEvent {
                    event: glium::glutin::WindowEvent::CloseRequested,
                    ..
                } => break 'main,
                _ => (),
            }
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }

    println!("exited successfully")
}
