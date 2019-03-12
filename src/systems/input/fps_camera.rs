use components::{ActiveCamera, Transform};
use glium::glutin;
use nalgebra_glm as glm;
use shred_derive::SystemData;
use specs::prelude::*;
use systems::event_handler::InputState;

pub struct FPSCamera {
    translate_speed: f32,
    rotate_speed: f32,
}

impl Default for FPSCamera {
    fn default() -> Self {
        FPSCamera {
            translate_speed: 0.025,
            rotate_speed: 0.01,
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
                let key_state = &data.input_state.key_state;
                // Rotate Camera
                if let Some(delta) = data.input_state.mouse_delta {
                    let yaw = -delta.0 as f32 * self.rotate_speed;
                    let pitch = -delta.1 as f32 * self.rotate_speed;
                    // May be faster to create one quaternion from euler angles but this constructor seems to be missing
                    let euler_angles = glm::quat_euler_angles(&transform.rot);
                    let yaw_axis = glm::quat_rotate_vec3(
                        &glm::quat_angle_axis(
                            if euler_angles[0].abs() < 1e-3 {
                                -euler_angles[2]
                            } else {
                                -euler_angles[2] - std::f32::consts::PI
                            },
                            &glm::vec3(1.0, 0.0, 0.0),
                        ),
                        &glm::vec3(0.0, 1.0, 0.0),
                    ); // after pitch transformation this will be world y axis
                    let yaw_q = glm::quat_angle_axis(yaw, &yaw_axis);
                    let pitch_q = glm::quat_angle_axis(pitch, &glm::vec3(1.0, 0.0, 0.0));
                    let q = transform.rot * glm::quat_normalize(&(yaw_q * pitch_q));
                    let final_euler_angles = glm::quat_euler_angles(&q);
                    // prevent over the top rotation, but allow roll around y axis
                    if (final_euler_angles[0].abs() < 1e-3
                        && final_euler_angles[2].abs() < std::f32::consts::FRAC_PI_2)
                        || (final_euler_angles[0].abs() > 1e-3
                            && final_euler_angles[2].abs() > std::f32::consts::FRAC_PI_2)
                    {
                        transform.rot = glm::quat_normalize(&q);
                    } else {
                        transform.rot = glm::quat_normalize(&(transform.rot * yaw_q));
                    }
                }

                // Translate Camera
                if !key_state.is_empty() {
                    let euler_angles = glm::quat_euler_angles(&transform.rot);
                    // get rotation without pitch
                    let forwards_q = glm::quat_rotate(
                        &transform.rot,
                        if euler_angles[0].abs() < 1e-3 {
                            -euler_angles[2]
                        } else {
                            -euler_angles[2] - std::f32::consts::PI
                        },
                        &glm::vec3(1.0, 0.0, 0.0),
                    );
                    let forwards = glm::quat_rotate_vec3(&forwards_q, &glm::vec3(0.0, 0.0, 1.0));
                    let left = glm::quat_rotate_vec3(&transform.rot, &glm::vec3(1.0, 0.0, 0.0)); // pitch has no effect on x axis
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
                    if key_state.contains(&glutin::VirtualKeyCode::R) {
                        transform.pos = glm::vec3(0.0, 0.0, 0.0);
                        transform.rot = glm::Quat::identity();
                    }
                }
            }
        }
    }
}
