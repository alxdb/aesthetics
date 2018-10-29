use super::object;
use glium;

pub mod basic;
pub use self::basic::BasicShader;

pub trait Shader {
    fn new(window: &::window::Window) -> Self;
    fn get_program(&self) -> &glium::program::Program;
}

pub trait Mesh<I, V, O>: Shader
where
    I: glium::index::Index,
    V: glium::Vertex,
    O: object::Mesh<I>,
{
    fn create_vertices(&self, &O) -> Vec<V>;
}
