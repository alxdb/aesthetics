extern crate aesthetics;
extern crate nalgebra_glm;
extern crate specs;

use aesthetics::{
    components::{mesh, transform},
    systems::renderers,
    window_utils,
};
use nalgebra_glm as glm;
use specs::{world::Builder, RunNow, World};

use std::time::Instant;

fn init_display() -> (glium::glutin::EventsLoop, glium::Display) {
    let window_builder = glium::glutin::WindowBuilder::new()
        // .with_fullscreen(Some(window_utils::get_primary_monitor()))
        .with_title("Hello world");
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(true);
    window_utils::new_window(window_builder, context_builder)
}

fn main() {
    let mut world = World::new();
    let (mut ev_loop, display) = init_display();
    let mut mesh_renderer = renderers::basic::Renderer::new(&mut world, &display);

    let camera = world
        .create_entity()
        // .with(renderers::camera::Camera::Ortho {
        //     size: 2.0,
        //     ratio: 1.0,
        // })
        .with(renderers::camera::Camera::Persp {
            fov: 45.0,
            aspect: 1.0,
        })
        .with(transform::Transform {
            pos: glm::vec3(0.0, 0.0, 3.0),
            // rot: glm::quat_look_at(&glm::vec3(0.0, 0.5, 0.0), &glm::vec3(0.0, 1.0, 0.0)),
            rot: glm::quat_angle_axis(45.0, &glm::vec3(1.0, 0.0, 0.0)),
            // rot: glm::Quat::identity(),
        })
        .build();

    mesh_renderer.set_main_camera(camera);

    let cube = world
        .create_entity()
        .with(mesh::cube((1.0, 1.0, 1.0)))
        .with(transform::Transform {
            pos: glm::vec3(0.0, 0.5, 0.0),
            rot: glm::quat_angle_axis(-45.0, &glm::vec3(1.0, 0.0, 0.0)),
        })
        .build();

    // let sphere = world
    //     .create_entity()
    //     .with(mesh::sphere(1.0, 13))
    //     .with(transform::Transform::new(glm::Vec3::new(0.0, 0.0, 0.0)))
    //     .build();

    'main: loop {
        let start = Instant::now();
        // Input Handle
        for ev in window_utils::get_events(&mut ev_loop).iter() {
            match ev {
                glium::glutin::Event::WindowEvent {
                    event: glium::glutin::WindowEvent::CloseRequested,
                    ..
                } => break 'main,
                _ => (),
            }
        }

        // Rendering
        mesh_renderer.run_now(&world.res);
        // println!("dur: {:#?}", Instant::now() - start);
    }
}
