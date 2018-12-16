extern crate aesthetic;
extern crate specs;

use aesthetic::{components::mesh, systems::renderers, window_utils};
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
    let camera = world
        .create_entity()
        .with(renderers::camera::Camera::Ortho { size: 0.5 })
        .build();

    let mut mesh_renderer = renderers::basic::Renderer::new(&mut world, &display, &camera);

    let cube = world
        .create_entity()
        .with(mesh::cube((0.7, 0.7, 1.0)))
        .build();

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
        println!("dur: {:#?}", Instant::now() - start);
    }
}
