pub mod object;
pub mod shader;

mod basic;

trait Renderer<F>
where
    F: glium::Surface,
{
    fn draw(&F) -> Result<(), glium::DrawError>;
}

// trait MeshShader<V, I>
// where
//     V: glium::Vertex,
//     I: glium::index::Index,
// {
//     fn add_object
// }
