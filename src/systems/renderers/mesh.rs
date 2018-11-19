use components::mesh::MeshData;
use specs::prelude::*;
use shred_derive::SystemData;

#[derive(SystemData)]
struct MeshRendererData<'a> {
    mesh8: ReadStorage<'a, MeshData<u8>>,
    mesh16: ReadStorage<'a, MeshData<u16>>,
    mesh32: ReadStorage<'a, MeshData<u32>>,
}

struct MeshRenderer {

}

impl<'a> System<'a> for MeshRenderer {
    type SystemData = MeshRendererData<'a>;

    fn run(&mut self, meshes: Self::SystemData) {
        unimplemented!()
    }
}
