extern crate generational_arena;
extern crate glium;
extern crate itertools;
extern crate nalgebra_glm;

use renderer::object;
use renderer::shader;
use renderer::shader::Shader;

mod renderer;
mod window;

fn main() {
    // Diplay Setup
    let window_builder = glium::glutin::WindowBuilder::new()
        .with_fullscreen(Some(window::get_primary_monitor()))
        .with_title("Hello world");
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(true);
    let (mut ev_loop, display) = window::new_window(window_builder, context_builder);

    // Shader Compilation
    use renderer::Renderer;
    let mut basic_render: renderer::BasicRenderer<_, shader::BasicShader> =
        renderer::BasicRenderer::new(&display);

    // Buffer Allocation
    let _sphere = basic_render.add_object(object::Sphere::new(0.5, 10));
    let _cube = basic_render.add_object(object::Cube::new((0.6, 0.6, 0.6)));

    // Draw Parameters
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    'main: loop {
        // Input Handle
        for ev in window::get_events(&mut ev_loop).iter() {
            match ev {
                glium::glutin::Event::WindowEvent {
                    event: glium::glutin::WindowEvent::CloseRequested,
                    ..
                } => break 'main,
                _ => (),
            }
        }

        // Rendering
        basic_render.draw((0.0, 0.0, 0.0, 1.0), &params).unwrap();
    }
}
