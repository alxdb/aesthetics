extern crate glium;
extern crate nalgebra_glm;

mod window;

use glium::implement_vertex;
use glium::Surface;

#[derive(Copy, Clone)]
struct BasicVertex {
    pos: [f32; 3],
}
implement_vertex!(BasicVertex, pos);

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
    let shader = glium::program::Program::from_source(
        window.display_ref(),
        include_str!("glsl/basic.vert"),
        include_str!("glsl/basic.frag"),
        None,
    ).unwrap();

    // Buffer Allocation
    let vertex_buffer = glium::vertex::VertexBuffer::dynamic(
        window.display_ref(),
        &vec![
            BasicVertex {
                pos: [-0.5, -0.5, 0.0],
            },
            BasicVertex {
                pos: [0.5, -0.5, 0.0],
            },
            BasicVertex {
                pos: [0.0, 0.5, 0.0],
            },
        ],
    ).unwrap();

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
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &shader,
                &glium::uniforms::EmptyUniforms,
                &params,
            ).unwrap();
        frame.finish().unwrap();
    }
}
