extern crate glium;
extern crate itertools;
extern crate nalgebra_glm;

mod renderer;
mod window;

use glium::Surface;

use renderer::object;
use renderer::shader;
use renderer::shader::Shader;

fn main() {
    // Diplay Setup
    let window_builder = glium::glutin::WindowBuilder::new()
        .with_fullscreen(Some(window::get_primary_monitor()))
        .with_title("Hello world");
    let context_builder = glium::glutin::ContextBuilder::new()
        .with_multisampling(8)
        .with_vsync(true);
    let mut window = window::Window::new(window_builder, context_builder);

    // Shader Compilation
    let basic_shader = shader::BasicShader::new(&window);

    // Buffer Allocation
    let sphere = object::Sphere::new(1.0, 32);

    let (index_buffer, vertex_buffer) = {
        let vertices = shader::Mesh::create_vertices(&basic_shader, &sphere);
        let indices = object::Mesh::get_mesh(&sphere).get_indices();
        (
            glium::index::IndexBuffer::immutable(window.display_ref(), indices.1, indices.0)
                .unwrap(),
            glium::vertex::VertexBuffer::dynamic(window.display_ref(), &vertices).unwrap(),
        )
    };

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
        for ev in window.get_events().iter() {
            match ev {
                glium::glutin::Event::WindowEvent {
                    event: glium::glutin::WindowEvent::CloseRequested,
                    ..
                } => break 'main,
                _ => (),
            }
        }

        // Rendering
        let mut frame = window.display_ref().draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                basic_shader.get_program(),
                &glium::uniforms::EmptyUniforms,
                &params,
            ).unwrap();
        frame.finish().unwrap();
    }
}
