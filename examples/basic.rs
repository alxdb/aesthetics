extern crate aesthetics;
extern crate glium;
extern crate itertools;
extern crate nalgebra_glm;
extern crate specs;

use aesthetics::{
    components::{mesh, Camera, Transform},
    systems::{event_handler, input, renderer},
    window_utils,
};

use glium::glutin;
use itertools::iproduct;
use nalgebra_glm as glm;
use specs::{world::Builder, DispatcherBuilder, World};

fn init_display() -> (glutin::EventsLoop, glium::Display) {
    let window_builder = glutin::WindowBuilder::new()
        // .with_fullscreen(Some(window_utils::get_primary_monitor()))
        .with_title("Hello world");
    let context_builder = glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(true);
    window_utils::new_window(window_builder, context_builder)
}

fn main() {
    let (ev_loop, display) = init_display();
    display.gl_window().window().grab_cursor(true).unwrap();
    display.gl_window().hide_cursor(true);
    let init_window_size = display.get_framebuffer_dimensions();

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut world = World::new();
    let renderer = renderer::Renderer::new(&mut world, display, draw_params, (0.1, 0.1, 0.1, 1.0));
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
        // .with(Camera::Ortho {
        //     size: 1.0,
        //     aspect: init_window_size.0 as f32 / init_window_size.1 as f32,
        // })
        .with(Transform {
            pos: glm::vec3(0.0, std::f32::consts::SQRT_2, std::f32::consts::SQRT_2),
            rot: glm::quat_look_at(&glm::vec3(0.0, 1.0, -1.0), &glm::vec3(0.0, 1.0, 0.0)),
            // pos: glm::vec3(0.0, 0.0, 0.0),
            // rot: glm::Quat::identity(),
            // rot: glm::quat_angle_axis(glm::half_pi(), &glm::vec3(0.0, 1.0, 0.0)),
        })
        .build();

    for (i, j, k) in iproduct!(
        (-1..=1).step_by(2),
        (-1..=1).step_by(2),
        (-1..=1).step_by(2)
    ) {
        let cube_dims = glm::vec3(0.3, 0.3, 0.3);
        world
            .create_entity()
            .with(mesh::cube(cube_dims))
            .with(Transform::new(glm::vec3(i as f32, j as f32, k as f32)))
            .build();
    }
    for (x, y, z) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)].iter() {
        for i in (-1..=1).step_by(2) {
            let (xx, yy, zz) = (i * x, i * y, i * z);
            world
                .create_entity()
                .with(mesh::sphere(0.1, 32))
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
