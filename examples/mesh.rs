extern crate aesthetic;
extern crate specs;

use aesthetic::systems::renderers::mesh::MeshRenderer;
use aesthetic::components::mesh;
use specs::{world::Builder, World, RunNow};
use aesthetic::window_utils;

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
    let mut mesh_renderer = MeshRenderer::new(&mut world, &display);

    let cube = world.create_entity().with(mesh::cube((1.0, 1.0, 1.0))).build();

    'main: loop {
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
    }

}
