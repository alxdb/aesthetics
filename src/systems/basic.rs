use super::*;
use generational_arena::Arena;
use nalgebra_glm as glm;

struct BasicObject<V>
where
    V: glium::Vertex,
{
    mesh: object::MeshData,
    buffers: Buffers<V>,
}

pub struct BasicRenderer<'a, V, S>
where
    V: glium::Vertex,
    S: shader::Shader<V>,
{
    shader: S,
    display: &'a glium::Display,
    objects: Arena<BasicObject<V>>,
}

impl<'a, V, S> Renderer<'a, V, S> for BasicRenderer<'a, V, S>
where
    V: glium::Vertex,
    S: shader::Shader<V>,
{
    fn new(display: &'a glium::Display) -> Self {
        BasicRenderer {
            shader: S::new(display),
            display,
            objects: Arena::new(),
        }
    }

    fn draw(
        &self,
        clear_colour: (f32, f32, f32, f32),
        draw_params: &glium::DrawParameters,
    ) -> Result<(), glium::SwapBuffersError> {
        use glium::Surface;
        let mut frame = self.display.draw();
        frame.clear_color_and_depth(clear_colour, 1.0);
        for (_, obj) in self.objects.iter() {
            frame
                .draw(
                    &obj.buffers.vertex,
                    &obj.buffers.index,
                    self.shader.get_program(),
                    &glium::uniforms::EmptyUniforms,
                    &draw_params,
                ).unwrap();
        }
        frame.finish()
    }
}

impl<'a, V, S> BasicRenderer<'a, V, S>
where
    V: glium::Vertex,
    S: shader::Shader<V>,
{
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
        self.objects.insert(
            BasicObject {
                mesh: object.ref_mesh().clone(),
                buffers: Buffers {
                    vertex: vertex_buffer,
                    index: index_buffer,
                },
            },
        )
    }

    pub fn update_object<F>(
        &mut self,
        index: generational_arena::Index,
        update_function: F,
    ) -> Result<(), &'static str>
    where
        F: Fn(&glm::Vec3) -> glm::Vec3,
    {
        let obj = match self.objects.get_mut(index) {
            Some(m) => m,
            None => return Err("object not found"),
        };
        obj.mesh.update_points(update_function);
        obj.buffers.vertex.write(&self.shader.create_vertices(&obj.mesh));
        Ok(())
    }
}
