use components::{ActiveCamera, Transform};
use glium::glutin;
use nalgebra_glm as glm;
use shred_derive::SystemData;
use specs::prelude::*;
use systems::event_handler::InputState;

pub struct FPSCamera {
    translate_speed: f32,
    rotate_speed: f32,
    invert_camera: (bool, bool),
}

impl Default for FPSCamera {
    fn default() -> Self {
        FPSCamera {
            translate_speed: 0.025,
            rotate_speed: 0.01,
            invert_camera: (false, false),
        }
    }
}

#[derive(SystemData)]
pub struct CameraControllerData<'a> {
    input_state: Read<'a, InputState>,
    active_camera: Read<'a, ActiveCamera>,
    transforms: WriteStorage<'a, Transform>,
}

impl<'a> System<'a> for FPSCamera {
    type SystemData = CameraControllerData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if let Some(active_camera) = data.active_camera.0 {
            if let Some(mut transform) = (&mut data.transforms).get_mut(active_camera) {
                // Rotate Camera
                if let Some((yaw, pitch)) = data.input_state.mouse_delta {
                    transform.rot *= glm::quat_angle_axis(
                        if self.invert_camera.1 { yaw } else { -yaw } as f32 * self.rotate_speed,
                        &glm::vec3(0.0, 1.0, 0.0),
                    );
                    let pitch_axis: glm::Vec3 =
                        glm::quat_axis(&transform.rot).cross(&glm::vec3(0.0, 1.0, 0.0));
                    println!("{}", pitch_axis);
                    transform.rot *= glm::quat_angle_axis(
                        if self.invert_camera.0 { pitch } else { -pitch } as f32
                            * self.rotate_speed,
                        &pitch_axis,
                    );
                    // println!("pitch {}", transform.rot);
                    transform.rot = glm::quat_normalize(&transform.rot);
                    // println!("yaw {}", transform.rot);
                }

                let key_state = &data.input_state.key_state;
                let forwards = glm::quat_rotate_vec3(&transform.rot, &glm::vec3(0.0, 0.0, 1.0));
                let left = glm::quat_rotate_vec3(&transform.rot, &glm::vec3(1.0, 0.0, 0.0));
                if key_state.contains(&glutin::VirtualKeyCode::W) {
                    transform.pos -= forwards * self.translate_speed;
                }
                if key_state.contains(&glutin::VirtualKeyCode::S) {
                    transform.pos += forwards * self.translate_speed;
                }
                if key_state.contains(&glutin::VirtualKeyCode::A) {
                    transform.pos -= left * self.translate_speed;
                }
                if key_state.contains(&glutin::VirtualKeyCode::D) {
                    transform.pos += left * self.translate_speed;
                }
                if key_state.contains(&glutin::VirtualKeyCode::LShift) {
                    transform.pos -= glm::vec3(0.0, 1.0, 0.0) * self.translate_speed;
                }
                if key_state.contains(&glutin::VirtualKeyCode::Space) {
                    transform.pos += glm::vec3(0.0, 1.0, 0.0) * self.translate_speed;
                }
            }
        }
    }
}
