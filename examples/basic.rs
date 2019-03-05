extern crate aesthetics;
extern crate glium;
extern crate nalgebra_glm;
extern crate specs;

use aesthetics::{
    components::{mesh, Camera, Transform},
    systems::{event_handler, input, renderer},
    window_utils,
};

use glium::glutin;
use nalgebra_glm as glm;
use specs::{world::Builder, DispatcherBuilder, World};

fn init_display() -> (glutin::EventsLoop, glium::Display) {
    let window_builder = glutin::WindowBuilder::new()
        .with_fullscreen(Some(window_utils::get_primary_monitor()))
        .with_title("Hello world");
    let context_builder = glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(true);
    window_utils::new_window(window_builder, context_builder)
}

fn main() {
    let mut world = World::new();
    let (ev_loop, display) = init_display();
    display.gl_window().hide_cursor(true);
    let init_window_size = display.get_framebuffer_dimensions();
    let renderer = renderer::Renderer::new(&mut world, display);
    let event_handler = event_handler::EventHandler::new(ev_loop);

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(event_handler)
        .with(
            input::fps_camera::FPSCamera::default(),
            "camera_controler",
            &[],
        )
        .with_thread_local(renderer)
        .build();

    dispatcher.setup(&mut world.res);

    let _camera = world
        .create_entity()
        .with(Camera::Persp {
            fov: 45.0,
            aspect: init_window_size.0 as f32 / init_window_size.1 as f32,
        })
        .with(Transform {
            // pos: glm::vec3(0.0, std::f32::consts::SQRT_2, std::f32::consts::SQRT_2),
            // rot: glm::quat_look_at(&glm::vec3(0.0, 1.0, -1.0), &glm::vec3(0.0, 1.0, 0.0)),
            pos: glm::vec3(0.0, 0.0, 0.0),
            rot: glm::Quat::identity(),
            // rot: glm::quat_angle_axis(0.5, &glm::vec3(0.0, 1.0, 0.0)),
        })
        .build();

    for (x, y, z) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)].iter() {
        for i in (-1..=1).step_by(2) {
            let (xx, yy, zz) = (i * x, i * y, i * z);
            world
                .create_entity()
                .with(mesh::cube((0.2, 0.2, 0.2)))
                .with(Transform {
                    pos: glm::vec3(xx as f32, yy as f32, zz as f32),
                    rot: glm::Quat::identity(),
                })
                .build();
        }
    }

    'main: loop {
        if world.read_resource::<event_handler::ShouldClose>().0 {
            break 'main;
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }

    println!("exited successfully")
}
