use super::object;
use glium::implement_vertex;
use glium::program;

#[derive(Copy, Clone, Debug)]
pub struct BasicVertex {
    pos: [f32; 3],
}
implement_vertex!(BasicVertex, pos);

pub struct BasicShader {
    program: program::Program,
}

impl super::Shader for BasicShader {
    fn new(window: &::window::Window) -> Self {
        let program = glium::program::Program::from_source(
            window.display_ref(),
            include_str!("glsl/basic.vert"),
            include_str!("glsl/basic.frag"),
            None,
        ).unwrap();
        BasicShader { program }
    }

    fn get_program(&self) -> &program::Program {
        &self.program
    }
}

impl<I, O> super::Mesh<I, BasicVertex, O> for BasicShader
where
    I: glium::index::Index,
    O: object::Mesh<I>,
{
    fn create_vertices(&self, object: &O) -> Vec<BasicVertex> {
        let mesh = object.get_mesh();
        let mut vertices = Vec::new();
        for point in mesh.get_points() {
            vertices.push(BasicVertex {
                pos: *point.as_ref(),
            });
        }
        vertices
    }
}
