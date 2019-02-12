use components::{
    mesh::{IndexType, MeshData},
    Camera, Transform,
};

use glium::{implement_vertex, uniform, IndexBuffer, VertexBuffer};
use shred_derive::SystemData;
use specs::prelude::*;

use std::collections::HashMap;

#[derive(SystemData)]
pub struct RendererData<'a> {
    entities: Entities<'a>,
    mesh: ReadStorage<'a, MeshData>,
    transform: ReadStorage<'a, Transform>,
    cameras: ReadStorage<'a, Camera>,
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    pos: [f32; 4],
}
implement_vertex!(Vertex, pos);

fn make_vertices(mesh: &MeshData) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for point in mesh.get_points() {
        let point_four = point.fixed_resize::<nalgebra_glm::U4, nalgebra_glm::U1>(1.0);
        vertices.push(Vertex {
            pos: *point_four.as_ref(),
        })
    }
    vertices
}

#[derive(Debug)]
struct Buffers {
    vertex: VertexBuffer<Vertex>,
    index: IndexBuffer<IndexType>,
}

pub struct Renderer<'a> {
    mesh_reader_id: ReaderId<ComponentEvent>,
    inserted_meshes: BitSet,
    modified_meshes: BitSet,
    removed_meshes: BitSet,
    buffers: HashMap<Entity, Buffers>,
    shader: glium::program::Program,
    display: &'a glium::Display,
    main_camera: Option<Entity>,
}

impl<'a> Renderer<'a> {
    pub fn new(world: &mut World, display: &'a glium::Display) -> Self {
        <Renderer as System>::SystemData::setup(&mut world.res);
        Renderer {
            mesh_reader_id: world.write_storage::<MeshData>().register_reader(),
            modified_meshes: BitSet::new(),
            inserted_meshes: BitSet::new(),
            removed_meshes: BitSet::new(),
            buffers: HashMap::new(),
            shader: glium::program::Program::from_source(
                display,
                include_str!("basic.vert"),
                include_str!("basic.frag"),
                None,
            )
            .unwrap(),
            display,
            main_camera: None,
        }
    }

    pub fn set_main_camera(&mut self, main_camera: Entity) {
        self.main_camera = Some(main_camera);
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
                Buffers {
                    vertex: glium::vertex::VertexBuffer::dynamic(
                        self.display,
                        &make_vertices(mesh),
                    )
                    .unwrap(),
                    index: glium::index::IndexBuffer::dynamic(
                        self.display,
                        *mesh.get_index_type(),
                        mesh.get_indices(),
                    )
                    .unwrap(),
                }
            };

            println!("created: {:?}\nfor {:?}", buffers, ent);
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
            if let Some(b) = self.buffers.remove(&ent) {
                println!("deleted: {:?}\nfor {:?}", b, ent);
            // Calls drop
            } else {
                panic!("desync");
            }
        }
    }
}

impl<'a, 'b> System<'b> for Renderer<'a> {
    type SystemData = RendererData<'b>;

    fn run(&mut self, data: Self::SystemData) {
        self.handle_buffer_events(&data);

        let main_camera = match self.main_camera {
            Some(ent) => match (&data.cameras, &data.transform)
                .join()
                .get(ent, &data.entities)
            {
                Some((camera, transform)) => (camera, transform),
                None => panic!("camera ref lost/no transform for camera"),
            },
            None => match (&data.cameras, &data.transform, &data.entities)
                .join()
                .next()
            {
                Some((camera, transform, ent)) => {
                    self.main_camera = Some(ent);
                    (camera, transform)
                }
                None => panic!("no camera entities with a transform"),
            },
        };

        use glium::Surface;
        let mut frame = self.display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        for (ent, buffer) in self.buffers.iter() {
            if let Some(transform) = data.transform.get(*ent) {
                let uniforms = uniform! {
                    model: *transform.make_model_matrix().as_ref(),
                    view: *main_camera.1.make_view_matrix().as_ref(),
                    projection: *main_camera.0.get_projection_matrix().as_ref()
                };

                frame
                    .draw(
                        &buffer.vertex,
                        &buffer.index,
                        &self.shader,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }
        }
        frame.finish().unwrap();
    }
}
