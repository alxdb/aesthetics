pub mod renderers;

//mod basic;
//pub use self::basic::BasicRenderer;
//
//struct Buffers<V>
//where
//    V: glium::Vertex,
//{
//    pub vertex: glium::VertexBuffer<V>,
//    pub index: glium::IndexBuffer<object::IndexType>,
//}
//
//pub trait Renderer<'a, V, S>
//where
//    V: glium::Vertex,
//    S: shader::Shader<V>,
//{
//    fn new(display: &'a glium::Display) -> Self;
//    fn draw(
//        &self,
//        clear_colour: (f32, f32, f32, f32),
//        draw_params: &glium::DrawParameters,
//    ) -> Result<(), glium::SwapBuffersError>;
//}
