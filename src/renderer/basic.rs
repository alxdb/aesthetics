use super::*;
use generational_arena::Arena;
use nalgebra_glm as glm;

pub struct BasicRenderer<'a, V, S>
where
    V: glium::Vertex,
    S: shader::Shader<V>,
{
    shader: S,
    display: &'a glium::Display,
    meshes: Arena<(object::MeshData, Buffers<V>)>,
}

impl<'a, V, S> BasicRenderer<'a, V, S>
where
    V: glium::Vertex,
    S: shader::Shader<V>,
{
    pub fn new(shader: S, display: &'a glium::Display) -> Self {
        BasicRenderer {
            shader,
            display,
            meshes: Arena::new(),
        }
    }

    pub fn add_object<O>(&mut self, object: O) -> generational_arena::Index
    where
        O: object::MeshObject,
    {
        let indices = object.ref_mesh().get_indices();
        let index_buffer =
            glium::index::IndexBuffer::immutable(self.display, indices.1, indices.0).unwrap();
        let vertex_buffer = glium::vertex::VertexBuffer::dynamic(
            self.display,
            &self.shader.create_vertices(object.ref_mesh()),
        ).unwrap();
        self.meshes.insert((
            object.ref_mesh().clone(),
            Buffers {
                vertex: vertex_buffer,
                index: index_buffer,
            },
        ))
    }

    pub fn update_object<F>(
        &mut self,
        index: generational_arena::Index,
        update_function: F,
    ) -> Result<(), &'static str>
    where
        F: Fn(&glm::Vec3) -> glm::Vec3,
    {
        let mesh = match self.meshes.get_mut(index) {
            Some(m) => m,
            None => return Err("object not found"),
        };
        mesh.0.update_points(update_function);
        mesh.1.vertex.write(&self.shader.create_vertices(&mesh.0));
        Ok(())
    }

    pub fn draw(
        &self,
        clear_colour: (f32, f32, f32, f32),
        draw_params: &glium::DrawParameters,
    ) -> Result<(), glium::SwapBuffersError> {
        use glium::Surface;
        let mut frame = self.display.draw();
        frame.clear_color_and_depth(clear_colour, 1.0);
        for (_, mesh) in self.meshes.iter() {
            frame
                .draw(
                    &mesh.1.vertex,
                    &mesh.1.index,
                    self.shader.get_program(),
                    &glium::uniforms::EmptyUniforms,
                    &draw_params,
                ).unwrap();
        }
        frame.finish()
    }
}
