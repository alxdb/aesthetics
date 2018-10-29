use super::*;

struct BasicRenderer<V, I>
where
    V: glium::vertex::Vertex,
    I: glium::index::Index,
{
    shader: shader::Mesh<V, I, object::Mesh<I>>,
    objects: Vec<object::Mesh<I>>,
}
