use super::*;
use glium::implement_vertex;
use glium::program;
use object;

#[derive(Copy, Clone, Debug)]
pub struct BasicVertex {
    pos: [f32; 3],
}
implement_vertex!(BasicVertex, pos);

pub struct BasicShader {
    program: program::Program,
}

impl Shader<BasicVertex> for BasicShader {
    fn new(display: &glium::Display) -> Self {
        let program = glium::program::Program::from_source(
            display,
            include_str!("glsl/mesh.vert"),
            include_str!("glsl/mesh.frag"),
            None,
        ).unwrap();
        BasicShader { program }
    }

    fn get_program(&self) -> &program::Program {
        &self.program
    }

    fn create_vertices(&self, mesh: &super::object::MeshData) -> Vec<BasicVertex> {
        let mut vertices = Vec::new();
        for point in mesh.get_points() {
            vertices.push(BasicVertex {
                pos: *point.as_ref(),
            });
        }
        vertices
    }
}
