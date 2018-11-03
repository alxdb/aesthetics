use super::object;
use glium;

pub mod basic;
pub use self::basic::BasicShader;

pub trait Shader<V>
where
    V: glium::Vertex,
{
    fn new(display: &glium::Display) -> Self;
    fn get_program(&self) -> &glium::program::Program;
    fn create_vertices(&self, mesh: &object::MeshData) -> Vec<V>;
}
