use components::{
    mesh::{IndexType, MeshData},
    ActiveCamera, Camera, Transform,
};

use glium::{implement_vertex, uniform, IndexBuffer, VertexBuffer};
use itertools::izip;
use shred_derive::SystemData;
use specs::prelude::*;

use std::collections::HashMap;

#[derive(SystemData)]
pub struct RendererData<'a> {
    entities: Entities<'a>,
    active_camera: Write<'a, ActiveCamera>,
    mesh: ReadStorage<'a, MeshData>,
    transform: ReadStorage<'a, Transform>,
    cameras: ReadStorage<'a, Camera>,
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    pos: [f32; 4],
    nrm: [f32; 4],
    tex: [f32; 2],
}
implement_vertex!(Vertex, pos, nrm, tex);

fn make_vertices(mesh: &MeshData) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for (point, normal, tex_coord) in
        izip!(mesh.get_points(), mesh.get_normals(), mesh.get_tex_coords())
    {
        let point_four = point.fixed_resize::<nalgebra_glm::U4, nalgebra_glm::U1>(1.0);
        let normal_four = normal.fixed_resize::<nalgebra_glm::U4, nalgebra_glm::U1>(0.0);
        vertices.push(Vertex {
            pos: *point_four.as_ref(),
            nrm: *normal_four.as_ref(),
            tex: *tex_coord.as_ref(),
        })
    }
    vertices
}

#[derive(Debug)]
struct Buffers {
    vertex: VertexBuffer<Vertex>,
    index: Option<IndexBuffer<IndexType>>,
}

pub struct Renderer {
    mesh_reader_id: ReaderId<ComponentEvent>,
    inserted_meshes: BitSet,
    modified_meshes: BitSet,
    removed_meshes: BitSet,
    buffers: HashMap<Entity, Buffers>,
    shader: glium::program::Program,
    display: glium::Display,
    draw_params: glium::DrawParameters<'static>,
    clear_color: (f32, f32, f32, f32),
}

impl Renderer {
    pub fn new(
        world: &mut World,
        display: glium::Display,
        draw_params: glium::DrawParameters<'static>,
        clear_color: (f32, f32, f32, f32),
    ) -> Self {
        <Renderer as System>::SystemData::setup(&mut world.res);
        Renderer {
            mesh_reader_id: world.write_storage::<MeshData>().register_reader(),
            modified_meshes: BitSet::new(),
            inserted_meshes: BitSet::new(),
            removed_meshes: BitSet::new(),
            buffers: HashMap::new(),
            shader: glium::program::Program::from_source(
                &display,
                include_str!("vertex.glsl"),
                include_str!("fragment.glsl"),
                None,
            )
            .unwrap(),
            display,
            draw_params,
            clear_color,
        }
    }

    fn handle_buffer_events(&mut self, data: &<Renderer as System>::SystemData) {
        self.inserted_meshes.clear();
        self.modified_meshes.clear();
        self.removed_meshes.clear();

        let events = data.mesh.channel().read(&mut self.mesh_reader_id);
        for event in events {
            match event {
                ComponentEvent::Modified(id) => {
                    self.modified_meshes.add(*id);
                }
                ComponentEvent::Inserted(id) => {
                    self.inserted_meshes.add(*id);
                }
                ComponentEvent::Removed(id) => {
                    self.removed_meshes.add(*id);
                }
            }
        }

        for (ent, mesh, _) in (&data.entities, &data.mesh, &self.inserted_meshes).join() {
            let buffers = {
                if let Some(index_buffer) = mesh.get_indices() {
                    Buffers {
                        vertex: glium::vertex::VertexBuffer::dynamic(
                            &self.display,
                            &make_vertices(mesh),
                        )
                        .unwrap(),
                        index: Some(
                            glium::index::IndexBuffer::dynamic(
                                &self.display,
                                *mesh.get_index_type(),
                                index_buffer,
                            )
                            .unwrap(),
                        ),
                    }
                } else {
                    Buffers {
                        vertex: glium::vertex::VertexBuffer::dynamic(
                            &self.display,
                            &make_vertices(mesh),
                        )
                        .unwrap(),
                        index: None,
                    }
                }
            };

            if let Some(_) = self.buffers.insert(ent, buffers) {
                panic!("desync");
            }
        }

        for (ent, mesh, _) in (&data.entities, &data.mesh, &self.modified_meshes).join() {
            // Only allows updating for the same number of vertices and identical indexes
            // Otherwise should just delete and recreate
            if let Some(b) = self.buffers.get(&ent) {
                b.vertex.write(&make_vertices(mesh));
            } else {
                panic!("desync");
            }
        }

        for (ent, _) in (&data.entities, &self.removed_meshes).join() {
            if let Some(_) = self.buffers.remove(&ent) {
                // Calls drop
            } else {
                panic!("desync");
            }
        }
    }
}

impl<'a> System<'a> for Renderer {
    type SystemData = RendererData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        self.handle_buffer_events(&data);

        let main_camera = match data.active_camera.0 {
            Some(ent) => match (&data.cameras, &data.transform)
                .join()
                .get(ent, &data.entities)
            {
                Some((camera, transform)) => (camera, transform),
                None => panic!("no transform and camera for entity: {:?}", ent),
            },
            None => match (&data.cameras, &data.transform, &data.entities)
                .join()
                .next()
            {
                Some((camera, transform, ent)) => {
                    *data.active_camera = ActiveCamera(Some(ent));
                    (camera, transform)
                }
                None => panic!("no entities with a transform and a camera"),
            },
        };

        use glium::Surface;
        let mut frame = self.display.draw();
        frame.clear_color_and_depth(self.clear_color, 1.0);

        for (ent, buffer) in self.buffers.iter() {
            if let Some(transform) = data.transform.get(*ent) {
                let uniforms = uniform! {
                    model: *transform.make_model_matrix().as_ref(),
                    view: *main_camera.1.make_view_matrix().as_ref(),
                    projection: *main_camera.0.get_projection_matrix().as_ref()
                };

                if let Some(index_buffer) = &buffer.index {
                    frame
                        .draw(
                            &buffer.vertex,
                            index_buffer,
                            &self.shader,
                            &uniforms,
                            &self.draw_params,
                        )
                        .unwrap();
                } else {
                    if let Some(mesh) = data.mesh.get(*ent) {
                        frame
                            .draw(
                                &buffer.vertex,
                                glium::index::NoIndices(*mesh.get_index_type()),
                                &self.shader,
                                &uniforms,
                                &self.draw_params,
                            )
                            .unwrap();
                    } else {
                        panic!("desync")
                    }
                }
            }
        }
        frame.finish().unwrap();
    }
}
